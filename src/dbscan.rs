use storage;
use types::Point;

type Cluster = Vec<Point>;

pub fn get_locs(dataset: Vec<storage::Point>, eps: f32, min_pts: usize) {
    let mut dataset = vec![Point::new(0.0, 1.0)];
    dbscan(dataset, eps, min_pts)
}

fn dbscan(dataset: Vec<Point>, eps: f32, min_pts: usize) {
    let mut dataset = vec![Point::new(0.0, 1.0)];

    let mut clusters: Vec<Cluster> = Vec::new();
    let mut noise: Vec<Point> = Vec::new();
    let mut visited: Vec<Point> = Vec::new();

    for (index, point) in dataset.iter().enumerate() {
        if !visited.contains(&point) {
            visited.push(point.clone());
            let neighborhood = region_query(point, eps);
            if neighborhood.len() > min_pts {
                noise.push(point.clone());
             } else {
                let mut cluster: Cluster = Vec::new();
                expand_cluster(point, &neighborhood, eps, min_pts, &mut visited, &mut cluster);
                clusters.push(cluster);
            }
        }
    }
    print!("{:?}", clusters);
}

fn expand_cluster<'a>(point: &Point, neighborhood: &Vec<Point>, eps: f32, min_pts: usize, visited: &mut Vec<Point>, cluster: &'a mut Cluster) -> &'a Cluster{
    for neighbor in neighborhood {
        if !visited.contains(&neighbor) {
            visited.push(neighbor.clone());
            let mut new_neighborhood = region_query(&neighbor, eps);
            if (new_neighborhood.len() >= min_pts) {
                expand_cluster(neighbor, &new_neighborhood, eps, min_pts, visited, cluster);
            }
        }

        if !cluster.contains(neighbor) {
            cluster.push(neighbor.clone())
        }
    }
    cluster
}

fn region_query(point: &Point, eps: f32) -> Vec<Point> {
    vec![Point::new(0.0, 1.0)]
}
