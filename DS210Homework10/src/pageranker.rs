use rand::Rng; 

pub fn steps(mut currentstep: usize, graph: &Vec<Vec<usize>>, num_vert: usize) -> usize{
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(0..10);
    if num  == 9 {
        let vertice = rng.gen_range(0..num_vert);
        currentstep = vertice
    } else {
        let neighbor = &graph[currentstep];
        let rand_neigh = rng.gen_range(0..(neighbor.len()));
        currentstep = neighbor[rand_neigh];
    }
    return currentstep;
}

pub fn walks(n: usize, graph: Vec<Vec<usize>>) -> Vec<usize> {
    let mut spot: usize;
    let mut walk_final: Vec<usize> = vec![0; n];
    for p1 in 0..n {
        spot = p1;
        for p2 in 0..100{
            for p3 in 0..100{
                spot = steps(spot, &graph, n);
            }
            walk_final[spot] = walk_final[spot] + 1;
        }
    }

    return walk_final;
}