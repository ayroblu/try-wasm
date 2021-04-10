extern crate libtsp;
extern crate serde_json;
extern crate wasm_bindgen;

#[macro_use]
extern crate serde_derive;

use libtsp::{tsp, Point};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
// #[serde(remote = "Point")]
struct PointDef {
    x: u32,
    y: u32,
}
// Provide a conversion to construct the remote type.
impl From<&PointDef> for Point {
    fn from(def: &PointDef) -> Point {
        Point { x: def.x, y: def.y }
    }
}

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
    let points_def: Vec<PointDef> = js_points.into_serde().unwrap();
    let points = points_def.iter().map(|p| Point::from(p)).collect();
    tsp(points)
}
