use std::collections::HashSet;

#[derive(Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone)]
pub struct IdentifiablePoint<'a> {
    pub id: usize,
    pub point: &'a Point,
}

#[derive(Debug, Clone)]
pub struct ResultPath<'a> {
    pub path: Vec<IdentifiablePoint<'a>>,
    pub traveled_points: HashSet<usize>,
    pub cost: f64,
}

pub fn calculate_edges(points: &Vec<Point>) -> Vec<Vec<f64>> {
    points
        .iter()
        .map(|point| {
            points
                .iter()
                .map(|p2| distance_between_points(point, p2))
                .collect()
        })
        .collect()
}

fn distance_between_points(point1: &Point, point2: &Point) -> f64 {
    ((get_squared_distance_usize(point1.x, point2.x)
        + get_squared_distance_usize(point1.y, point2.y)) as f64)
        .sqrt()
}

fn get_squared_distance_usize(a: usize, b: usize) -> u64 {
    get_squared_distance(a as u64, b as u64)
}
fn get_squared_distance(a: u64, b: u64) -> u64 {
    if a >= b {
        (a - b).pow(2)
    } else {
        (b - a).pow(2)
    }
}
