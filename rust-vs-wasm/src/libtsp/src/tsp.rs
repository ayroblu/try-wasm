pub struct Point {
    pub x: u32,
    pub y: u32,
}

pub fn tsp(points: Vec<Point>) -> usize {
    let edges = calculate_edges(&points);
    points.len()
}

fn calculate_edges(points: &Vec<Point>) -> Vec<Vec<f64>> {
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
    (((point1.x - point2.x).pow(2) + (point1.y - point2.y).pow(2)) as f64).sqrt()
}
