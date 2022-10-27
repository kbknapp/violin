// This example shows how to create a node and update it's coordinates with an
// arbitrary Round Trip Time (RTT) (usually measured in milliseconds).
//
// Run this example with `cargo run --example basic`
use std::time::Duration;

use rand::{distributions::Distribution, Rng};

use violin::{heapless::VecD, Coord, Node};

fn main() {
    // Create two nodes and an "origin" coordinate, all using an 8-Dimensional
    // coordinate. `VecD` is a dimensional vector.
    //
    // Number of diminsions is based on topology, but powers of two (2, 4, 8, 16,
    // etc.) may allow better performance in some cases.
    let origin = Coord::<VecD<8>>::default();
    let mut a = Node::<VecD<8>>::default();
    let mut b = Node::<VecD<8>>::default();

    let (a_real, a_rtts) = gen_synthetic_latencies();
    let (b_real, b_rtts) = gen_synthetic_latencies();

    // Both nodes will be "measuring latency" (our synthetic latencies) between
    // themselves and a "base point" called origin
    //
    // Then, without having ever measured latency between one another, they can
    // simply pass their coordinates and determine within a decently accurate
    // guess what the latency between 'a' and 'b' is.
    //
    // In a real world application there isn't really a need for an origin,
    // although having one does give something for nodes to measure against.
    // Nodes only need measure against _some other node_ to find where in space
    // they are. The more measurements the more accurate.
    //
    // This approach is useful because each node only need maintain it's own
    // coordinates, and give any other nodes coordinates can calculate a latency
    // to that node. Whereas the alternative is every node maintaining latency
    // maps for _every other_ node which would 1) take a lot of storage and 2)
    // require TONS of network traffic to perform all the N:N latency checks.

    for (a_rtt, b_rtt) in a_rtts.iter().zip(b_rtts.iter()) {
        // Update the node from the peer, and vice a versa
        a.update(*a_rtt, &origin);
        b.update(*b_rtt, &origin);
    }

    println!(
        "a's estimate to base: {:.2}ms (actual: {:.2}ms)",
        a.distance_to(&origin).as_millis(),
        a_real.as_millis()
    );
    println!(
        "b's estimate to base: {:.2}ms (actual: {:.2}ms)",
        b.distance_to(&origin).as_millis(),
        b_real.as_millis()
    );
    println!(
        "a's estimate to b: {:.2}ms",
        a.distance_to(b.coordinate()).as_millis()
    );
}

// We need round-trip-time (RTT) latencies between two nodes. In the real
// world the latency between the nodes will be jittery, but should be close to
// static. To simulate this we pick a random latency to be our "real"
// latency then generate a bunch of simulated latencies within a small range of
// that real latency
//
// We return the "real" latency generated so we can compare the final estimate
fn gen_synthetic_latencies() -> (Duration, Vec<Duration>) {
    let mut rng = rand::thread_rng();

    // Generate the "real" lantency
    let base: f64 = rng.gen_range(1.0e-4..1.0e-1);

    // Create the random die
    let die = rand::distributions::Uniform::from((base - 0.01).abs()..base + 0.01);

    // 1000 synthetic latencies
    (
        Duration::from_secs_f64(base),
        vec![0.0f64; 1000]
            .iter_mut()
            .map(|_rtt| Duration::from_secs_f64(die.sample(&mut rng).abs()))
            .collect(),
    )
}
