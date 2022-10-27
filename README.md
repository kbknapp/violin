# `violin`

![Rust Version][rustc-image]
[![crates.io][crate-image]][crate-link]
[![Documentation][docs-image]][docs-link]
[![Dependency Status][deps-image]][deps-link]

A Rust `no_std` no `alloc` implementation of the [Vivaldi algorithm][1](PDF)
for a network coordinate system.

A network coordinate system allows nodes to accurately estimate network
latencies by merely exchanging coordinates.


<!-- vim-markdown-toc GFM -->

* [Violin - The Pitch](#violin---the-pitch)
* [Violin - The Anit-Pitch](#violin---the-anit-pitch)
* [Compile from Source](#compile-from-source)
* [Usage](#usage)
* [Benchmarks](#benchmarks)
* [License](#license)
    * [Contribution](#contribution)
* [Related Papers and Research](#related-papers-and-research)

<!-- vim-markdown-toc -->

## Violin - The Pitch

Violin is an implementation of Vivaldi network coordinates that works in
`no_std` and no `alloc` environments. Each coordinate is small consisting of a
dimensional vector made up of an array of `f64`s. The arrays use const
generics, so they can be as small as a single f64 or large as one needs.
Although above a certain dimension there are diminishing returns.

Nodes can measure real latencies between an origin node, or each-other to
adjust their coordinates in space.

The real power comes from being able to calculate distance between a remote
coordinate without ever having done a real latency check. For example node `A`
measures against node `Origin`, node `B` does the same. Then `A` can be given
the coordinates to `B` and accurately estimate the latency without ever having
measured `B` directly.

## Violin - The Anit-Pitch

Vivaldi isn't a magic bullet and still requires measuring real latencies to
adjust the coordinates. In a naive implementation, conducting a latency check
prior to a coordinate calculation is not much better than just using the
latency check directly as the answer. However, this is not how it's supposed to
be used.

Transferring a Violin coordinate in practice can be comparable data to a small
set of ICMP messages. For example an 8-Dimension coordinate (plus three 
additional `f64`s of metadata) is 88 bytes. However, unlike ICMP messages, the
Violin coordinates are a single transmission and only need to be re-transmitted
on significant change. Work could even be done to only transmit deltas as well.

## Compile from Source

Ensure you have a [Rust toolchain installed][rustup].

```
$ git clone https://github.com/kbknapp/violin
$ cd violin
$ RUSTFLAGS='-Ctarget-cpu=native' cargo build --release
```

**NOTE:** The `RUSTFLAGS` can be omitted. However, if on a recent CPU that
supports SIMD instructions, and the code will be run on the same CPU it's
compiled for, including this flag can improve performance.

## Usage

See the `examples/` directory in this repository for complete details, although
at quick glance creating three coordinates (`origin`, `a` and `b`) and updating
`a` and `b`'s coordinate from experienced real latency would look like this:

```rust
use std::time::Duration;
use violin::{heapless::VecD, Coord, Node};

// Create two nodes and an "origin" coordinate, all using a 4-Dimensional
// coordinate. `VecD` is a dimensional vector.
let origin = Coord::<VecD<4>>::rand();
let mut a = Node::<VecD<4>>::rand();
let mut b = Node::<VecD<4>>::rand();

// **conduct some latency measurement from a to origin**
// let's assume we observed a value of `0.2` seconds...
//
// **conduct some latency measurement from b to origin**
// let's assume we observed a value of `0.03` seconds...

a.update(Duration::from_secs_f64(0.2), &origin);
b.update(Duration::from_secs_f64(0.03), &origin);

// Estimate from a to b even though we never measured them directly
println!("a's estimate to b: {:.2}ms", a.distance_to(&b.coordinate()).as_millis());
```

## Benchmarks

A set of benchmarks are included using 8D, 4D, and 2D coordinates both using
`heap::VecD` (requires the `alloc` feature) and `heapless::VecD`. 

The benchmarks measure both the higher level `Node` as well as a lower level
`Coord` abstractions.

To measure we create 10,000 coordinates and the coordinates are
update for each coordinate 100 times, totaling 1,000,000 updates.

On my 8 core AMD Ryzen 7 5850U laptop with 16GB RAM the benchmarks look as
follows:

| Abstraction | Memory   | Dimensions | Time |
| :-: | :-:      | :-:        | :-:  |
| `Node` | heap     | 8          | 66.537 ms |
| `Coord` | heap     | 8          | 55.402 ms |
| `Node` | heapless | 8          | 24.997 ms |
| `Coord` | heapless | 8          | 16.552 ms |
| `Node` | heap     | 4          | 49.501 ms |
| `Coord` | heap     | 4          | 39.163 ms |
| `Node` | heapless | 4          | 16.795 ms |
| `Coord` | heapless | 4          | 11.780 ms |
| `Node` | heap     | 2          | 54.363 ms |
| `Coord` | heap     | 2          | 46.001 ms |
| `Node` | heapless | 2          | 13.181 ms |
| `Coord` | heapless | 2          | 10.916 ms |

To run the benchmarks yourself use `RUSTFLAGS='-Ctarget-cpu=native' cargo bench`.

## License

This crate is licensed under either of

 * [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
 * [MIT license](http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly Node otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Related Papers and Research

- [Vivaldi - A Decentralized Network Coordinate System][1](PDF)
- [Network Coordinates in the Wild][2](PDF)
- [Towards Network Triangle Inequality Violation Aware Distributed Systems][3](PDF)
- [On Suitability of Euclidean Embedding for Host-based Network Coordinate Systems][4](PDF)
- [Practical, Distributed Network Coordinates][5](PDF)
- [Armon Dadgar on Vivaldi: Decentralized Network Coordinate System][6](Video)

[//]: # (badges)

[rustc-image]: https://img.shields.io/badge/rustc-1.59+-blue.svg
[crate-image]: https://img.shields.io/crates/v/violin.svg
[crate-link]: https://crates.io/crates/violin
[docs-image]: https://docs.rs/violin/badge.svg
[docs-link]: https://docs.rs/violin
[deps-image]: https://deps.rs/repo/github/kbknapp/violin/status.svg
[deps-link]: https://deps.rs/repo/github/kbknapp/violin

[//]: # (links)

[rustup]: https://rustup.rs
[1]: https://pdos.csail.mit.edu/papers/vivaldi:sigcomm/paper.pdf
[2]: https://www.usenix.org/legacy/event/nsdi07/tech/full_papers/ledlie/ledlie.pdf
[3]: https://www.cs.rice.edu/~eugeneng/papers/IMC07.pdf
[4]: https://www-users.cse.umn.edu/~zhang089/Papers/Lee-Suitability-tonfinal.pdf
[5]: http://www.news.cs.nyu.edu/~jinyang/pub/hotnets03.pdf
[6]: https://youtu.be/AszPoJjWK9Q?t=1690
