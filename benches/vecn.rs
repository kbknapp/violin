// Run with `RUSTFLAGS='-Ctarget-cpu=native' cargo bench` to enable all
// optimizations such as SSE

#[macro_use]
extern crate criterion;

use criterion::{Criterion, Throughput};
use rand::distributions::{Distribution, Uniform};

use violin::VecN;

const SAMPLES: u64 = 10_000;
const NODES: u64 = 10_000_000;

fn setup<const N: usize>(nodes: &mut Vec<VecN<N>>, peers: &mut Vec<VecN<N>>) {
    // Pre-compute "random" rtts
    let mut rng = rand::thread_rng();
    let die = Uniform::from(-5.0..5.0);
    // Pre-move the peers at least once so they're not all clustered around the
    // origin
    for _ in 0..nodes.len() {
        let mut n = VecN::<N>::new();
        for nn in n.iter_mut() {
            *nn = die.sample(&mut rng);
        }
        nodes.push(n);
        let mut p = VecN::<N>::new();
        for pp in p.iter_mut() {
            *pp = die.sample(&mut rng);
        }
        peers.push(p);
    }
}

fn do_math<const N: usize>(nodes: &[VecN<N>], peers: &[VecN<N>]) -> VecN<N> {
    let mut i: usize = 0;
    let mut sum = VecN::<N>::new();
    while i < SAMPLES as usize {
        for (n, p) in nodes.iter().zip(peers.iter()) {
            sum = n + p
        }
        i += 1;
    }
    sum
}

pub fn benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("vector add");
    group.throughput(Throughput::Elements(SAMPLES * NODES));
    group.bench_function("8D", |b| {
        // Create Nodes
        let mut nodes = Vec::with_capacity(NODES as usize);
        let mut peers = Vec::with_capacity(NODES as usize);
        setup::<8>(&mut nodes, &mut peers);
        b.iter(|| do_math(&mut nodes, &mut peers))
    });
    group.bench_function("7D", |b| {
        // Create Nodes
        let mut nodes = Vec::with_capacity(NODES as usize);
        let mut peers = Vec::with_capacity(NODES as usize);
        setup::<7>(&mut nodes, &mut peers);
        b.iter(|| do_math(&mut nodes, &mut peers))
    });
    group.bench_function("6D", |b| {
        // Create Nodes
        let mut nodes = Vec::with_capacity(NODES as usize);
        let mut peers = Vec::with_capacity(NODES as usize);
        setup::<6>(&mut nodes, &mut peers);
        b.iter(|| do_math(&mut nodes, &mut peers))
    });
    group.bench_function("5D", |b| {
        // Create Nodes
        let mut nodes = Vec::with_capacity(NODES as usize);
        let mut peers = Vec::with_capacity(NODES as usize);
        setup::<5>(&mut nodes, &mut peers);
        b.iter(|| do_math(&mut nodes, &mut peers))
    });
    group.bench_function("4D", |b| {
        // Create Nodes
        let mut nodes = Vec::with_capacity(NODES as usize);
        let mut peers = Vec::with_capacity(NODES as usize);
        setup::<4>(&mut nodes, &mut peers);
        b.iter(|| do_math(&mut nodes, &mut peers))
    });
    group.bench_function("3D", |b| {
        // Create Nodes
        let mut nodes = Vec::with_capacity(NODES as usize);
        let mut peers = Vec::with_capacity(NODES as usize);
        setup::<3>(&mut nodes, &mut peers);
        b.iter(|| do_math(&mut nodes, &mut peers))
    });
    group.bench_function("2D", |b| {
        // Create Nodes
        let mut nodes = Vec::with_capacity(NODES as usize);
        let mut peers = Vec::with_capacity(NODES as usize);
        setup::<2>(&mut nodes, &mut peers);
        b.iter(|| do_math(&mut nodes, &mut peers))
    });
    group.finish();
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
