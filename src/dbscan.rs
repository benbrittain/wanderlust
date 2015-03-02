use std::num::Float;
use types::Point;

type Cluster = Vec<Point>;
//type Dataset = Vec<Point>; TODO: make an indexed dataset (O(n^2) -> O(nlog(n))

pub fn dbscan(dataset: Vec<Point>, eps: f32, min_pts: usize) -> (Vec<Cluster>, Vec<Point>)  {
    let mut clusters: Vec<Cluster> = Vec::new();
    let mut noise: Vec<Point> = Vec::new();
    let mut visited: Vec<Point> = Vec::new();

    for (index, point) in dataset.iter().enumerate() {
        if !visited.contains(&point) {
            visited.push(point.clone());
            let neighborhood = region_query(point, eps, &dataset);
            if neighborhood.len() < min_pts {
                noise.push(point.clone());
             } else {
                let mut cluster: Cluster = Vec::new();
                expand_cluster(point, &neighborhood, eps, min_pts, &dataset, &mut visited, &mut cluster);
                clusters.push(cluster);
            }
        }
    }
    (clusters, noise)
}

fn expand_cluster<'a>(point: &Point, neighborhood: &Vec<Point>, eps: f32, min_pts: usize,
                      dataset: &Vec<Point>, visited: &mut Vec<Point>, cluster: &'a mut Cluster) -> &'a Cluster{
    for neighbor in neighborhood {
        if !visited.contains(&neighbor) {
            visited.push(neighbor.clone());
            let mut new_neighborhood = region_query(&neighbor, eps, &dataset);
            if (new_neighborhood.len() >= min_pts) {
                expand_cluster(neighbor, &new_neighborhood, eps, min_pts, dataset, visited, cluster);
            }
        }

        if !cluster.contains(neighbor) {
            cluster.push(neighbor.clone())
        }
    }
    cluster
}

fn region_query(point: &Point, eps: f32, dataset: &Vec<Point>) -> Vec<Point> {
    dataset.iter().filter(|&other| distance(point, other) < eps).map(|x| x.clone()).collect()
}

fn distance(p1: &Point, p2: &Point) -> f32 {
    ((p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2)).sqrt() as f32
}
