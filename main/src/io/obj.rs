use nalgebra_glm as glm;
use glm::{DVec3, U32Vec3};
use super::super::mesh::SharedMesh;

use std::io::BufWriter;
use std::io::BufReader;
use std::io::prelude::*;

pub fn read<T: Read>(reader: &mut BufReader<T>) -> SharedMesh {

    let mut positions = Vec::<DVec3>::new();
    let mut triangles = Vec::<U32Vec3>::new();

    for line in reader.lines() {
        if let Ok(l) = line {
            let split = l.split(" ").collect::<Vec<&str>>();
            match split[0] {
                "v" => {
                    let position = DVec3::new(split[1].parse::<f64>().unwrap(), split[2].parse::<f64>().unwrap(), split[3].parse::<f64>().unwrap());
                    positions.push(position);
                },
                "f" => {
                    triangles.push(U32Vec3::new(
                        split[1].parse::<u32>().unwrap() - 1,
                        split[2].parse::<u32>().unwrap() - 1,
                        split[3].parse::<u32>().unwrap() - 1));
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
        colors: None,
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

    match &shared_mesh.normals {
        Some(normals) => {
            for i in 0..normals.len() {
                write!("vn {} {} {}", normals[i].x, normals[i].y, normals[i].z);
            }
            for i in 0..shared_mesh.triangles.len() {
                let triangle = shared_mesh.triangles[i];
                write!("f {}//{} {}//{} {}//{}", triangle[0] + 1, triangle[1] + 1, triangle[2] + 1, triangle[0] + 1, triangle[1] + 1, triangle[2] + 1 );
            }
        },
        None => {
            for i in 0..shared_mesh.triangles.len() {
                let triangle = shared_mesh.triangles[i];
                write!("f {} {} {}", triangle[0] + 1, triangle[1] + 1, triangle[2] + 1);
            }
        }
    };
}