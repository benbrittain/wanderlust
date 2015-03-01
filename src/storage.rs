extern crate csv;

#[derive(Debug)]
struct Point {
    lat: f64,
    long: f64,
    time: i64
}

impl Point {
    fn new(ts: i64, x: f64, y: f64) -> Point {
        Point {
            lat: x,
            long: y,
            time: ts
        }
    }
}

#[derive(RustcDecodable)]
struct Record {
    s1: String,
    s2: String,
    dist: Option<u32>,
}

pub fn load_points() -> Vec<Point> {
    let fp = &Path::new("./data/gpsdata_simple.csv");
    let mut rdr = csv::Reader::from_file(fp);

    let mut points = vec![];
    for record in rdr.decode() {
        let (timestamp, long, lat, _, _, _, _) : (i64, f64, f64, f32, f32, f32, f32) = record.unwrap();
        points.push(Point::new(timestamp, lat, long))
    }
    points
}
