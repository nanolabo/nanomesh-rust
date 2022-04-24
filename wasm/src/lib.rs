// cargo install wasm-pack
// wasm-pack build
// wasm-pack build --release --target web

use wasm_bindgen::prelude::*;
use std::io::BufWriter;
use std::io::BufReader;

#[wasm_bindgen(module = "/callbacks.js")]
extern {
    fn set_progress(progress: f32, status: &str);
}

#[wasm_bindgen]
pub fn read_obj(bytes: &[u8]) -> Vec<u8> {

  let mut result = Vec::new();

  {
    set_progress(0., "Reading...");
  
    let slice: &[u8] = &bytes[..];
    let mut reader = BufReader::new(slice);

    let mesh = nanomesh::io::obj::read(&mut reader);

    set_progress(0.5, "Writing...");
    
    {
      let mut writer = BufWriter::new(&mut result);
      nanomesh::io::obj::write(&mesh, &mut writer);
    }
  }

  set_progress(1., "Done!");
  return result;
}