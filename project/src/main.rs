mod read;
use read::Network;
mod graph;
use graph::Graph;
mod friends;

fn main() {
    //identifiers for 10 individuals in facebook dataset
    let ids = [0, 107, 348, 414, 686, 698, 1684, 1912, 3437, 3980];
    
    //reading data into memory
    println!("-----Data Overview-----");
    let graph: Graph = Graph::parse("facebook_combined.txt");

    let mut users: Vec<Network> = Vec::new();
    for i in ids {
        read::read(i as u32);
        users.push(Network::new(i as u32));
    }

    println!("\n-----Comparing Circles within Indiviudals' Networks-----");
    let mut friends_of_friends: Vec<(u32, f64)> = Vec::new();

    for i in users {
        let user_id = i.id;

        //friends of friends calculations per individual
        friends_of_friends.push((user_id, friends::friends_of_friends(&i, &graph)));

        //comparing circles per individual
        let mut similarities = friends::compare_circles(i);

        let mut overlapping = true;
        println!("\nIndividual {}:", user_id);
        for j in &similarities {
            if j.2 > 0.0 {
                overlapping = false
            } else {continue}
        } 

        if overlapping == true {
            println!("No Overlapping Circles:");
        } else {
            similarities.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
            println!("Most similar circles:");
            println!("{:?}", similarities[0]);

            similarities.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
            println!("Lease similar circles:");
            println!("{:?}", similarities[0]);
        }
        
    }
    
    println!("\n-----Friends of Friends Similarities-----");
    println!("\nHow often are friends of my friends my friends?");
    println!("{:?}", friends_of_friends);
    
    //comparing sets of friends
    let friend_similarities = friends::compare_users(graph);

    let mut most_similar = friend_similarities.clone();
    most_similar.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
    println!("\nMost similar users (user 1, user2, similarity):");
    println!("{:?}", most_similar[0]);

    let mut most_dissimilar = friend_similarities.clone();
    most_dissimilar.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    println!("\nMost dissimilar users (user 1, user2, similarity):");
    println!("{:?}", most_dissimilar[0]);
}