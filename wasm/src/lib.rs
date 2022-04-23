// cargo install wasm-pack
// wasm-pack build
// wasm-pack build --release --target web

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn read_obj(bytes: &[u8]) -> Vec<u8> {
  let mut sum = 0i32;
  for i in 0..bytes.len() {
    sum += bytes[i] as i32;
  }
  return Vec::new();
}