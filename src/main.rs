extern crate "rustc-serialize" as rustc_serialize;

mod storage;
mod dbscan;
mod types;

fn main() {
    println!("Staring Wanderlust...");
    let points = storage::load_points();

    let locs = dbscan::get_locs(points, 0.01, 20);
}
