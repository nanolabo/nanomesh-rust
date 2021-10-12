#![allow(dead_code)]
#![allow(incomplete_include)]

mod base;
mod utils;
mod mesh;

use crate::base::*;
use crate::mesh::*;
use crate::utils::*;

use std::time::Instant;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {

    let now = Instant::now();

    let mut positions = Vec::<Vector3>::new();
    let mut triangles = Vec::<i32>::new();

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./sphere_no_normals.obj") {
        println!("start parting");
        for line in lines {
            if let Ok(l) = line {
                let split = l.split(" ").collect::<Vec<&str>>();
                match split[0] {
                    "v" => {
                        let position = Vector3::new(split[1].parse::<f64>().unwrap(), split[2].parse::<f64>().unwrap(), split[3].parse::<f64>().unwrap());
                        positions.push(position);
                    },
                    "f" => {
                        triangles.push(split[1].parse::<i32>().unwrap() - 1);
                        triangles.push(split[2].parse::<i32>().unwrap() - 1);
                        triangles.push(split[3].parse::<i32>().unwrap() - 1);
                    },
                    _ => ()
                }
            }
        }
        println!("end parsing");
    }

    let shared_mesh = SharedMesh {
        positions: positions,
        triangles: triangles,
        normals: Vec::new(),
        groups: Vec::new(),
    };
    println!("shared mesh built");

    let mut mesh = ConnectedMesh::from(&shared_mesh);
    println!("connected mesh built");
    mesh.decimate(1000);
    println!("decimated");

    println!("ms={}", now.elapsed().as_millis());
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}