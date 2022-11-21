use wasm_bindgen::prelude::*;
use js_sys::{Math, eval as eval_js, decode_uri_component, JsString, Array, Map};
use web_sys::{console};

#[wasm_bindgen]
pub fn abs(n: f64) -> f64 {
  Math::abs(n)
}

#[wasm_bindgen]
pub fn eval(code: &str) -> Result<JsValue, JsValue> {
  eval_js(code)
}

#[wasm_bindgen]
pub fn decode(s: &str) -> Result<JsString, JsValue> {
  decode_uri_component(s)
}

#[wasm_bindgen(start)]
pub fn run() {
  let msg = Array::new();
  msg.push(&JsValue::from("hello, this is out from rust"));
  console::log(&msg);

  #[wasm_bindgen]
  struct Person {
    name: String,
    age: u32,
  }

  let person = Person {
    name: "xx".to_string(),
    age: 10,
  };

  msg.push(&JsValue::from(person.name).into());
  console::log(&msg);

  let js_map = Map::new();
  let key = JsValue::from_str("name");
  let value = JsValue::from_str("xx");
  js_map.set(&key, &value);
  msg.push(&js_map);
  console::log(&msg);
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