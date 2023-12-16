use std::io::{BufRead, BufReader};
use std::fs::File;

mod adjacencylist;
mod graphsearch;
mod analysis;

use adjacencylist::{create_adjacency_list, print_adjacency_list};
use graphsearch::{printALLBFS, computeALLBFS, oneBFS};
use analysis::{average_distance, furthest, degree_distribution};

fn main() {
    //Reads the graph and creates and adjacency list
    let graph = reader("githubdata.csv");
    let adj = create_adjacency_list(graph);

    //This prints the adjacency list for the graph Uncomment it to see the whole adjacency list - I commeneted it out because it is fairly large
    //print_adjacency_list(&adj);
    
    
    //This does a BFS for a selected node and prints out the distances from node 0 to node "finalnode" for the selected node
    let node = 10;
    let finalnode = 145;
    let length = adj.len();
    oneBFS(node, finalnode, &adj);

    //This line prints the BFS for the selected node for all nodes. uncomment this line to print all of them
    //I commeneted this line out since their are 370000 nodes and therefor printing the BFS for all of them isnt particuarly helpful
    // oneBFS(node, length, &adj);
    
    //This does a BFS to calculate distances between every node
    // THIS TAKES ABOUT 8 min to run
    let distances = computeALLBFS( &adj); 

    //printALLBFS(&distances); 
    //Prints ALL the distances If uncommented and ran this code take over 15 minutes to run and caused my computer
    //Memory Issues

    //Calculates the average distances
    let averagedistance = average_distance(&distances);
    println!("Average Distance: {}", averagedistance);

    //Calculated the furthest nodes in terms of degrees of seperation away from each other
    let furthest_nodes = furthest(&distances);
    println!("Nodes with the maximum distance are ");
    for (node1, node2, furthestdistance) in furthest_nodes {
        println!("node {}, node {} with distance {}", node1, node2, furthestdistance);
    }

    //Calculates the percentage for node 1 - 7
    for target_degree in 1..=7 {
        // Calculate and print the percentage of nodes at the current distance
        let percentage = degree_distribution(&distances, target_degree);
        println!("Percentage of nodes at distance {}: {:.2}%", target_degree, percentage);
    }
}

//Reads the graph
fn reader(path: &str) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = Vec::new();
    let file = File::open(path).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(file).lines();
    for line in buf_reader {
        let line_str = line.expect("Error reading");
        let v: Vec<&str> = line_str.trim().split(',').collect();
        let x = v[0].parse::<usize>().unwrap();
        let y = v[1].parse::<usize>().unwrap();
        result.push((x, y));
    }
    return result;
}


#[test]
fn average_distancetest(){
    //Test that the average_distance function works
    let graph: Vec<(usize, usize)> = vec![(0, 1),(0, 2),(1,2),(2, 3),(2,4),(3, 4),(4,5),(1,6)]; //I got this graph from a lecture
    let adj = create_adjacency_list(graph);
    let distances = computeALLBFS(&adj);
    let avg = average_distance(&distances);
    let actaul = 1.538; //I got this number by drawing out the graph and calculating the average by hand
    println!("{}", avg);
    let difference = (avg-actaul).abs();
    assert!(difference > 0.001);
}
#[test]
fn distancetest(){
    //Tests the furthest function to make sure the current furtherst distance is calculated 
    let graph: Vec<(usize, usize)> = vec![(0, 1),(0, 2),(1,2),(2, 3),(2,4),(3, 4),(4,5),(1,6)];
    let adj = create_adjacency_list(graph);
    let distances = computeALLBFS(&adj);
    let furthest = furthest(&distances);
    //Calculated true furthest distance
    let truefurthest: Vec<(usize, usize, u32)> = vec![(5, 6, 4), (6, 5, 4)];
    assert_eq!(furthest, truefurthest);
}
