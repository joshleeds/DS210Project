//Collaborators
//Paul, Yana
//https://stackoverflow.com/questions/40091161/sorting-a-vector-of-tuples-needs-a-reference-for-the-second-value
//https://gist.github.com/mehmetsefabalik/2f07df9bcc902d6149a80ad0ccfd5cde
mod pageranker;
use std::io::{BufRead, BufReader};
use std::fs::File;
use pageranker::steps;
use pageranker::walks;


fn main() {
    let (size, tuple) = read_file("data.txt");
    println!("number of nodes: {}", size);

    let edge: Vec<(usize, usize)> = tuple;
    let mut graph_list: Vec<Vec<usize>> = vec![vec![]; size];
    for (v, w) in edge.iter() {
        graph_list[*v].push(*w);
    }
    let walk_list = walks(size, graph_list);
    let mut newVect: Vec<(usize, f64)> = Vec::new();
    for (index, &value) in walk_list.iter().enumerate() {
        let percentage = value as f64 / 100000.0;
        newVect.push((index,percentage));
        //println!("walk_list[{}] = {:.5}", index, percentage);
    }

    newVect.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap()); // i looked up the sorting on github and stackoverflow
    for (_rank, (index, percentage)) in newVect.iter().take(5).enumerate() {
        println!("Vertex {}: approximate page rank {:.5}", index, percentage);
    }

}
#[test]
fn tests() { //this test checks if all percentage of page ranks add to 1
    let (number, tuple) = read_file("data.txt");
    let edge: Vec<(usize, usize)> = tuple;
    let mut graph_list: Vec<Vec<usize>> = vec![vec![]; number];
    for (v, w) in edge.iter() {
        graph_list[*v].push(*w);
    }
    let walk_list = walks(number, graph_list);
    let mut sum: f64 = 0.0;
    for (index, &value) in walk_list.iter().enumerate() {
        let percentage = value as f64 / 100000.0;
        sum +=percentage;
    }
    let dif = (sum - 1.0).abs();
    println!("{}",dif);
    assert!(dif < 0.000001) 
    

}

fn read_file(path: &str) -> (usize, Vec<(usize, usize)>) {

    let mut result: Vec<(usize, usize)> = Vec::new();
    let file = File::open(path).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(file).lines();
    let mut number: usize = 0;
    for (line_number, line) in buf_reader.enumerate() {
        let line_str = line.expect("Error reading");
        let mut iter = line_str.trim().split_whitespace();

        if line_number == 0 {
            number = iter.next().expect("Error parsing number of vertices").parse::<usize>().expect("Error parsing number of vertices");
        } 
        else {
        //let line_str = line.expect("Error reading");
        let v: Vec<&str> = line_str.trim().split(' ').collect();
        let x: usize = v[0].parse::<usize>().unwrap();
        let y: usize = v[1].parse::<usize>().unwrap();
        result.push((x, y));
        }
    }

    return (number, result);
}