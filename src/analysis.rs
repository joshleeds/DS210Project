
pub fn average_distance(distances: &Vec<Vec<Option<u32>>>) -> f64 {
    let mut total = 0;
    let mut count: u32 = 0;

    for distance in distances.iter() {
        for &current in distance.iter() {
            if let Some(current) = current {
                total += current as usize;
                count += 1;
            }
        }
    }
    let average = total as f64 / count as f64;
    return average;
}
//This function finds the nodes in the data set which have the highest degrees of freedom away from each other
//This works by finding all of the distances between every node and iterating through them
//It outputs a vector with all of node pairs with the furthest distance
pub fn furthest(all_distances: &Vec<Vec<Option<u32>>>) -> Vec<(usize, usize, u32)> {
    let mut max = 0;
    let mut furthest = Vec::new();
    for (nodeone, distances) in all_distances.iter().enumerate() {
        for (nodetwo, &distance) in distances.iter().enumerate() {
            if let Some(d) = distance {
                if d > max {
                    max = d;
                    furthest.clear(); 
                    furthest.push((nodeone, nodetwo, max));
                } else if d == max {
                    furthest.push((nodeone, nodetwo, max));
                }
            }
        }
    }
    return furthest
}

//This function calculates the distribution for distances between nodes in the graph
//The user inputs the amount of degrees they want to know the distribution for 
//And the function returns the percentage of total connections occur with in x degrees of seperation
pub fn degree_distribution(distances: &Vec<Vec<Option<u32>>>, degrees: u32) -> f64 {
    let mut total = 0;
    let mut target = 0;
    for (nodeone, node) in distances.iter().enumerate() {
        for (nodetwo, &d) in node.iter().enumerate() {
            if nodeone != nodetwo { //This gets rid of nodes with distance 0 
                if let Some(distance) = d { //The internet helped me with this line of code
                    total += 1;
                    if distance == degrees {
                        target += 1;
                    }
                }
            }
        }
    }
    let percent = (target as f64 / total as f64) * 100.0;
    return percent;
}