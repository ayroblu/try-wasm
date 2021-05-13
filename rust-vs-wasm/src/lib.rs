extern crate libtsp;
extern crate serde_json;
extern crate wasm_bindgen;

#[macro_use]
extern crate serde_derive;

mod serde_util;

use libtsp::{tsp, Point};
use serde_util::{PointDef, ResultPathDef};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    log(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn run(js_points: &JsValue) -> JsValue {
    let points_def: Vec<PointDef> = js_points.into_serde().unwrap();
    let points = points_def.iter().map(|p| Point::from(p)).collect();
    let result_path = ResultPathDef::from(&tsp(&points));
    JsValue::from_serde(&result_path).unwrap()
}
