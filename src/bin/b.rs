use wasm_bindgen::prelude::wasm_bindgen;
use js_sys;

#[wasm_bindgen]
pub fn abs(n: f64) -> f64 {
  return js_sys::Math::abs(n);
}

pub fn main() {
  let msg = "Hello";
  abs(1.0);
  println!("{}", msg);
}
