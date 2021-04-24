use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use num_cpus;

use algs_stanford::rand_contraction_alg::{Edge, Graph, karger_min_cut, karger_min_cut_multi};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./examples/p4_karger_min_cut/kargerMinCut.txt")?;
    let input_raw = BufReader::new(file)
        .lines()
        .map(|line| {
            line.unwrap().split_whitespace().map(|s| s.to_string()).collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut graph = Graph::new();
    for mut line in input_raw {
        let n1 = line.swap_remove(0);
        graph.nodes.push(n1.clone());
        for n2 in line {
            let edge = Edge::new(n1.clone(), n2);
            if !graph.edges.contains(&edge) {
                graph.edges.push(edge);
            }
        }
    }

    println!("{:?}", graph);
    let min_cut = karger_min_cut(&graph);
    println!("min_cut run once: {}", min_cut);
    let min_cut = karger_min_cut_multi(&graph);
    println!("min_cut run n * {} times (n={}): {}", num_cpus::get(), graph.nodes.len(), min_cut);
    // result seems to be 17?
    // output: min_cut run n * 12 times (n=200): 17
    Ok(())
}