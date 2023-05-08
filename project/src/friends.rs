use crate::graph::Graph;
use crate::read::Network;
use std::collections::HashSet;

// Function to find how often friends of friends are friends
pub fn friends_of_friends(network: &Network, graph: &Graph) -> f64 {
    // Get the set of ids of the user's friends
    let friends: &Vec<u32> = &graph.nodes[&network.id];

    // Get the set of ids of the user's friends' friends
    let mut friends_of_friends: HashSet<u32> = HashSet::new();
    for i in friends {
        let list: &Vec<u32> = graph.nodes.get(&i).unwrap();
        friends_of_friends.extend(list.iter().cloned());
    }

    // Get the set of ids of the user's friends' friends that are also the user's friends
    let common_friends: Vec<&u32> = friends_of_friends.iter().filter(|&x| friends.contains(x)).collect();

    // Calculate the ratio of common friends of friends to total friends of friends
    let num_friends_of_friends = friends_of_friends.len() as f64;
    let num_common_friends = common_friends.len() as f64;
    let fraction = num_common_friends / num_friends_of_friends;

    fraction
}

// Function to find the users who have the most similar sets of friends
pub fn compare_users(graph: Graph) -> Vec<(u32, u32, f64)> {
    let mut similarities: Vec<(u32, u32, f64)> = Vec::new();
    let mut skip: HashSet<u32> = HashSet::new();

    for (user1, friends1) in &graph.nodes {
        skip.insert(*user1);

        for (user2, friends2) in &graph.nodes {
            if skip.contains(user2) {
                continue;
            }

            // Calculate the similarity
            let common_friends: Vec<&u32> = friends1.iter().filter(|&x| friends2.contains(x)).collect();

            let mut users_union = friends1.clone();
            for x in friends2 {
                if !users_union.contains(x) {
                    users_union.push(*x);
                }
            }

            let intersection = common_friends.len() as f64;
            let union_len = users_union.len() as f64;
            let similarity = intersection / union_len;

            similarities.push((*user1, *user2, similarity));
        }
    }
    
    similarities
}

// Function to find the circles who have the most in common
pub fn compare_circles(network: Network) -> Vec<(String, String, f64)> {
    
    let mut similarities: Vec<(String, String, f64)> = Vec::new();
    let mut skip: HashSet<String> = HashSet::new();

    for (circle1, nodes1) in &network.circles {
        skip.insert(circle1.to_owned());

        for (circle2, nodes2) in &network.circles {
            if skip.contains(circle2) {
                continue;
            }

            // Calculate the Jaccard similarity coefficient
            let common_nodes: Vec<&u32> = nodes1.iter().filter(|&x| nodes2.contains(x)).collect();

            let mut users_union = nodes1.clone();
            for x in nodes2 {
                if !users_union.contains(x) {
                    users_union.insert(*x);
                }
            }

            let intersection = common_nodes.len() as f64;
            let union_len = users_union.len() as f64;
            let similarity = intersection / union_len;

            similarities.push((circle1.to_owned(), circle2.to_owned(), similarity));
        }
    }
    
    similarities
}