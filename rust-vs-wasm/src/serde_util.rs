use std::collections::HashSet;

use libtsp::{IdentifiablePoint, Point, ResultPath};

#[derive(Serialize, Deserialize)]
// #[serde(remote = "Point")]
pub struct PointDef {
    x: usize,
    y: usize,
}
impl From<&PointDef> for Point {
    fn from(def: &PointDef) -> Point {
        Point { x: def.x, y: def.y }
    }
}
impl From<&Point> for PointDef {
    fn from(p: &Point) -> PointDef {
        PointDef { x: p.x, y: p.y }
    }
}

#[derive(Serialize, Deserialize)]
pub struct IdentifiablePointDef {
    id: usize,
    point: PointDef,
}
impl From<&IdentifiablePoint<'_>> for IdentifiablePointDef {
    fn from(p: &IdentifiablePoint) -> IdentifiablePointDef {
        IdentifiablePointDef {
            id: p.id,
            point: PointDef::from(p.point),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ResultPathDef {
    pub path: Vec<IdentifiablePointDef>,
    pub traveled_points: HashSet<usize>,
    pub cost: f64,
}

// Provide a conversion to construct the remote type.
impl From<&ResultPath<'_>> for ResultPathDef {
    fn from(res: &ResultPath) -> ResultPathDef {
        ResultPathDef {
            path: res
                .path
                .iter()
                .map(|p| IdentifiablePointDef::from(p))
                .collect(),
            traveled_points: res.traveled_points.clone(),
            cost: res.cost,
        }
    }
}
