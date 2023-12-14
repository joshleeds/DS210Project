use std::collections::VecDeque;
use std::collections::HashMap;

pub fn seperate_vector(vector: Vec<(usize, usize)>) -> (Vec<usize>, Vec<usize>) {
    let mut nodes: Vec<usize> = Vec::new();
    let mut edges: Vec<usize> = Vec::new();
    for (node, edge) in vector {
        nodes.push(node);
        edges.push(edge);
    }
    return (nodes, edges)
}

pub fn create_adjacency_list(graph: Vec<(usize, usize)>) -> Vec<Vec<usize>> {
    let length = graph.iter().map(|&(node, edge)| node.max(edge)).max().unwrap_or(0) + 1; //Used the internet to get the correct length
    let mut AdjacencyLists : Vec<Vec<usize>> = vec![vec![];length];

    for &(node, edge) in &graph {
        AdjacencyLists[node].push(edge);
        AdjacencyLists[edge].push(node);  
    }

    return AdjacencyLists
}


pub fn compute_print_all_BFS(start: usize, adjacency_lists: &Vec<Vec<usize>>) {
    let mut distance: Vec<Option<u32>> = vec![None; adjacency_lists.len()];
    distance[start] = Some(0); // <= we know this distance
    let mut queue: VecDeque<usize> = VecDeque::new();
    queue.push_back(start);

    while let Some(v) = queue.pop_front() {
        // new unprocessed vertex
        for &u in &adjacency_lists[v] {
            if distance[u].is_none() {
                // consider all unprocessed neighbors of v
                distance[u] = Some(distance[v].unwrap() + 1);
                queue.push_back(u);
            }
        }
    }

    print!("vertex:distance");
    for v in 0..adjacency_lists.len() {
        print!(" {}:{} ", v, distance[v].unwrap());
    }
    println!();
}

pub fn computeBFS(start: usize, adjacency_lists: &Vec<Vec<usize>>) -> Vec<Vec<Option<u32>>> {
    let mut all_distances: Vec<Vec<Option<u32>>> = Vec::new();

    for start_vertex in 0..adjacency_lists.len() {
        let mut distance: Vec<Option<u32>> = vec![None; adjacency_lists.len()];
        distance[start_vertex] = Some(0); // <= we know this distance
        let mut queue: VecDeque<usize> = VecDeque::new();
        queue.push_back(start_vertex);

        while let Some(v) = queue.pop_front() {
            // new unprocessed vertex
            for &u in &adjacency_lists[v] {
                if distance[u].is_none() {
                    // consider all unprocessed neighbors of v
                    distance[u] = Some(distance[v].unwrap() + 1);
                    queue.push_back(u);
                }
            }
        }
        
        all_distances.push(distance);
        println!("{}",start_vertex);
    }

    all_distances
}

pub fn printBFS(distances: &[Vec<Option<u32>>]) {
    for (start_vertex, distances) in distances.iter().enumerate() {
        print!("BFS for node {}: ", start_vertex);
        for (v, dist) in distances.iter().enumerate() {
            print!("{}:{} ", v, dist.unwrap_or_default());
        }
        println!();
    }
}
