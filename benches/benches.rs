#[macro_use]
extern crate criterion;

use criterion::{black_box, BenchmarkId, Criterion, Throughput};
use rand::{
    distributions::{Distribution, Uniform},
    thread_rng,
};

use violin::State;

const SAMPLES: u64 = 1_000;
const NODES: u64 = 10_000;

pub fn baseline(buf: &[u8]) -> usize {
    buf.iter().count()
}

pub fn aes_gcm(key: &[u8], nonce: &[u8], buf: &[u8]) {
    todo!()
}

pub fn benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("vivaldi");
    group.throughput(Throughput::Elements(SAMPLES * NODES));
    group.bench_function("updates", |b| {
        // Create Nodes
        let mut nodes = vec![State::<8>::new(); NODES as usize];
        let mut peers = vec![State::<8>::new(); NODES as usize];
        // Pre-compute "random" rtts
        let mut rng = rand::thread_rng();
        let die = Uniform::from(1.0..5000.0);
        let rtts: Vec<f64> = vec![0f64; (NODES * SAMPLES) as usize]
            .iter_mut()
            .map(|_rtt| die.sample(&mut rng))
            .collect();
        let errs: Vec<f64> = vec![0f64; (NODES * SAMPLES) as usize]
            .iter_mut()
            .map(|_rtt| die.sample(&mut rng))
            .collect();
        // Pre-move the peers at least once so they're not all clustered around the origin
        for (i, n) in rtts.iter().enumerate() {
            let peer = peers.get_mut(i % NODES as usize).unwrap();
            let node = nodes.get(i + 1).unwrap();
            peer.update(*n, node.point().clone(), errs[i]);
        }
        b.iter(|| {
            for (i, n) in rtts.iter().enumerate() {
                let node = nodes.get_mut(i % NODES as usize).unwrap();
                let peer = peers.get(i + 1).unwrap();
                node.update(*n, peer.point().clone(), errs[i]);
            }
        })
    });
    group.finish();
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
