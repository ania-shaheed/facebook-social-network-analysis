use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashMap, HashSet};

pub fn edges(id: String) -> HashSet<(u32, u32)> {
    let file = File::open(format!("facebook_dataset/{}.edges", id)).unwrap();
    let reader = BufReader::new(file);
    let mut edges = HashSet::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut parts = line.split_whitespace();
        let node1 = parts.next().unwrap().parse().unwrap();
        let node2 = parts.next().unwrap().parse().unwrap();
        edges.insert((node1, node2));
    }

    edges
}

pub fn circles(id: String) -> HashMap<String, HashSet<u32>> {
    let file = File::open(format!("facebook_dataset/{}.circles", id)).unwrap();
    let reader = BufReader::new(file);
    let mut circles = HashMap::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut parts = line.split_whitespace();
        let circle_name = parts.next().unwrap().to_string();
        let nodes = parts.next().unwrap().split_whitespace()
            .map(|id| id.parse().unwrap())
            .collect();
        circles.insert(circle_name, nodes);
    }

    circles
}

#[derive(Debug, Clone)]
pub struct Network {
    pub id: u32,
    pub edges: HashSet<(u32, u32)>,
    pub circles: HashMap<String, HashSet<u32>>
}

impl Network {
    pub fn new(id: u32) -> Network {
        let edges = edges(id.to_string());
        let circles = circles(id.to_string());

        Network {id, edges, circles}
    }

}

pub fn read(id: u32) {
    let data: Network = Network::new(id);

    println!("\nIndividual {}", id);
    println!("Number of edges: {}", data.edges.len());
    println!("Number of circles: {}", data.circles.len());
}