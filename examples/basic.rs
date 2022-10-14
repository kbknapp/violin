// This example shows how to create a node and update it's coordinates with an
// arbitrary Round Trip Time (RTT) (usually measured in milliseconds).
//
// Run this example with `cargo run --example basic`
use rand::distributions::Distribution;
use violin::{Node, VecN};

fn main() {
    // Create a node and a peer with an 8-Dimensional coordinate
    // `VecN` is a stack based vector which does not require heap
    // allocation.
    //
    // `BoxVecN` is also available to use heap based vectors.
    //
    // Number of diminsions is based on topology, but powers of two
    // (2, 4, 8, 16, etc.) may allow slightly better performance in
    // some cases.
    let mut node = Node::<VecN<8>>::new();
    let mut peer = Node::<VecN<8>>::new();

    // Generate "random" RTTs (these would normally be determined from real
    // latencies)
    let mut rng = rand::thread_rng();
    let die = rand::distributions::Uniform::from(1.0..5000.0); // synthetic latenies will range
                                                               // from 1 to 5000 ms
    let rtts: Vec<f64> = vec![0.0f64; 2 * 100] // 1000 synthetic latencies for each node
        .iter_mut()
        .map(|_rtt| die.sample(&mut rng))
        .collect();

    for lats in rtts.chunks(2) {
        // Update the node from the peer, and vice a versa
        node.update(lats[0], peer.coordinate(), 25.0);
        peer.update(lats[1], node.coordinate(), 25.0);
    }

    println!("node coordinates: {:?}", node.coordinate());
    println!("peer coordinates: {:?}", peer.coordinate());
    println!("---");
    // In a real application there would probably more than two nodes...but this
    // shows how to get a distance estimate.
    println!("Estimated Distance: {}", node.distance(peer.coordinate()));
}
