//This function creates an adjacency list to use for BFS from the inputed graph
pub fn create_adjacency_list(graph: Vec<(usize, usize)>) -> Vec<Vec<usize>> {
    let length = graph.iter().map(|&(node, edge)| node.max(edge)).max().unwrap_or(0) + 1; //Used the internet to get the correct length
    let mut Adj : Vec<Vec<usize>> = vec![vec![];length];

    for &(node, edge) in &graph {
        Adj[node].push(edge);
        Adj[edge].push(node);  
    }
    return Adj
}

//This function is used the print the adjacency list
pub fn print_adjacency_list(adjacency_lists: &Vec<Vec<usize>>) {
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