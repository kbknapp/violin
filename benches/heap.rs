// Run with `RUSTFLAGS='-Ctarget-cpu=native' cargo bench` to enable all
// optimizations such as SSE

#[macro_use]
extern crate criterion;

use std::time::Duration;

use criterion::{Criterion, Throughput};
use rand::distributions::{Distribution, Uniform};

use violin::{Config, Coord, Node, VecD, Vector};

const SAMPLES: u64 = 100;
const NODES: u64 = 10_000;

// Pre-compute "random" rtts
fn gen_duration_rtts() -> Vec<Duration> {
    gen_rtts()
        .iter()
        .map(|&rtt| Duration::from_secs_f64(rtt))
        .collect()
}

fn gen_rtts() -> Vec<f64> {
    let mut rng = rand::thread_rng();
    let die = Uniform::from(1.0e-5..5.0);
    let rtts: Vec<f64> = vec![0.0f64; (NODES * SAMPLES) as usize]
        .iter_mut()
        .map(|_rtt| die.sample(&mut rng))
        .collect();

    rtts
}

fn do_node_updates<T: Vector + Clone>(
    nodes: &mut [Node<T>],
    peers: &mut [Node<T>],
    rtts: &[Duration],
) {
    let mut i: usize = 0;
    while i < SAMPLES as usize {
        for (j, (n, p)) in nodes.iter_mut().zip(peers.iter()).enumerate() {
            n.update(rtts[i + j], p.coordinate());
        }
        i += 1;
    }
}

fn do_coord_updates<T: Vector + Clone>(
    nodes: &mut [Coord<T>],
    peers: &mut [Coord<T>],
    cfg: &Config,
    rtts: &[f64],
) {
    let mut i: usize = 0;
    while i < SAMPLES as usize {
        for (j, (n, p)) in nodes.iter_mut().zip(peers.iter()).enumerate() {
            n.update(rtts[i + j], p, cfg);
        }
        i += 1;
    }
}

pub fn benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("violin::Node");
    group.throughput(Throughput::Elements(SAMPLES * NODES));
    group.bench_function("heap 8D (0 adjustment window)", |b| {
        // Create Coords
        let mut nodes = vec![Node::<VecD<8>>::rand(); NODES as usize];
        let mut peers = vec![Node::<VecD<8>>::rand(); NODES as usize];
        let rtts = gen_duration_rtts();
        b.iter(|| do_node_updates(&mut nodes, &mut peers, &*rtts))
    });
    group.bench_function("heap 4D (0 adjustment window)", |b| {
        // Create Coords
        let mut nodes = vec![Node::<VecD<4>>::rand(); NODES as usize];
        let mut peers = vec![Node::<VecD<4>>::rand(); NODES as usize];
        let rtts = gen_duration_rtts();
        b.iter(|| do_node_updates(&mut nodes, &mut peers, &*rtts))
    });
    group.bench_function("heap 2D (0 adjustment window)", |b| {
        // Create Coords
        let mut nodes = vec![Node::<VecD<2>>::rand(); NODES as usize];
        let mut peers = vec![Node::<VecD<2>>::rand(); NODES as usize];
        let rtts = gen_duration_rtts();
        b.iter(|| do_node_updates(&mut nodes, &mut peers, &*rtts))
    });
    group.finish();

    let mut group = c.benchmark_group("violin::Coord");
    group.throughput(Throughput::Elements(SAMPLES * NODES));
    group.bench_function("heap 8D", |b| {
        // Create Coords
        let mut nodes = vec![Coord::<VecD<8>>::rand(); NODES as usize];
        let mut peers = vec![Coord::<VecD<8>>::rand(); NODES as usize];
        let rtts = gen_rtts();
        let cfg = Config::default();
        b.iter(|| do_coord_updates(&mut nodes, &mut peers, &cfg, &*rtts))
    });
    group.bench_function("heap 4D", |b| {
        // Create Coords
        let mut nodes = vec![Coord::<VecD<4>>::rand(); NODES as usize];
        let mut peers = vec![Coord::<VecD<4>>::rand(); NODES as usize];
        let rtts = gen_rtts();
        let cfg = Config::default();
        b.iter(|| do_coord_updates(&mut nodes, &mut peers, &cfg, &*rtts))
    });
    group.bench_function("heap 2D", |b| {
        // Create Coords
        let mut nodes = vec![Coord::<VecD<2>>::rand(); NODES as usize];
        let mut peers = vec![Coord::<VecD<2>>::rand(); NODES as usize];
        let rtts = gen_rtts();
        let cfg = Config::default();
        b.iter(|| do_coord_updates(&mut nodes, &mut peers, &cfg, &*rtts))
    });
    group.finish();
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
