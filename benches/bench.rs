use criterion::{criterion_group, criterion_main, Criterion};
use graph_canon::{canon::CanonLabeling, canonize, DenseGraph};
use nauty_pet::prelude::CanonGraph;
use petgraph::{Directed, Graph};
use petgraph_gen::random_gnp_graph;
use rand::SeedableRng;
use rand_chacha::ChaChaRng;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = ChaChaRng::seed_from_u64(0);
    let graph: Graph<(), (), Directed> = random_gnp_graph(&mut rng, 10, 0.3);

    c.bench_function("my_canon", |b| b.iter(|| canonize(&graph)));
    c.bench_function("their_canon", |b| {
        b.iter(|| CanonGraph::from(graph.clone()))
    });
    c.bench_function("to_dense_graph", |b| {
        b.iter(|| DenseGraph::from_petgraph(&graph))
    });
    c.bench_function("to_label", |b| b.iter(|| CanonLabeling::new(&graph)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
