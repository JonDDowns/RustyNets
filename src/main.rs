//use petgraph::dot::{Config, Dot};
use petgraph::graph::UnGraph;
use rand::distributions::Distribution;
use std::env;

fn main() {
    // Pull number of desired nodes from user input
    let args: Vec<String> = env::args().collect();
    let num_nodes = &args[1];
    let num_nodes: u32 = num_nodes.trim().parse().expect("Input not an integer");

    // Create graph, add nodes
    let mut graph = UnGraph::<_, ()>::new_undirected();
    for i in 0..num_nodes {
        graph.add_node(i);
    }

    // For now, make number of edges same as number of nodes
    let mut nedge = num_nodes;

    // Set up a RNG to assign edges randomly
    let mut rng = &mut rand::thread_rng();
    let dist = rand::distributions::Uniform::from(0..num_nodes);

    // Loop to create edges
    while nedge > 0 {
        // Sample two nodes randomly by integer value
        let a: u32 = dist.sample(&mut rng);
        let b: u32 = dist.sample(&mut rng);

        // Check that edge is unique/valid
        if graph.contains_edge(a.into(), b.into()) || a == b {
            continue;
        }

        // Add edge
        graph.add_edge(a.into(), b.into(), ());
        nedge -= 1;
    }

    let mut maxedge = 0;
    for node in graph.node_indices() {
        let nedge = graph.neighbors(node).count();
        if nedge > maxedge {
            maxedge = nedge;
        }
    }
    println!("{:?}", maxedge);

    //println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
}
