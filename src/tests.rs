use super::floyd_warshall;

#[test]
fn test_no_intermediate() {
    use petgraph::Graph;
    let mut graph = Graph::new_undirected();

    let a = graph.add_node(0);
    let b = graph.add_node(1);
    let c = graph.add_node(2);
    let d = graph.add_node(3);

    graph.extend_with_edges(
        &[
            (a, b, 1usize),
            (a, c, 1usize),
            (a, d, 1usize),
            (b, c, 1usize),
            (b, d, 1usize),
            (c, d, 1usize),
        ],
    );

    let m = floyd_warshall(&graph);
    println!("{:?}", m);

    for i in 0..4 {
        for j in 0..4 {
            if i == j {
                assert_eq!(m.get_path_len(i, j), 0);
            } else {
                assert_eq!(m.get_path_len(i, j), 1);
            }
        }
    }
}

#[test]
fn test_intermediate() {
    use petgraph::Graph;
    let mut graph = Graph::new_undirected();

    let a = graph.add_node(0);
    let b = graph.add_node(1);
    let c = graph.add_node(2);

    graph.extend_with_edges(&[(a, b, 1usize), (b, c, 1), (a, c, 3)]);

    let m = floyd_warshall(&graph);
    println!("{:?}", m);

    assert_eq!(m.get_path_len(0, 0), 0);
    assert_eq!(m.get_path_len(1, 1), 0);
    assert_eq!(m.get_path_len(2, 2), 0);

    assert_eq!(m.get_path_len(0, 1), 1);
    assert_eq!(m.get_path_len(1, 2), 1);
    assert_eq!(m.get_path_len(0, 2), 2);
}

// #[test]
// fn test_cache_friendliness() {
//     use petgraph::Graph;

//     let mut graph: Graph<usize, usize, _, _> = Graph::new_undirected();
//     for i in 0..10 {
//         graph.add_node(i);
//     }
//     let dists = floyd_warshall(&graph);

//     println!("{:?}", dists);
// }

#[test]
#[ignore]
fn test_random() {
    use petgraph::Graph;
    use rand;
    use rand::Rng;

    let mut graph = Graph::new_undirected();
    let mut rng = rand::thread_rng();
    let mut vec = Vec::new();

    for i in 0..10 {
        vec.push(graph.add_node(i));
    }

    for v1 in &vec {
        for v2 in &vec {
            if v1 != v2 && rng.next_f32() < 0.1 {
                let w = (rng.next_u64() as usize) % 100;
                graph.add_edge(*v1, *v2, w);
            }
        }
    }

    let m = floyd_warshall(&graph);
    println!("{:?}", m);

    use petgraph::dot::Dot;
    use std::fs::File;
    use std::io::prelude::*;

    let mut file = File::create("random.dot").unwrap();
    let b = format!("{:?}", Dot::new(&graph));
    let b = b.as_bytes();
    file.write_all(b).unwrap();

    loop {
        let i: usize = read!();
        let j: usize = read!();

        if m.does_path_exist(i, j) {

            let len = m.get_path_len(i, j);
            let path = m.get_path_iter(i, j);

            let path: Vec<&usize> = if i > j {
                path.rev().collect()
            } else {
                path.collect()
            };

            println!("The path from {} to {} has total length {}.", i, j, len);
            println!("Path: {:?}", path);
        } else {
            println!("There is no path from {} to {}.", i, j)
        }
    }
}