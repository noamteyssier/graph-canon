# graph-canon

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE.md)
![actions status](https://github.com/noamteyssier/graph-canon/workflows/CI/badge.svg)
[![codecov](https://codecov.io/gh/noamteyssier/graph-canon/branch/main/graph/badge.svg?token=RVHY8O3873)](https://codecov.io/gh/noamteyssier/graph-canon)
[![docs.rs](https://img.shields.io/docsrs/graph-canon?color=green&label=docs.rs)](https://docs.rs/graph-canon/latest/graph-canon/)

Super fast and barebones graph canonicalization using `nauty` C-lib
and built on `petgraph`.

## Usage

### Hashable Labels

If you are just looking to create a hashable object to determine isomorphism
then it is simples to use the `CanonLabeling` struct.

This can be created from a `Graph` object directly.

#### Directed Graphs

```rust
use petgraph::{Directed, Graph};
use graph_canon::CanonLabeling;

let e1 = vec![(0, 1), (0, 2), (1, 2)]; // Isomorphic
let e2 = vec![(1, 0), (1, 2), (0, 2)]; // Isomorphic
let e3 = vec![(1, 0), (1, 2), (2, 1)]; // Non-Isomorphic

let g1 = Graph::<(), (), Directed>::from_edges(&e1);
let g2 = Graph::<(), (), Directed>::from_edges(&e2);
let g3 = Graph::<(), (), Directed>::from_edges(&e3);

let l1 = CanonLabeling::new(&g1);
let l2 = CanonLabeling::new(&g2);
let l3 = CanonLabeling::new(&g3);

assert_eq!(l1, l2);
assert_ne!(l1, l3);
```

#### Undirected Graphs

```rust
use petgraph::{Undirected, Graph};
use graph_canon::CanonLabeling;

let e1 = vec![(0, 1), (0, 2), (1, 2)]; // Isomorphic
let e2 = vec![(1, 0), (1, 2), (0, 2)]; // Isomorphic
let e3 = vec![(1, 0), (1, 2)];         // Non-Isomorphic

let g1 = Graph::<(), (), Undirected>::from_edges(&e1);
let g2 = Graph::<(), (), Undirected>::from_edges(&e2);
let g3 = Graph::<(), (), Undirected>::from_edges(&e3);

let l1 = CanonLabeling::new(&g1);
let l2 = CanonLabeling::new(&g2);
let l3 = CanonLabeling::new(&g3);

assert_eq!(l1, l2);
assert_ne!(l1, l3);
```

### Recovering the Canonical `Graph`

If instead you are interested in working with the graph itself,
you can use the `canonize` function to return a new `Graph` object

```rust
use petgraph::{Directed, Graph};
use graph_canon::canonize;

let edges = vec![(0, 1), (0, 2), (1, 2)];
let graph = Graph::<(), (), Directed>::from_edges(&edges);
let canon = canonize(&graph);
assert_eq!(canon.edge_count(), 3);
```

## Timing Comparison

This crate is inspired by [`nauty-pet`](https://crates.io/crates/nauty-pet)
but is much faster as it is much simpler.
(tests measured with [`criterion`](https://docs.rs/criterion/latest/criterion/))

This test is using a randomly generated graph of `10` nodes and `0.5` probability
of edge connection using [`random_gpn_graph`](https://docs.rs/petgraph-gen/0.1.3/petgraph_gen/fn.random_gnp_graph.html)

```text
graph-canon             time:   [1.3272 µs 1.3276 µs 1.3285 µs]
Found 14 outliers among 100 measurements (14.00%)
  3 (3.00%) high mild
  11 (11.00%) high severe

nauty-pet               time:   [6.2591 µs 6.2738 µs 6.2956 µs]
Found 10 outliers among 100 measurements (10.00%)
  2 (2.00%) low mild
  4 (4.00%) high mild
  4 (4.00%) high severe
```
