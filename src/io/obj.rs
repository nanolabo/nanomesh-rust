use crate::SharedMesh;
use std::fs::File;
use std::path::Path;

use std::io::prelude::*;
use std::io::LineWriter;
use crate::Vector3;

pub fn read(path: &str) -> SharedMesh {

    let mut positions = Vec::<Vector3>::new();
    let mut triangles = Vec::<u32>::new();

    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(l) = line {
                let split = l.split(" ").collect::<Vec<&str>>();
                match split[0] {
                    "v" => {
                        let position = Vector3::new(split[1].parse::<f64>().unwrap(), split[2].parse::<f64>().unwrap(), split[3].parse::<f64>().unwrap());
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
    }

    SharedMesh {
        positions: positions,
        triangles: triangles,
        normals: Vec::new(),
        groups: Vec::new(),
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> std::io::Result<std::io::Lines<std::io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(std::io::BufReader::new(file).lines())
}

pub fn write(shared_mesh: SharedMesh, path: &str) {

    let file = File::create(path).unwrap();
    let mut file = LineWriter::new(file);

    for i in 0..shared_mesh.positions.len() {
        file.write_all(format!("v {} {} {}\n", shared_mesh.positions[i].x, shared_mesh.positions[i].y, shared_mesh.positions[i].z).as_bytes()).unwrap();
    }
    for i in (0..shared_mesh.triangles.len()).step_by(3) {
        file.write_all(format!("f {} {} {}\n", shared_mesh.triangles[i] + 1, shared_mesh.triangles[i + 1] + 1, shared_mesh.triangles[i + 2] + 1).as_bytes()).unwrap();
    }

    file.flush().unwrap();
}