// Run with `RUSTFLAGS='-Ctarget-cpu=native' cargo bench` to enable all optimizations such as SSE

#[macro_use]
extern crate criterion;

use criterion::{Criterion, Throughput};
use rand::distributions::{Distribution, Uniform};

use violin::{Node, VecN};

const SAMPLES: u64 = 100;
const NODES: u64 = 10_000;

fn setup<const N: usize>(
    nodes: &mut [Node<VecN<N>>],
    peers: &mut [Node<VecN<N>>],
) -> (Vec<f64>, Vec<f64>) {
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

    (rtts, errs)
}

fn do_updates<const N: usize>(
    nodes: &mut [Node<VecN<N>>],
    peers: &mut [Node<VecN<N>>],
    rtts: &[f64],
    errs: &[f64],
) {
    let mut i: usize = 0;
    while i < SAMPLES as usize {
        for (j, (n, p)) in nodes.iter_mut().zip(peers.iter()).enumerate() {
            n.update(rtts[i + j], p.coordinate(), errs[i + j]);
        }
        i += 1;
    }
}

pub fn benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("vivaldi");
    group.throughput(Throughput::Elements(SAMPLES * NODES));
    group.bench_function("heapless 8D", |b| {
        // Create Nodes
        let mut nodes = vec![Node::<VecN<8>>::new(); NODES as usize];
        let mut peers = vec![Node::<VecN<8>>::new(); NODES as usize];
        let (rtts, errs) = setup(&mut nodes, &mut peers);
        b.iter(|| do_updates(&mut nodes, &mut peers, &rtts, &errs))
    });
    group.bench_function("heapless 7D", |b| {
        // Create Nodes
        let mut nodes = vec![Node::<VecN<7>>::new(); NODES as usize];
        let mut peers = vec![Node::<VecN<7>>::new(); NODES as usize];
        let (rtts, errs) = setup(&mut nodes, &mut peers);
        b.iter(|| do_updates(&mut nodes, &mut peers, &rtts, &errs))
    });
    group.bench_function("heapless 6D", |b| {
        // Create Nodes
        let mut nodes = vec![Node::<VecN<7>>::new(); NODES as usize];
        let mut peers = vec![Node::<VecN<7>>::new(); NODES as usize];
        let (rtts, errs) = setup(&mut nodes, &mut peers);
        b.iter(|| do_updates(&mut nodes, &mut peers, &rtts, &errs))
    });
    group.bench_function("heapless 5D", |b| {
        // Create Nodes
        let mut nodes = vec![Node::<VecN<5>>::new(); NODES as usize];
        let mut peers = vec![Node::<VecN<5>>::new(); NODES as usize];
        let (rtts, errs) = setup(&mut nodes, &mut peers);
        b.iter(|| do_updates(&mut nodes, &mut peers, &rtts, &errs))
    });
    group.bench_function("heapless 4D", |b| {
        // Create Nodes
        let mut nodes = vec![Node::<VecN<4>>::new(); NODES as usize];
        let mut peers = vec![Node::<VecN<4>>::new(); NODES as usize];
        let (rtts, errs) = setup(&mut nodes, &mut peers);
        b.iter(|| do_updates(&mut nodes, &mut peers, &rtts, &errs))
    });
    group.bench_function("heapless 3D", |b| {
        // Create Nodes
        let mut nodes = vec![Node::<VecN<3>>::new(); NODES as usize];
        let mut peers = vec![Node::<VecN<3>>::new(); NODES as usize];
        let (rtts, errs) = setup(&mut nodes, &mut peers);
        b.iter(|| do_updates(&mut nodes, &mut peers, &rtts, &errs))
    });
    group.bench_function("heapless 2D", |b| {
        // Create Nodes
        let mut nodes = vec![Node::<VecN<2>>::new(); NODES as usize];
        let mut peers = vec![Node::<VecN<2>>::new(); NODES as usize];
        let (rtts, errs) = setup(&mut nodes, &mut peers);
        b.iter(|| do_updates(&mut nodes, &mut peers, &rtts, &errs))
    });
    group.finish();
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
