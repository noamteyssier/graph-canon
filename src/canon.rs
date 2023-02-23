use crate::dense::DenseGraph;
use bitvec::prelude::*;
use nauty_Traces_sys::{densenauty, empty_graph, optionblk, statsblk};
use petgraph::{EdgeType, Graph};
use std::os::raw::c_int;

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct CanonLabeling {
    g: Vec<u64>,
    e: usize,
    n: usize,
}
impl CanonLabeling {
    pub fn new<N, E, Ty>(graph: &Graph<N, E, Ty>) -> Self
    where
        Ty: EdgeType,
    {
        let mut dg = DenseGraph::from_petgraph(graph);
        let mut opt = canon_opts(graph.is_directed());
        let mut stat = statsblk::default();
        let mut cg = empty_graph(dg.m, dg.n);
        unsafe {
            densenauty(
                dg.g.as_mut_ptr(),
                dg.nodes.lab.as_mut_ptr(),
                dg.nodes.ptn.as_mut_ptr(),
                dg.nodes.orbits.as_mut_ptr(),
                &mut opt,
                &mut stat,
                dg.m as c_int,
                dg.n as c_int,
                cg.as_mut_ptr(),
            )
        }
        Self {
            g: cg,
            e: dg.e,
            n: dg.n,
        }
    }
}

impl<Ty> From<&CanonLabeling> for Graph<(), (), Ty>
where
    Ty: EdgeType,
{
    fn from(cl: &CanonLabeling) -> Self {
        bit_adj_to_graph(&cl.g, cl.e, cl.n)
    }
}

impl<Ty> Into<Graph<(), (), Ty>> for CanonLabeling
where
    Ty: EdgeType,
{
    fn into(self) -> Graph<(), (), Ty> {
        bit_adj_to_graph(&self.g, self.e, self.n)
    }
}

/// Creates a `optionblk` struct for canonization
///
/// # Arguments
/// * `is_directed` - Whether the graph is directed
pub fn canon_opts(is_directed: bool) -> optionblk {
    optionblk {
        getcanon: 1,
        digraph: is_directed.into(),
        ..Default::default()
    }
}

/// Creates an edge list from a bit adjacency matrix
///
/// # Arguments
/// * `adj` - A bit adjacency matrix
/// * `e` - The number of edges
/// * `n` - The number of nodes
pub fn bit_adj_to_edgelist(adj: &[u64], e: usize, n: usize) -> Vec<(u32, u32)> {
    let mut edges = Vec::with_capacity(e);
    for (idx, num) in adj.iter().enumerate() {
        let bv = num.view_bits::<Msb0>();
        for (jdx, bit) in bv.iter().enumerate().take(n) {
            if *bit {
                edges.push((idx as u32, jdx as u32));
            }
        }
    }
    edges
}

/// Creates a graph from a bit adjacency matrix
///
/// # Arguments
/// * `adj` - A bit adjacency matrix
/// * `e` - The number of edges
/// * `n` - The number of nodes
pub fn bit_adj_to_graph<Ty>(adj: &[u64], e: usize, n: usize) -> Graph<(), (), Ty>
where
    Ty: EdgeType,
{
    let edges = bit_adj_to_edgelist(adj, e, n);
    Graph::from_edges(&edges)
}

/// Returns a `Graph` with canonically labeled nodes
pub fn canonize<N, E, Ty>(graph: &Graph<N, E, Ty>) -> Graph<(), (), Ty>
where
    Ty: EdgeType,
{
    let canon = CanonLabeling::new(graph);
    Graph::from(&canon)
}
