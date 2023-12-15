use std::io::{BufRead, BufReader};
use std::fs::File;

mod adjacencylist;
mod graphsearch;
mod analysis;

use adjacencylist::{create_adjacency_list, print_adjacency_list};

use graphsearch::{printALLBFS, computeALLBFS, oneBFS};

use analysis::{average_path_length};

fn main() {
    let graph = reader("githubdata.csv");
    let adj = create_adjacency_list(graph);

    //This prints the adjacency list for the graph
    // print_adjacency_list(&adj);
    // for i in 0..1{//adj.len() {
    //     println!("Distances from node {}", i);
    //     compute_print_all_BFS(i, &adj);
    // }
    
    let node = 60;
    let finalnode = 200;
    let length = adj.len();
    oneBFS(node, finalnode, &adj);
    //This line prints the BFS for the selected node for all nodes. uncomment this line to print all of them
    //I commeneted this line out since their are 370000 nodes and therefor printing the BFS for all of them isnt particuarly helpful
    // oneBFS(node, length, &adj);

    let distances = computeALLBFS(0, &adj); // THIS TAKES ABOUT 8 min to run
    //printALLBFS(&distances); //Prints ALL the distances //If uncommented and ran this code take over 15 minutes to run and caused my computer
    //Memory Issues
    let pathlength = average_path_length(&distances);
    println!("Average Path Length: {}", pathlength);
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


