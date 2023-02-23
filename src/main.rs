use graph_canon::{canon::CanonLabeling, canonize};
use petgraph::{Directed, Graph};

fn main() {
    let edges = vec![(0, 1), (0, 2), (1, 2)];
    let alternate_edges = vec![(1, 0), (1, 2), (0, 2)];
    let g1 = Graph::<(), (), Directed>::from_edges(&edges);
    let g2 = Graph::<(), (), Directed>::from_edges(&alternate_edges);
    let l1 = CanonLabeling::new(&g1);
    let l2 = CanonLabeling::new(&g2);
    assert_eq!(l1, l2);
    let cg1 = canonize(&g1);
    println!("graph: {:?}", g1);
    println!("canon: {:?}", cg1);
}
