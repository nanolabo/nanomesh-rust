// cargo install wasm-pack
// wasm-pack build

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(n1 : i32, n2: i32) -> i32 {
  return n1 + n2;
}