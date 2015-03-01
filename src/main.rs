extern crate "rustc-serialize" as rustc_serialize;

mod storage;

fn main() {
    println!("Staring Wanderlust...");
    let points = storage::load_points();
    println!("{:?}", points)
}
