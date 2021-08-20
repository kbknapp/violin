use rand::distributions::Distribution;
use violin::{Node, VecN};

fn main() {
    // Create a node with an 8-Dimensional coordinate
    let mut node = Node::<VecN<8>>::new();
    let mut peer = Node::<VecN<8>>::new();

    // Generate "random" RTTs (these would normally be determined from real latencies)
    let mut rng = rand::thread_rng();
    let die = rand::distributions::Uniform::from(1.0..5000.0); // synthetic latenies will range from 1 to 5000 ms
    let rtts: Vec<f64> = vec![0.0f64; 2 * 1000] // 1000 synthetic latencies for each node
        .iter_mut()
        .map(|_rtt| die.sample(&mut rng))
        .collect();

    for lats in rtts.chunks(2) {
        node.update(lats[0], peer.coordinate(), 25.0);
        peer.update(lats[1], node.coordinate(), 25.0);
    }

    println!("node coordinates: {:?}", node.coordinate());
    println!("peer coordinates: {:?}", peer.coordinate());
    println!("---");
    println!("Estimated Distance: {}", node.distance(peer.coordinate()));
}
