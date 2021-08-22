// Run with `RUSTFLAGS='-Ctarget-cpu=native' cargo bench` to enable all optimizations such as SSE

#[macro_use]
extern crate criterion;

use criterion::{Criterion, Throughput};
use rand::distributions::{Distribution, Uniform};

use violin::{Node, VecN};

const SAMPLES: u64 = 100;
const NODES: u64 = 10_000;

pub fn baseline(buf: &[u8]) -> usize {
    buf.iter().count()
}

pub fn benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("vivaldi");
    group.throughput(Throughput::Elements(SAMPLES * NODES));
    group.bench_function("updates", |b| {
        // Create Nodes
        let mut nodes = vec![Node::<VecN<8>>::new(); NODES as usize];
        let mut peers = vec![Node::<VecN<8>>::new(); NODES as usize];
        // Pre-compute "random" rtts
        let mut rng = rand::thread_rng();
        let die = Uniform::from(1.0..5000.0);
        let rtts: Vec<f64> = vec![0.0f64; (NODES * SAMPLES) as usize]
            .iter_mut()
            .map(|_rtt| die.sample(&mut rng))
            .collect();
        let errs: Vec<f64> = vec![0.0f64; (NODES * SAMPLES) as usize]
            .iter_mut()
            .map(|_rtt| die.sample(&mut rng))
            .collect();
        // Pre-move the peers at least once so they're not all clustered around the origin
        for (i, n) in rtts.iter().enumerate() {
            if let Some(peer) = peers.get_mut(i % NODES as usize) {
                if let Some(node) = nodes.get(i + 1) {
                    peer.update(*n, &node.coordinate(), errs[i]);
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        b.iter(|| {
            let mut i: usize = 0;
            while i < SAMPLES as usize {
                for (j, (n, p)) in nodes.iter_mut().zip(peers.iter()).enumerate() {
                    n.update(rtts[i + j], p.coordinate(), errs[i + j]);
                }
                i += 1;
            }
        })
    });
    group.finish();
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
