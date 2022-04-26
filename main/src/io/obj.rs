use super::super::base::DVec3;
use super::super::mesh::SharedMesh;

use std::io::BufWriter;
use std::io::BufReader;
use std::io::prelude::*;

pub fn read<T: Read>(reader: &mut BufReader<T>) -> SharedMesh {

    let mut positions = Vec::<DVec3>::new();
    let mut triangles = Vec::<u32>::new();

    for line in reader.lines() {
        if let Ok(l) = line {
            let split = l.split(" ").collect::<Vec<&str>>();
            match split[0] {
                "v" => {
                    let position = DVec3::new(split[1].parse::<f64>().unwrap(), split[2].parse::<f64>().unwrap(), split[3].parse::<f64>().unwrap());
                    positions.push(position);
                },
                "f" => {
                    triangles.push(split[1].parse::<u32>().unwrap() - 1);
                    triangles.push(split[2].parse::<u32>().unwrap() - 1);
                    triangles.push(split[3].parse::<u32>().unwrap() - 1);
                },
                _ => ()
            }
        }
    }

    SharedMesh {
        groups: Vec::new(),
        triangles: triangles,
        positions: positions,
        normals: None,
    }
}

pub fn write<T: Write>(shared_mesh: &SharedMesh, writer: &mut BufWriter<T>) {

    macro_rules! write {
        () => {{
            writer.write("\n".as_bytes()).unwrap();
        }};
        ($text:expr) => {{
            writer.write($text.as_bytes()).unwrap();
            write!();
        }};
        ($text:expr, $($args:expr), *) => {{
            writer.write(format!($text, $($args), *).as_bytes()).unwrap();
            write!();
        }}
    }

    for i in 0..shared_mesh.positions.len() {
        write!("v {} {} {}", shared_mesh.positions[i].x, shared_mesh.positions[i].y, shared_mesh.positions[i].z);
    }

    for i in (0..shared_mesh.triangles.len()).step_by(3) {
        write!("f {} {} {}", shared_mesh.triangles[i] + 1, shared_mesh.triangles[i + 1] + 1, shared_mesh.triangles[i + 2] + 1);
    }
}