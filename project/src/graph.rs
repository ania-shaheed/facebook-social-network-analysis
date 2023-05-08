use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Graph {
    pub nodes: HashMap<u32, Vec<u32>>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            nodes: HashMap::new(),
        }
    }

    pub fn edge(&mut self, u: u32, v: u32) {
        self.nodes.entry(u).or_insert(vec![]).push(v);
        self.nodes.entry(v).or_insert(vec![]).push(u);
    }

    pub fn parse(filename: &str) -> Graph {
        let file = File::open(filename).expect("Could not open file");
        let reader = BufReader::new(file);

        let lines = reader.lines().map(|l| l.unwrap());

        let mut graph = Graph::new();

        for line in lines {
            let mut parts = line.split_whitespace();
            let u = parts.next().unwrap().parse().unwrap();
            let v = parts.next().unwrap().parse().unwrap();
            graph.edge(u, v);
        }

        graph
    }
}