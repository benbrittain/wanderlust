
extern crate gnuplot;
extern crate "rustc-serialize" as rustc_serialize;

use std::old_io::File;
use std::iter::repeat;
use gnuplot::*;
use std::rand;
use std::rand::distributions::{IndependentSample, Range};

use types::Point;

mod storage;
mod dbscan;
mod types;

fn main() {
    println!("Starting Wanderlust...");
    let points = storage::load_points();
    let (clusters, noise): (Vec<Vec<Point>>, Vec<Point>) = dbscan::dbscan(points, 0.000001, 20);

    // Temporary test output
    let mut fg = Figure::new();
    let between = Range::new(0, 255);
    let mut rng = rand::thread_rng();
    for (index, cluster) in clusters.iter().enumerate() {
        let xs = cluster.iter().map(|&point| point.x);
        let ys = cluster.iter().map(|&point| point.y);
        let x: i32  = between.ind_sample(&mut rng);
        let y: i32  = between.ind_sample(&mut rng);
        let z: i32  = between.ind_sample(&mut rng);
        fg.axes2d().points(xs, ys, &[Caption(format!("cluster {}", index).as_slice()),
                                     Color(format!("#{:x}{:x}{:x}",x,y,z).as_slice())]);
    }

    let x: i32  = between.ind_sample(&mut rng);
    let y: i32  = between.ind_sample(&mut rng);
    let z: i32  = between.ind_sample(&mut rng);
    let index_str = "noise";
    let xs = noise.iter().map(|&point| point.x);
    let ys = noise.iter().map(|&point| point.y);
    fg.axes2d().points(xs, ys, &[Caption(index_str), Color(format!("#{:x}{:x}{:x}",x,y,z).as_slice())]);

    fg.show();
}
