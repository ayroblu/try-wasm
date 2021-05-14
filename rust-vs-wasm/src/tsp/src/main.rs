extern crate libtsp;

use libtsp::{tsp, Point};

fn main() {
    let points = vec![
        Point { x: 98877, y: 81007 },
        Point { x: 65637, y: 49794 },
        Point { x: 36151, y: 29682 },
        Point { x: 22902, y: 77883 },
        Point { x: 5324, y: 79475 },
        Point { x: 40309, y: 73254 },
        Point { x: 61921, y: 22686 },
        Point { x: 30675, y: 24617 },
        Point { x: 61717, y: 14688 },
        Point { x: 74171, y: 46210 },
        Point { x: 74579, y: 82316 },
        // Point { x: 51086, y: 88711 },
        // Point { x: 51480, y: 69536 },
        // Point { x: 12734, y: 94524 },
        // Point { x: 4398, y: 27160 },
        // Point { x: 44814, y: 36432 },
        // Point { x: 64124, y: 96393 },
        // Point { x: 29768, y: 67782 },
        // Point { x: 13581, y: 82293 },
        // Point { x: 12583, y: 88596 },
    ];
    let res = tsp(&points);
    println!("res: {:#?}", res);
}
