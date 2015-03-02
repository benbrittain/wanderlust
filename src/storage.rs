extern crate csv;
use types::Point;

//#[derive(Debug)]
//pub struct Point {
//    lat: f64,
//    long: f64,
//    time: i64
//}
//
//impl Point {
//    fn new(ts: i64, x: f64, y: f64) -> Point {
//        Point {
//            lat: x,
//            long: y,
//            time: ts
//        }
//    }
//}

pub fn load_points() -> Vec<Point> {
    let fp = &Path::new("./data/gpsdata_simple.csv");
    let mut rdr = csv::Reader::from_file(fp);

    let mut points = vec![];
    for record in rdr.decode() {
        let (timestamp, long, lat, _, _, _, _) : (i64, f64, f64, f32, f32, f32, f32) = record.unwrap();
        points.push(Point::new(lat, long, timestamp))
    }
    points
}
