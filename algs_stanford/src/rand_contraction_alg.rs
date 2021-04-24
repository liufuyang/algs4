use num_cpus;
use rand::prelude::thread_rng;
use rand::Rng;
use rayon::prelude::*;

#[derive(Clone, Debug)]
pub struct Graph {
    pub nodes: Vec<String>,
    pub edges: Vec<Edge>,
}

#[derive(Clone, Debug)]
pub struct Edge {
    pub n1: String,
    pub n2: String,
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        if self.n1 == other.n1 && self.n2 == other.n2 || (
            self.n2 == other.n1 && self.n1 == other.n2
        ) {
            return true;
        }
        false
    }
}

/// Return a very possible minimum cut number to split graph into 2 graphs
/// after doing n * nun_cpus times karger_min_cut guesses
pub fn karger_min_cut_multi(graph: &Graph) -> usize {
    let num_cpus = num_cpus::get();
    let one_call = |thread| {
        let mut min = graph.edges.len();
        let loop_limit = graph.nodes.len();
        for i in 0..loop_limit {
            println!("thread: {}, iter: {}/{}", thread, i, loop_limit);
            let min_cut = karger_min_cut(&graph);
            if min_cut < min {
                min = min_cut;
            }
        }
        return min;
    };
    (1..num_cpus).collect::<Vec<usize>>()
        .par_iter()
        .map(|t| one_call(t))
        .min()
        .unwrap_or(graph.edges.len())
}

/// Return a possible minimum cut number to split graph into 2 graphs
pub fn karger_min_cut(graph: &Graph) -> usize {
    let mut graph = graph.clone();
    if graph.nodes.len() == 2 {
        return graph.edges.len();
    }

    while graph.nodes.len() > 2 {
        let edge = random_choose(&mut graph.edges);
        // remove contracted nodes (merge 2 nodes in this edge)
        graph.nodes.retain(|n| n.as_str() != edge.n1.as_str() && n.as_str() != edge.n2.as_str());

        // add the merged new nodes in
        let mut new_node = edge.n1.clone();
        new_node.push_str(edge.n2.as_str());
        graph.nodes.push(new_node.clone());

        // remove all the self connect edges
        graph.edges.retain(|e| e != &edge);

        // change left edges' old node to new node
        for e in graph.edges.iter_mut() {
            if e.n1.as_str() == edge.n1 || e.n1.as_str() == edge.n2 {
                e.n1 = new_node.clone();
            }
            if e.n2.as_str() == edge.n1 || e.n2.as_str() == edge.n2 {
                e.n2 = new_node.clone();
            }
        }
    }
    return graph.edges.len();
}

fn random_choose<T>(raw: &mut Vec<T>) -> T {
    let i = thread_rng().gen_range(0..raw.len());
    raw.swap_remove(i)
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            nodes: vec![],
            edges: vec![],
        }
    }
}

impl Edge {
    pub fn new(n1: String, n2: String) -> Edge {
        Edge {
            n1,
            n2,
        }
    }
}