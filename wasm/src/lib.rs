// cargo install wasm-pack
// wasm-pack build
// wasm-pack build --release --target web

extern crate console_error_panic_hook;
use wasm_bindgen::prelude::*;
use std::io::BufWriter;
use log::{Level};

#[wasm_bindgen]
pub struct Parameters {
  pub polygon_reduction: f32,
  pub export_format: u32,
}

#[wasm_bindgen]
impl Parameters {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Parameters {
    Parameters {
      export_format: 0,
      polygon_reduction: 0.0 
    }
  }
}

#[wasm_bindgen(module = "/callbacks.js")]
extern {
  fn set_progress(progress: f32, status: &str);
}

#[wasm_bindgen]
pub fn setup_logging() {
  console_error_panic_hook::set_once();
  console_log::init_with_level(Level::Info).expect("Failed to initialize log");
}

#[wasm_bindgen]
pub fn read_obj(parameters: &Parameters, bytes: &[u8]) -> Vec<u8> {

  set_progress(0., "Reading...");

  //let slice: &[u8] = &bytes[..];
  //let mut reader = BufReader::new(slice);

  use step::step_file::StepFile;
  use triangulate::triangulate::triangulate; // lol

  set_progress(0.25, "Parsing...");
  let flat = StepFile::strip_flatten(bytes);
  let step = StepFile::parse(&flat);

  set_progress(0.5, "Tesselating...");
  let (mesh, _stats) = triangulate(&step);

  set_progress(0.75, "Writing...");
  let mut result = Vec::new();
  {
    let mut writer = BufWriter::new(&mut result);
    nanomesh::io::obj::write(&mesh, &mut writer);
  }

  set_progress(1., "Done!");
  return result;
}