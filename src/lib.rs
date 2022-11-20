use wasm_bindgen::prelude::*;
use js_sys::Math;

#[wasm_bindgen]
pub fn abs(n: f64) -> f64 {
  Math::abs(n)
}

// #[wasm_bindgen]
// pub fn abs(n: f64) -> f64 {
//   n + 1.0
// }

// #[no_mangle]
// pub extern "C" fn add(x: i32) -> i32 {
//     // x + 1
//     Math::abs(x as f64) as i32
// }