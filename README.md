# `violin`

![Rust Version][rustc-image]
[![crates.io][crate-image]][crate-link]
[![Documentation][docs-image]][docs-link]
[![Dependency Status][deps-image]][deps-link]

An implementation of the [Vivaldi algorithm][1](PDF) for a decentralized network coordinate system.

<!-- markdown-toc start - Don't edit this section. Run M-x markdown-toc-refresh-toc -->
**Table of Contents**

- [`violin`](#violin)
    - [The Pitch](#the-pitch)
    - [The Anit-Pitch](#the-anit-pitch)
    - [Compile from Source](#compile-from-source)
- [Usage](#usage)
- [License](#license)
    - [Contribution](#contribution)
- [Related Papers and Research](#related-papers-and-research)

<!-- markdown-toc end -->

## The Pitch

@TODO: pitch

## The Anit-Pitch

@TODO: anti-pitch

## Compile from Source

Ensure you have a [Rust toolchain installed][0].

```
$ git clone https://github.com/kbknapp/violin
$ cd violin
$ cargo build --release
```

# Usage

@TODO: usage

# License

This crate is licensed under either of

 * [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
 * [MIT license](http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

# Related Papers and Research

- [Vivaldi - A Decentralized Network Coordinate System][1](PDF)
- [Network Coordinates in the Wild][2](PDF)
- [Towards Network Triangle Inequality Violation Aware Distributed Systems][3](PDF)
- [On Suitability of Euclidean Embedding for Host-based Network Coordinate Systems][4](PDF)
- [Practical, Distributed Network Coordinates][5](PDF)
- [Armon Dadgar on Vivaldi: Decentralized Network Coordinate System][6](Video)

[//]: # (badges)

[rustc-image]: https://img.shields.io/badge/rustc-1.53+-blue.svg
[crate-image]: https://img.shields.io/crates/v/violin.svg
[crate-link]: https://crates.io/crates/violin
[docs-image]: https://docs.rs/violin/badge.svg
[docs-link]: https://docs.rs/violin
[deps-image]: https://deps.rs/repo/github/kbknapp/violin/status.svg
[deps-link]: https://deps.rs/repo/github/kbknapp/violin

[//]: # (links)

[0]: https://rustup.rs
[1]: https://pdos.csail.mit.edu/papers/vivaldi:sigcomm/paper.pdf
[2]: https://www.usenix.org/legacy/event/nsdi07/tech/full_papers/ledlie/ledlie.pdf
[3]: https://www.cs.rice.edu/~eugeneng/papers/IMC07.pdf
[4]: https://www-users.cse.umn.edu/~zhang089/Papers/Lee-Suitability-tonfinal.pdf
[5]: http://www.news.cs.nyu.edu/~jinyang/pub/hotnets03.pdf
[6]: https://youtu.be/AszPoJjWK9Q?t=1690
