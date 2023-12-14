mod graph_analysis;
use std::io::{BufRead, BufReader};
use std::fs::File;
use graph_analysis::create_adjacency_list;
use graph_analysis::compute_print_all_BFS;
use graph_analysis::printBFS;
use graph_analysis::computeBFS;
fn main() {

    let graph = reader("githubdata.csv");
    let adj = create_adjacency_list(graph);
    // print_adjacency_list(&adj);
    // for i in 0..1{//adj.len() {
    //     println!("Distances from node {}", i);
    //     compute_print_all_BFS(i, &adj);
    // }

    let distances = computeBFS(0, &adj);
    printBFS(&distances);
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

//This function is used the print the adjacency list
fn print_adjacency_list(adjacency_lists: &Vec<Vec<usize>>) {
    //To print the entire adjacency list
    for (node, neighbors) in adjacency_lists.iter().enumerate() {
        println!("Node {}: {:?}", node, neighbors);
    }
    //If you want to print the first x number of nodes enter a number in the .take() and uncomment this out and 
    //comment the first for loop out
    // for (node, neighbors) in adjacency_lists.iter().enumerate().take(//ENTER NUMBER) {
    //     println!("Node {}: {:?}", node, neighbors);
    // }
}
