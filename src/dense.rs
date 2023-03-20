use nauty_Traces_sys::{empty_graph, ADDONEARC, SETWORDSNEEDED};
use petgraph::{visit::GetAdjacencyMatrix, EdgeType, Graph};
use std::ffi::c_int;

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct DenseGraph {
    pub g: Vec<u64>,
    pub n: usize,
    pub e: usize,
    pub m: usize,
    pub nodes: Nodes,
}
impl DenseGraph {
    pub fn from_petgraph<N, E, Ty>(graph: &Graph<N, E, Ty>) -> Self
    where
        Ty: EdgeType,
    {
        let n = graph.node_count();
        let e = graph.edge_count();
        let m = SETWORDSNEEDED(n);
        let nodes = Nodes::new(n);
        let mut g = empty_graph(m, n);
        let adj = graph.adjacency_matrix();
        for idx in 0..n {
            for jdx in 0..n {
                if adj.contains(idx * n + jdx) {
                    ADDONEARC(&mut g, idx, jdx, m)
                }
            }
        }
        Self { g, n, e, m, nodes }
    }

    pub fn orbits(&self) -> &[i32] {
        &self.nodes.orbits
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Nodes {
    pub lab: Vec<c_int>,
    pub ptn: Vec<c_int>,
    pub orbits: Vec<c_int>,
}
impl Nodes {
    pub fn new(n: usize) -> Self {
        Self {
            lab: (0..n as i32).collect(),
            ptn: vec![0; n],
            orbits: vec![0; n],
        }
    }
}
