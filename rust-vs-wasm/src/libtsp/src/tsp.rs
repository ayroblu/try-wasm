mod util;

pub use util::{calculate_edges, IdentifiablePoint, Point, ResultPath};
// use web_sys::console;

pub fn tsp<'a>(points: &'a Vec<Point>) -> ResultPath<'a> {
    let edges = calculate_edges(&points);

    let path = ResultPath {
        path: vec![IdentifiablePoint {
            id: 0,
            point: &points[0],
        }],
        traveled_points: vec![0].into_iter().collect(),
        cost: 0.0,
    };
    run_tsp(points, edges, path)
}

// macro_rules! log {
//     // Note that this is using the `log` function imported above during
//     // `bare_bones`
//     ($($t:tt)*) => (console::log_1(&format_args!($($t)*).to_string().into()))
// }

fn run_tsp<'a>(
    points: &'a Vec<Point>,
    edges: Vec<Vec<f64>>,
    path: ResultPath<'a>,
) -> ResultPath<'a> {
    // log!("Hello using web-sys");
    // log!("Edges {:?}", edges);

    let mut working_vec: Vec<ResultPath> = vec![path];
    let mut min_path_opt: Option<ResultPath> = None;
    while working_vec.len() > 0 {
        let mut new_working_vec: Vec<ResultPath> = Vec::new();
        for p in working_vec.iter() {
            for i in 0..points.len() {
                if !p.traveled_points.contains(&i) {
                    let mut new_path = p.clone();
                    new_path.cost += edges[new_path.path.last().unwrap().id][i];
                    new_path.path.push(IdentifiablePoint {
                        id: i,
                        point: &points[i],
                    });
                    new_path.traveled_points.insert(i);
                    // log!("Cost {}", new_path.cost);
                    // log!("path {:?}", new_path.path);
                    // log!("{:?}", new_path.traveled_points);

                    if new_path.traveled_points.len() == points.len() {
                        match min_path_opt {
                            Some(ref min_path) => {
                                if min_path.cost > new_path.cost {
                                    min_path_opt = Some(new_path);
                                }
                            }
                            None => min_path_opt = Some(new_path),
                        }
                    } else {
                        if let Some(ref min_path) = min_path_opt {
                            if new_path.cost >= min_path.cost {
                                continue;
                            }
                        }
                        new_working_vec.push(new_path);
                    }
                }
            }
        }
        working_vec = new_working_vec;
    }
    min_path_opt.unwrap()
}
