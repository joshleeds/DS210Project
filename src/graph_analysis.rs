use std::collections::VecDeque;

// pub fn seperate_vector(vector: Vec<(usize, usize)>) -> (Vec<usize>, Vec<usize>) {
//     let mut nodes: Vec<usize> = Vec::new();
//     let mut edges: Vec<usize> = Vec::new();
//     for (node, edge) in vector {
//         nodes.push(node);
//         edges.push(edge);
//     }
//     return (nodes, edges)
// }


//This function computes a BFS for each node in the graph and returns a vector of vectors
//Which has the distances for each node to every other node in the graph
//It takes a while to run for my selected data set as it has 37000 nodes
//I used the lecture on BFS for this function and also researched online
pub fn computeALLBFS(start: usize, adj: &Vec<Vec<usize>>) -> Vec<Vec<Option<u32>>> {
    let mut all_distances: Vec<Vec<Option<u32>>> = Vec::new();
    for start_vertex in 0..adj.len() {
        let mut distance: Vec<Option<u32>> = vec![None; adj.len()];
        distance[start_vertex] = Some(0); // <= we know this distance
        let mut queue: VecDeque<usize> = VecDeque::new();
        queue.push_back(start_vertex);

        while let Some(vertex) = queue.pop_front() {
            for &current in &adj[vertex] {
                if distance[current].is_none() {
                    distance[current] = Some(distance[vertex].unwrap() + 1);
                    queue.push_back(current);
                }
            }
        }
        
        all_distances.push(distance);
        //I used this line while testing to make sure that the program was actaully running
        println!("{}",start_vertex);
    }

    all_distances
}
//This function prints ALL the BFS's for each node. IT takes a while to run 
//I seperated the function computeALLBFS and printALLBFS because running them together took too long and gave my computer issues
//I used the lecture on BFS for this function and also researched online
pub fn printALLBFS(distances: &[Vec<Option<u32>>]) {
    for (start_vertex, distances) in distances.iter().enumerate() {
        print!("BFS for node {}: ", start_vertex);
        for (v, dist) in distances.iter().enumerate() {
            print!("{}:{} ", v, dist.unwrap());
        }
        println!();
    }
}

pub fn average_path_length(all_distances: &Vec<Vec<Option<u32>>>) -> f64 {
    let mut sum_distances = 0;
    let mut count_valid_distances = 0;

    for distances in all_distances.iter() {
        for &distance in distances.iter() {
            if let Some(d) = distance {
                sum_distances += d as usize;
                count_valid_distances += 1;
            }
        }
    }

    return sum_distances as f64 / count_valid_distances as f64;
    // if count_valid_distances > 0 {
    //     return sum_distances as f64 / count_valid_distances as f64
    // } else {
    //     return 0.0 
    // }
}


//This function does a BFS to find distances for any node up to any node
//Meaning if the input is start: 10 and lastnode = 100, then it will tell the shortest connections for node 10
// From node 0 to node 100
pub fn oneBFS(start: usize, lastnode: usize, adj: &Vec<Vec<usize>>) {
    let mut distance: Vec<Option<u32>> = vec![None; adj.len()];
    distance[start] = Some(0); // <= we know this distance
    let mut queue: VecDeque<usize> = VecDeque::new();
    queue.push_back(start);
    while let Some(vertex) = queue.pop_front() {
        // new unprocessed vertex
        for &current in &adj[vertex] {
            if distance[current].is_none() {
                // consider all unprocessed neighbors of v
                distance[current] = Some(distance[vertex].unwrap() + 1);
                queue.push_back(current);
            }
        }
    }
    print!("BFS for node {} (from 0 up to node {}): ", start, lastnode);
    for (node, dist) in distance.iter().enumerate().take(lastnode + 1) {
        print!("{}:{} ", node, dist.unwrap());
    }
    println!();
}