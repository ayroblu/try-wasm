extern crate serde_json;
extern crate wasm_bindgen;

#[macro_use]
extern crate serde_derive;

mod tsp;

use tsp::{tsp, Point};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn run(js_points: &JsValue) -> usize {
    let points: Vec<Point> = js_points.into_serde().unwrap();
    tsp(points)
}
