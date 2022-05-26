use super::super::mesh::SharedMesh;

use std::io::BufWriter;
use std::io::prelude::*;
use std::convert::TryFrom;

// Binary STL https://fr.wikipedia.org/wiki/Fichier_de_st%C3%A9r%C3%A9olithographie
pub fn write<T: Write>(shared_mesh: &SharedMesh, writer: &mut BufWriter<T>) {

    // Header
    writer.write(&['x' as u8; 80]).unwrap();

    let u: u32 = u32::try_from(shared_mesh.triangles.len())
        .expect("Cannot handle more than 4,294,967,295 triangles");

    writer.write(&u.to_le_bytes()).unwrap();

    for triangle in shared_mesh.triangles.iter() {
        writer.write(&[0; 12]).unwrap(); // Triangle normal (todo)
        for v in triangle.iter() {
            let v = shared_mesh.positions[*v as usize];
            writer.write(&(v.x as f32).to_le_bytes()).unwrap();
            writer.write(&(v.y as f32).to_le_bytes()).unwrap();
            writer.write(&(v.z as f32).to_le_bytes()).unwrap();
        }
        writer.write(&[0; 2]).unwrap(); // Mot de contrôle (wat?)
    }
}