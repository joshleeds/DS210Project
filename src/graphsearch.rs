use std::collections::VecDeque;

//This function computes a BFS for each node in the graph and returns a vector of vectors
//Which has the distances for each node to every other node in the graph
//It takes a while to run for my selected data set as it has 37000 nodes
//I used the lecture on BFS for this function and also researched online
pub fn computeALLBFS(adj: &Vec<Vec<usize>>) -> Vec<Vec<Option<u32>>> {
    let mut all_distances: Vec<Vec<Option<u32>>> = Vec::new();
    for node in 0..adj.len() {
        let mut distance: Vec<Option<u32>> = vec![None; adj.len()];
        distance[node] = Some(0); // <= we know this distance
        let mut queue: VecDeque<usize> = VecDeque::new();
        queue.push_back(node);
        while let Some(vertex) = queue.pop_front() {
            for &current in &adj[vertex] {
                if distance[current].is_none() {
                    distance[current] = Some(distance[vertex].unwrap() + 1);
                    queue.push_back(current);
                }
            }
        }   
        all_distances.push(distance);
        println!("{}",node);  //I added this line so the user can know when the BFS's are almost done (when the number reaches 37000)
    }
    return all_distances
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

//This function does a BFS to find distances for any node up to any node
//Meaning if the input is start: 10 and lastnode = 100, then it will tell the shortest connections for node 10
// From node 0 to node 100
pub fn oneBFS(selected: usize, lastnode: usize, adj: &Vec<Vec<usize>>) {
    let mut distance: Vec<Option<u32>> = vec![None; adj.len()];
    distance[selected] = Some(0); //VS code Rust compilior told me to do the Some(0)
    let mut queue: VecDeque<usize> = VecDeque::new();
    queue.push_back(selected);
    while let Some(vertex) = queue.pop_front() {
        for &current in &adj[vertex] {
            if distance[current].is_none() {
                distance[current] = Some(distance[vertex].unwrap() + 1);
                queue.push_back(current);
            }
        }
    }
    print!("Distances from BFS for node {} (from 0 up to node {}): ", selected, lastnode);
    for (node, dist) in distance.iter().enumerate().take(lastnode + 1) { //The internet helped me with this line
        print!("{}:{} ", node, dist.unwrap());
    }
}