extern crate "rustc-serialize" as rustc_serialize;

mod storage;
mod dbscan;
mod types;

fn main() {
    println!("Staring Wanderlust...");
    let points = storage::load_points();

    let locs = dbscan::dbscan(points, 0.0000001, 20);
    println!("{:?}", locs);
    println!("{:?}", locs.len());
}
