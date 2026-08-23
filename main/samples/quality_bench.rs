#![allow(dead_code)]
#![allow(incomplete_include)]

// Decimation quality benchmark: renders each model before/after decimation
// with a software rasterizer from multiple camera angles and reports the
// PSNR between the screenshots.
//
// Shading uses smooth vertex normals computed with a crease angle (the same
// way on the original and the decimated mesh), interpolated per pixel, so
// the PSNR is sensitive to normal quality and not just to silhouettes.
//
// Run from the repo root:
//   cargo run --release --example quality_bench
//
// Models are read from main/samples/bench/models/*.obj. Outputs land in
// main/samples/bench/out/: per-view screenshots, results.csv, and a
// self-contained report.html generated from a fixed template.

#[path = "../src/lib.rs"]
mod nanomesh;

use nalgebra_glm as glm;
use glm::{DVec3, U32Vec3};
use nanomesh::mesh::{ConnectedMesh, SharedMesh};
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::time::Instant;

const IMG_SIZE: usize = 600;
const RATIOS: [f32; 4] = [0.5, 0.25, 0.1, 0.05];
const CREASE_ANGLE_DEG: f64 = 45.0; // dihedral angle above which shading splits into a hard edge
const MODELS_DIR: &str = "main/samples/bench/models";
const OUT_DIR: &str = "main/samples/bench/out";

// BENCH_MODELS and BENCH_CREASE env vars override the defaults, to iterate on
// a single model or compare crease settings without recompiling.
fn models_dir() -> String {
    std::env::var("BENCH_MODELS").unwrap_or_else(|_| MODELS_DIR.to_string())
}

fn crease_angle_deg() -> f64 {
    std::env::var("BENCH_CREASE").ok().and_then(|v| v.parse().ok()).unwrap_or(CREASE_ANGLE_DEG)
}

struct RatioResult {
    ratio: f32,
    triangles: usize,
    decimate_ms: u128,
    psnr_mean: f64,
    psnr_min: f64,
    image_png: Vec<u8>, // view 0 screenshot
}

struct ModelResult {
    name: String,
    triangles: usize,
    vertices: usize,
    original_png: Vec<u8>, // view 0 screenshot
    ratios: Vec<RatioResult>,
}

fn main() {
    fs::create_dir_all(OUT_DIR).unwrap();

    let mut model_paths: Vec<_> = fs::read_dir(models_dir())
        .expect("run from the repo root: main/samples/bench/models not found")
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().map_or(false, |e| e == "obj"))
        .collect();
    model_paths.sort();

    let mut csv = String::from("model,ratio,triangles_before,triangles_after,decimate_ms,psnr_min_db,psnr_mean_db\n");
    let mut results = Vec::new();

    for path in &model_paths {
        let name = path.file_stem().unwrap().to_string_lossy().to_string();
        let original = load_obj(path.to_str().unwrap());
        println!("\n=== {} ({} triangles, {} vertices)", name, original.triangles.len(), original.positions.len());

        let cameras = make_cameras(&original.positions);
        let original_normals = compute_corner_normals(&original.positions, &original.triangles);
        let reference_views: Vec<Vec<u8>> = cameras
            .iter()
            .map(|c| render(&original.positions, &original.triangles, &original_normals, c))
            .collect();
        for (v, img) in reference_views.iter().enumerate().take(2) {
            save_png(&format!("{}/{}_original_view{}.png", OUT_DIR, name, v), img);
        }

        let mut model_result = ModelResult {
            name: name.clone(),
            triangles: original.triangles.len(),
            vertices: original.positions.len(),
            original_png: encode_png(&reference_views[0]),
            ratios: Vec::new(),
        };

        for &ratio in &RATIOS {
            let result = catch_unwind(AssertUnwindSafe(|| {
                let mut connected = ConnectedMesh::from(&original);
                let start = Instant::now();
                connected.decimate_to_ratio(ratio);
                let elapsed_ms = start.elapsed().as_millis();
                (SharedMesh::from(&connected), elapsed_ms)
            }));

            let (decimated, elapsed_ms) = match result {
                Ok(r) => r,
                Err(_) => {
                    println!("  ratio {:>4}: PANIC during decimation", ratio);
                    csv.push_str(&format!("{},{},{},PANIC,,,\n", name, ratio, original.triangles.len()));
                    continue;
                }
            };

            let decimated_normals = compute_corner_normals(&decimated.positions, &decimated.triangles);
            let mut psnrs = Vec::new();
            let mut view0_png = Vec::new();
            for (v, camera) in cameras.iter().enumerate() {
                let img = render(&decimated.positions, &decimated.triangles, &decimated_normals, camera);
                psnrs.push(psnr(&reference_views[v], &img));
                if v < 2 {
                    save_png(&format!("{}/{}_r{}_view{}.png", OUT_DIR, name, ratio, v), &img);
                }
                if v == 0 {
                    view0_png = encode_png(&img);
                }
            }
            let mean = psnrs.iter().sum::<f64>() / psnrs.len() as f64;
            let min = psnrs.iter().cloned().fold(f64::INFINITY, f64::min);
            println!(
                "  ratio {:>4}: {:>7} tris, decimated in {:>5} ms, PSNR mean {:>6.2} dB, min {:>6.2} dB",
                ratio, decimated.triangles.len(), elapsed_ms, mean, min
            );
            csv.push_str(&format!(
                "{},{},{},{},{},{:.2},{:.2}\n",
                name, ratio, original.triangles.len(), decimated.triangles.len(), elapsed_ms, min, mean
            ));
            model_result.ratios.push(RatioResult {
                ratio,
                triangles: decimated.triangles.len(),
                decimate_ms: elapsed_ms,
                psnr_mean: mean,
                psnr_min: min,
                image_png: view0_png,
            });
        }
        results.push(model_result);
    }

    File::create(format!("{}/results.csv", OUT_DIR)).unwrap().write_all(csv.as_bytes()).unwrap();
    write_report(&results);
    println!("\nresults written to {}/results.csv and {}/report.html", OUT_DIR, OUT_DIR);
}

// ---------------------------------------------------------------------------
// Mesh loading and normals

// Robust-ish obj loader: handles `f a/b/c` styles, polygon fans and negative
// indices, unlike the minimal loader in io::obj.
fn load_obj(path: &str) -> SharedMesh {
    let reader = BufReader::new(File::open(path).unwrap());
    let mut positions = Vec::<DVec3>::new();
    let mut triangles = Vec::<U32Vec3>::new();

    for line in reader.lines() {
        let line = match line { Ok(l) => l, Err(_) => continue };
        let mut tokens = line.split_whitespace();
        match tokens.next() {
            Some("v") => {
                let coords: Vec<f64> = tokens.take(3).map(|t| t.parse().unwrap()).collect();
                positions.push(DVec3::new(coords[0], coords[1], coords[2]));
            }
            Some("f") => {
                let indices: Vec<u32> = tokens
                    .map(|t| {
                        let idx: i64 = t.split('/').next().unwrap().parse().unwrap();
                        if idx < 0 { (positions.len() as i64 + idx) as u32 } else { (idx - 1) as u32 }
                    })
                    .collect();
                for i in 1..indices.len() - 1 {
                    triangles.push(U32Vec3::new(indices[0], indices[i], indices[i + 1]));
                }
            }
            _ => (),
        }
    }

    SharedMesh { groups: Vec::new(), triangles, positions, normals: None, colors: None }
}

// Per-corner smooth normals with a crease angle: each corner averages the
// angle-weighted normals of the faces around its vertex whose dihedral with
// the corner's own face stays under CREASE_ANGLE_DEG. Sharper transitions
// split into hard shading edges, like typical game-engine normal generation.
fn compute_corner_normals(positions: &[DVec3], triangles: &[U32Vec3]) -> Vec<[DVec3; 3]> {
    let cos_crease = crease_angle_deg().to_radians().cos();

    let mut face_normals = Vec::with_capacity(triangles.len());
    for t in triangles {
        let n = glm::cross(
            &(positions[t[1] as usize] - positions[t[0] as usize]),
            &(positions[t[2] as usize] - positions[t[0] as usize]),
        );
        let len = glm::length(&n);
        face_normals.push(if len > 1e-30 { n / len } else { DVec3::new(0.0, 0.0, 1.0) });
    }

    let mut vertex_faces: Vec<Vec<u32>> = vec![Vec::new(); positions.len()];
    for (fi, t) in triangles.iter().enumerate() {
        for k in 0..3 {
            vertex_faces[t[k] as usize].push(fi as u32);
        }
    }

    let corner_angle = |fi: usize, vertex: u32| -> f64 {
        let t = triangles[fi];
        let k = (0..3).find(|&k| t[k] == vertex).unwrap();
        let p = positions[t[k] as usize];
        let a = glm::normalize(&(positions[t[(k + 1) % 3] as usize] - p));
        let b = glm::normalize(&(positions[t[(k + 2) % 3] as usize] - p));
        glm::dot(&a, &b).clamp(-1.0, 1.0).acos()
    };

    triangles
        .iter()
        .enumerate()
        .map(|(fi, t)| {
            let mut corners = [DVec3::new(0.0, 0.0, 1.0); 3];
            for k in 0..3 {
                let vertex = t[k];
                let mut acc = DVec3::new(0.0, 0.0, 0.0);
                for &other in &vertex_faces[vertex as usize] {
                    if glm::dot(&face_normals[other as usize], &face_normals[fi]) >= cos_crease {
                        acc += face_normals[other as usize] * corner_angle(other as usize, vertex);
                    }
                }
                let len = glm::length(&acc);
                corners[k] = if len > 1e-30 { acc / len } else { face_normals[fi] };
            }
            corners
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Software renderer

struct Camera {
    eye: DVec3,
    right: DVec3,
    up: DVec3,
    forward: DVec3,
    focal: f64, // 1 / tan(fov/2)
    near: f64,
}

fn make_cameras(positions: &[DVec3]) -> Vec<Camera> {
    let mut min = DVec3::repeat(f64::INFINITY);
    let mut max = DVec3::repeat(f64::NEG_INFINITY);
    for p in positions {
        min = glm::min2(&min, p);
        max = glm::max2(&max, p);
    }
    let center = (min + max) * 0.5;
    let radius = glm::length(&(max - min)) * 0.5;

    // 8 orbit views at low elevation + 2 high-angle views
    let mut angles = Vec::new();
    for i in 0..8 {
        angles.push((i as f64 * 45.0, 20.0));
    }
    angles.push((30.0, 60.0));
    angles.push((210.0, 60.0));

    angles
        .iter()
        .map(|&(azimuth_deg, elevation_deg): &(f64, f64)| {
            let az = azimuth_deg.to_radians();
            let el = elevation_deg.to_radians();
            let dir = DVec3::new(el.cos() * az.cos(), el.sin(), el.cos() * az.sin());
            let eye = center + dir * radius * 2.4;
            let forward = glm::normalize(&(center - eye));
            let world_up = DVec3::new(0.0, 1.0, 0.0);
            let right = glm::normalize(&glm::cross(&forward, &world_up));
            let up = glm::cross(&right, &forward);
            Camera {
                eye,
                right,
                up,
                forward,
                focal: 1.0 / (25.0_f64.to_radians()).tan(), // 50 deg vertical fov
                near: radius * 0.01,
            }
        })
        .collect()
}

// Software rasterizer with a 1/z depth buffer and per-pixel interpolated
// normals (perspective-correct). Returns RGB8.
fn render(positions: &[DVec3], triangles: &[U32Vec3], corner_normals: &[[DVec3; 3]], cam: &Camera) -> Vec<u8> {
    let w = IMG_SIZE;
    let h = IMG_SIZE;
    let mut color = vec![30u8; w * h * 3]; // dark gray background
    let mut depth = vec![0.0f64; w * h]; // stores 1/z, larger = closer

    let key_light = glm::normalize(&DVec3::new(0.6, 1.0, 0.4));

    for (ti, tri) in triangles.iter().enumerate() {
        let world = [
            positions[tri[0] as usize],
            positions[tri[1] as usize],
            positions[tri[2] as usize],
        ];

        // camera space (z = distance along view direction)
        let cs = world.map(|p| {
            let d = p - cam.eye;
            DVec3::new(glm::dot(&d, &cam.right), glm::dot(&d, &cam.up), glm::dot(&d, &cam.forward))
        });
        if cs.iter().any(|c| c.z < cam.near) {
            continue;
        }

        // screen space, keep 1/z for perspective-correct interpolation
        let sc = cs.map(|c| {
            let inv_z = 1.0 / c.z;
            let x = (c.x * cam.focal * inv_z * 0.5 + 0.5) * w as f64;
            let y = (1.0 - (c.y * cam.focal * inv_z * 0.5 + 0.5)) * h as f64;
            (x, y, inv_z)
        });

        let area = (sc[1].0 - sc[0].0) * (sc[2].1 - sc[0].1) - (sc[2].0 - sc[0].0) * (sc[1].1 - sc[0].1);
        if area.abs() < 1e-12 {
            continue;
        }

        // normals premultiplied by 1/z: interpolating these linearly in screen
        // space and renormalizing gives the perspective-correct normal
        let n_over_z = [
            corner_normals[ti][0] * sc[0].2,
            corner_normals[ti][1] * sc[1].2,
            corner_normals[ti][2] * sc[2].2,
        ];

        let x_min = sc.iter().map(|s| s.0).fold(f64::INFINITY, f64::min).floor().max(0.0) as usize;
        let x_max = sc.iter().map(|s| s.0).fold(f64::NEG_INFINITY, f64::max).ceil().min(w as f64 - 1.0) as usize;
        let y_min = sc.iter().map(|s| s.1).fold(f64::INFINITY, f64::min).floor().max(0.0) as usize;
        let y_max = sc.iter().map(|s| s.1).fold(f64::NEG_INFINITY, f64::max).ceil().min(h as f64 - 1.0) as usize;
        if x_min > x_max || y_min > y_max {
            continue;
        }

        let inv_area = 1.0 / area;
        for y in y_min..=y_max {
            let py = y as f64 + 0.5;
            for x in x_min..=x_max {
                let px = x as f64 + 0.5;
                let w0 = ((sc[1].0 - px) * (sc[2].1 - py) - (sc[2].0 - px) * (sc[1].1 - py)) * inv_area;
                let w1 = ((sc[2].0 - px) * (sc[0].1 - py) - (sc[0].0 - px) * (sc[2].1 - py)) * inv_area;
                let w2 = 1.0 - w0 - w1;
                if w0 < 0.0 || w1 < 0.0 || w2 < 0.0 {
                    continue;
                }
                let inv_z = w0 * sc[0].2 + w1 * sc[1].2 + w2 * sc[2].2;
                let idx = y * w + x;
                if inv_z <= depth[idx] {
                    continue;
                }
                depth[idx] = inv_z;

                let n = n_over_z[0] * w0 + n_over_z[1] * w1 + n_over_z[2] * w2;
                let len = glm::length(&n);
                let n = if len > 1e-30 { n / len } else { cam.forward };

                // double-sided: headlight + fixed key light
                let headlight = glm::dot(&n, &cam.forward).abs();
                let key = glm::dot(&n, &key_light).abs();
                let intensity = (0.12 + 0.53 * headlight + 0.35 * key).min(1.0);
                color[idx * 3] = (intensity * 235.0) as u8;
                color[idx * 3 + 1] = (intensity * 232.0) as u8;
                color[idx * 3 + 2] = (intensity * 225.0) as u8;
            }
        }
    }

    color
}

fn psnr(a: &[u8], b: &[u8]) -> f64 {
    let mse: f64 = a
        .iter()
        .zip(b.iter())
        .map(|(&x, &y)| {
            let d = x as f64 - y as f64;
            d * d
        })
        .sum::<f64>()
        / a.len() as f64;
    if mse == 0.0 {
        f64::INFINITY
    } else {
        10.0 * (255.0 * 255.0 / mse).log10()
    }
}

// ---------------------------------------------------------------------------
// Output: PNG, CSV, HTML report

fn encode_png(rgb: &[u8]) -> Vec<u8> {
    let mut buf = Vec::new();
    {
        let mut encoder = png::Encoder::new(&mut buf, IMG_SIZE as u32, IMG_SIZE as u32);
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(rgb).unwrap();
    }
    buf
}

fn save_png(path: &str, rgb: &[u8]) {
    File::create(path).unwrap().write_all(&encode_png(rgb)).unwrap();
}

fn base64(data: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity((data.len() + 2) / 3 * 4);
    for chunk in data.chunks(3) {
        let b = [chunk[0], *chunk.get(1).unwrap_or(&0), *chunk.get(2).unwrap_or(&0)];
        let n = ((b[0] as u32) << 16) | ((b[1] as u32) << 8) | b[2] as u32;
        out.push(TABLE[(n >> 18) as usize & 63] as char);
        out.push(TABLE[(n >> 12) as usize & 63] as char);
        out.push(if chunk.len() > 1 { TABLE[(n >> 6) as usize & 63] as char } else { '=' });
        out.push(if chunk.len() > 2 { TABLE[n as usize & 63] as char } else { '=' });
    }
    out
}

fn psnr_class(v: f64) -> &'static str {
    if v >= 35.0 {
        "good"
    } else if v >= 28.0 {
        "mid"
    } else {
        "bad"
    }
}

fn today() -> String {
    // civil date from unix days (Howard Hinnant's algorithm), avoids a chrono dep
    let secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
    let z = (secs / 86400) as i64 + 719468;
    let era = z.div_euclid(146097);
    let doe = z.rem_euclid(146097);
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = yoe + era * 400 + if m <= 2 { 1 } else { 0 };
    format!("{:04}-{:02}-{:02}", y, m, d)
}

const REPORT_CSS: &str = r#"
:root {
  --ink: #17181b; --panel: #212327; --panel-2: #2a2c31; --line: #34363c;
  --text: #e9e6df; --muted: #a09d94; --accent: #d0925c;
  --good: #84b070; --mid: #d8b46a; --bad: #d6705f;
  --mono: "IBM Plex Mono", ui-monospace, SFMono-Regular, Menlo, monospace;
}
* { box-sizing: border-box; }
body {
  background: var(--ink); color: var(--text); margin: 0;
  font-family: "Archivo", "Helvetica Neue", Arial, sans-serif;
  font-size: 16px; line-height: 1.55;
}
main { max-width: 1060px; margin: 0 auto; padding: 56px 28px 80px; }
header { border-bottom: 1px solid var(--line); padding-bottom: 28px; margin-bottom: 36px; }
.eyebrow { font-family: var(--mono); font-size: 12px; letter-spacing: 0.14em; text-transform: uppercase; color: var(--accent); margin: 0 0 10px; }
h1 { font-size: 40px; font-weight: 700; font-stretch: 80%; letter-spacing: -0.01em; margin: 0 0 14px; text-wrap: balance; }
.lede { max-width: 68ch; color: var(--muted); margin: 0; font-size: 16px; }
h2 { font-size: 22px; font-weight: 650; font-stretch: 85%; margin: 52px 0 6px; }
h2 + p.sub { color: var(--muted); margin: 0 0 20px; max-width: 68ch; font-size: 15px; }
table { border-collapse: collapse; width: 100%; font-family: var(--mono); font-size: 13.5px; font-variant-numeric: tabular-nums; }
.table-wrap { overflow-x: auto; border: 1px solid var(--line); border-radius: 6px; background: var(--panel); }
th { text-align: right; font-weight: 500; color: var(--muted); font-size: 11px; letter-spacing: 0.1em; text-transform: uppercase; padding: 12px 16px 10px; border-bottom: 1px solid var(--line); }
th:first-child { text-align: left; }
td { padding: 8px 16px; border-bottom: 1px solid #2b2d32; }
tr:last-child td { border-bottom: none; }
td.num { text-align: right; white-space: nowrap; }
td.model-cell { vertical-align: top; padding-top: 12px; border-right: 1px solid var(--line); font-family: "Archivo", sans-serif; }
.model-name { display: block; font-weight: 600; font-size: 15px; }
.model-tris { display: block; font-family: var(--mono); color: var(--muted); font-size: 12px; margin-top: 4px; }
td.psnr.good { color: var(--good); }
td.psnr.mid { color: var(--mid); }
td.psnr.bad { color: var(--bad); font-weight: 500; }
.model { margin-top: 36px; background: var(--panel); border: 1px solid var(--line); border-radius: 8px; padding: 20px 20px 18px; }
.model-head { display: flex; align-items: baseline; gap: 14px; flex-wrap: wrap; margin-bottom: 14px; }
.model-head h3 { margin: 0; font-size: 19px; font-weight: 650; font-stretch: 85%; }
.model-head .kind { font-family: var(--mono); font-size: 12px; color: var(--muted); }
.strip { display: grid; grid-template-columns: repeat(auto-fit, minmax(180px, 1fr)); gap: 12px; }
.strip figure { margin: 0; }
.strip img { width: 100%; height: auto; display: block; border-radius: 4px; border: 1px solid var(--line); background: #1e1e1e; }
.strip figcaption { font-family: var(--mono); font-size: 12px; color: var(--muted); margin-top: 7px; }
.strip figcaption .good { color: var(--good); }
.strip figcaption .mid { color: var(--mid); }
.strip figcaption .bad { color: var(--bad); }
.method { border-top: 1px solid var(--line); margin-top: 56px; padding-top: 24px; }
.method p { color: var(--muted); font-size: 14px; max-width: 76ch; }
.method code { font-family: var(--mono); font-size: 13px; color: var(--text); background: var(--panel-2); padding: 1px 6px; border-radius: 3px; }
@media (max-width: 640px) { h1 { font-size: 30px; } }
"#;

fn write_report(results: &[ModelResult]) {
    let mut table_rows = String::new();
    for model in results {
        for (i, r) in model.ratios.iter().enumerate() {
            let model_cell = if i == 0 {
                format!(
                    r#"<td class="model-cell" rowspan="{}"><span class="model-name">{}</span><span class="model-tris">{} tris · {} verts</span></td>"#,
                    model.ratios.len(),
                    model.name,
                    model.triangles,
                    model.vertices
                )
            } else {
                String::new()
            };
            table_rows.push_str(&format!(
                r#"<tr>{}<td class="num">{}%</td><td class="num">{}</td><td class="num">{} ms</td><td class="num psnr {}">{:.1}</td><td class="num psnr {}">{:.1}</td></tr>
"#,
                model_cell,
                (r.ratio * 100.0).round() as u32,
                r.triangles,
                r.decimate_ms,
                psnr_class(r.psnr_mean),
                r.psnr_mean,
                psnr_class(r.psnr_min),
                r.psnr_min
            ));
        }
    }

    let mut strips = String::new();
    for model in results {
        let mut figures = format!(
            r#"<figure><img src="data:image/png;base64,{}" alt="{} original"><figcaption>original · {} tris</figcaption></figure>"#,
            base64(&model.original_png),
            model.name,
            model.triangles
        );
        for r in &model.ratios {
            figures.push_str(&format!(
                r#"<figure><img src="data:image/png;base64,{}" alt="{} at {}%"><figcaption>{}% · <span class="{}">{:.1} dB</span></figcaption></figure>"#,
                base64(&r.image_png),
                model.name,
                (r.ratio * 100.0).round() as u32,
                (r.ratio * 100.0).round() as u32,
                psnr_class(r.psnr_mean),
                r.psnr_mean
            ));
        }
        strips.push_str(&format!(
            r#"<section class="model"><div class="model-head"><h3>{}</h3><span class="kind">{} tris · {} verts</span></div><div class="strip">{}</div></section>
"#,
            model.name, model.triangles, model.vertices, figures
        ));
    }

    let html = format!(
        r#"<title>Nanomesh Decimation Benchmark</title>
<link rel="preconnect" href="https://fonts.googleapis.com">
<link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Archivo:wdth,wght@62..125,300..800&family=IBM+Plex+Mono:wght@400;500&display=swap">
<style>{css}</style>
<main>
<header>
<p class="eyebrow">nanomesh-rust · quality_bench · {date}</p>
<h1>Nanomesh Decimation Benchmark</h1>
<p class="lede">Each model is decimated to {ratios:?} of its original triangle count, then rendered from 10 camera angles ({size}×{size}, smooth vertex normals with a {crease}° crease angle, interpolated per pixel) and compared to the original render with PSNR. Rough reading: above 35 dB differences are hard to spot, 28–35 dB is visible faceting or shading drift, below 28 dB is structural damage.</p>
</header>
<h2>PSNR by model and ratio</h2>
<p class="sub">Mean and worst-case PSNR over the 10 views. Normals are recomputed on the decimated mesh with the same crease rule, so shading and normal quality count, not just silhouettes.</p>
<div class="table-wrap"><table>
<thead><tr><th>Model</th><th>Ratio</th><th>Triangles</th><th>Decimate</th><th>PSNR mean (dB)</th><th>PSNR min (dB)</th></tr></thead>
<tbody>
{rows}</tbody>
</table></div>
<h2>Visual comparisons</h2>
<p class="sub">View 0 of 10 for each model, original on the left. Captions show mean PSNR across all views.</p>
{strips}
<div class="method">
<h2>Method</h2>
<p>Generated by <code>main/samples/quality_bench.rs</code> (<code>cargo run --release --example quality_bench</code>). Models come from <code>main/samples/bench/models/*.obj</code>; source normals are ignored and smooth vertex normals are computed with a {crease}° crease angle on both the original and the decimated mesh, matching typical game-engine normal generation. The renderer is a dependency-free software rasterizer (per-pixel perspective-correct normal interpolation, headlight + key light, double-sided). Per-view screenshots and <code>results.csv</code> sit next to this file.</p>
</div>
</main>
"#,
        css = REPORT_CSS,
        date = today(),
        ratios = RATIOS,
        size = IMG_SIZE,
        crease = crease_angle_deg(),
        rows = table_rows,
        strips = strips
    );

    File::create(format!("{}/report.html", OUT_DIR)).unwrap().write_all(html.as_bytes()).unwrap();
}
