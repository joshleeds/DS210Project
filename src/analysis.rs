use std::collections::VecDeque;

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
}
//This function finds the nodes in the data set which have the highest degrees of freedom away from each other
//This works by finding all of the distances between every node and iterating through them
//It outputs a vector with all of node pairs with the furthest distance
pub fn furthest(all_distances: &Vec<Vec<Option<u32>>>) -> Vec<(usize, usize, u32)> {
    let mut max_distance = 0;
    let mut furthest = Vec::new();

    for (v1, distances) in all_distances.iter().enumerate() {
        for (v2, &distance) in distances.iter().enumerate() {
            if let Some(d) = distance {
                if d > max_distance {
                    max_distance = d;
                    furthest.clear(); //This clears the vector when nodes with a largest distance are found
                    furthest.push((v1, v2, max_distance));
                } else if d == max_distance {
                    furthest.push((v1, v2, max_distance));
                }
            }
        }
    }

    return furthest
}