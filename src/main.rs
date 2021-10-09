#![allow(dead_code)]
#![allow(incomplete_include)]

mod base;
mod utils;
mod mesh;

use crate::base::*;
use crate::mesh::*;
use crate::utils::*;

use std::time::Instant;


fn main() {

    let now = Instant::now();

    let mut mesh = ConnectedMesh::default();
    mesh.decimate(10);

    println!("ms={}", now.elapsed().as_millis());
}
