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