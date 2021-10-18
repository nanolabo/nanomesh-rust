#![allow(dead_code)]
#![allow(incomplete_include)]

include!("utils/macros.rs");

mod base;
mod utils;
mod mesh;
mod io;

use crate::base::*;
use crate::mesh::*;

use std::time::Instant;

fn main()
{
    let now = Instant::now();
    let shared_mesh = crate::io::obj::read("./sphere_flat_hp.obj");
    println!("read obj done in {} ms", now.elapsed().as_millis());

    let now = Instant::now();
    let mut mesh = ConnectedMesh::from(&shared_mesh);
    println!("to connected mesh done in {} ms", now.elapsed().as_millis());

    let now = Instant::now();
    mesh.decimate_to_ratio(0.5);
    println!("decimation done in {} ms", now.elapsed().as_millis());

    let now = Instant::now();
    let shared_mesh = SharedMesh::from(&mesh);
    println!("to shared mesh done in {} ms", now.elapsed().as_millis());

    let now = Instant::now();
    crate::io::obj::write(shared_mesh, "./output.obj");
    println!("write obj done in {} ms", now.elapsed().as_millis());
}