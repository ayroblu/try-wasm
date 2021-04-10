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
            (0..points.len())
                .map(|idx| distance_between_points(point, &points[idx]))
                .collect()
        })
        .collect()
}

fn distance_between_points(point1: &Point, point2: &Point) -> f64 {
    ((get_squared_distance(point1.x, point2.x) + get_squared_distance(point1.y, point2.y)) as f64)
        .sqrt()
}

fn get_squared_distance(a: usize, b: usize) -> usize {
    if a >= b {
        (a - b).pow(2)
    } else {
        (b - a).pow(2)
    }
}