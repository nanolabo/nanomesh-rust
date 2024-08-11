#![allow(dead_code)]
#![allow(incomplete_include)]

#[path = "../src/lib.rs"]
mod nanomesh;

use std::{time::Instant, fs::File, io::{BufReader, BufWriter}};

fn main()
{
    let input = File::open("main/samples/sphere_flat_hp.obj").unwrap();
    let mut reader: BufReader<File> = BufReader::new(input);

    let output = File::create("main/samples/output.obj").unwrap();
    let mut writer = BufWriter::new(output);

    let now = Instant::now();
    let shared_mesh = nanomesh::io::obj::read(&mut reader);
    println!("read obj done in {} ms", now.elapsed().as_millis());

    let now = Instant::now();
    let mut mesh = nanomesh::mesh::ConnectedMesh::from(&shared_mesh);
    println!("to connected mesh done in {} ms", now.elapsed().as_millis());

    let now = Instant::now();
    mesh.decimate_to_ratio(0.99);
    println!("decimation done in {} ms", now.elapsed().as_millis());

    let now = Instant::now();
    let shared_mesh = nanomesh::mesh::SharedMesh::from(&mesh);
    println!("to shared mesh done in {} ms", now.elapsed().as_millis());

    let now = Instant::now();
    nanomesh::io::obj::write(&shared_mesh, &mut writer);
    println!("write obj done in {} ms", now.elapsed().as_millis());
}