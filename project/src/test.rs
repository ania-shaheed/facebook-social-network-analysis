#[cfg(test)]
mod tests {
    #[test]
    fn parse() {
        let graph = Graph::parse("test_graph.txt");

        assert_eq!(graph.nodes.len(), 4);
        assert_eq!(graph.nodes[&1], vec![2, 3]);
        assert_eq!(graph.nodes[&2], vec![1, 4]);
        assert_eq!(graph.nodes[&3], vec![1]);
        assert_eq!(graph.nodes[&4], vec![2]);
    }

    #[test]
    fn edge() {
        let mut graph = Graph::new();

        graph.edge(1, 2);
        graph.edge(2, 3);

        assert_eq!(graph.nodes.len(), 3);
        assert_eq!(graph.nodes[&1], vec![2]);
        assert_eq!(graph.nodes[&2], vec![1, 3]);
        assert_eq!(graph.nodes[&3], vec![2]);
    }
}