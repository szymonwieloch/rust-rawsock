# rust-rawsock
[![Travis CI][tcii]][tci] [![Crates CI][ccii]][cci]

[tcii]: https://travis-ci.org/szymonwieloch/rust-rawsock.svg?branch=master
[tci]: https://travis-ci.org/szymonwieloch/rust-rawsock
[ccii]: https://img.shields.io/crates/v/rawsock.svg
[cci]: https://crates.io/crates/rawsock

# Overview
**rawsock** is a Rust library that highly simplifies use of packet capturing libraries
such as **pcap**, **wpcap** or **pf_ring** and also libraries with a compatible API, such as **npcap**. It can help you to send and receive raw socket frames.
It also provides a consistent API for using these libraries, so that the internal complexity is
hidden.

# Main features

* Support of pcap, wpcap (with Windows-specific optimizations), npcap and pfring
* Libraries are loaded in a dynamic manner, so that the library does not havee any direct
    dependency - it's going to work with whatever is available on the given platform.
* Consistent API for all packet capturing libraries.
* Provided wrapper that automatically chooses an implementation available on your platform.

# Usage:
For now:

Cargo.toml:

```toml
[dependencies]
rawsock = { git = "https://github.com/szymonwieloch/rust-rawsock"}
```

# Documentation
For now:

```bash
cargo doc --open
```

# License
This code is licensed under MIT license.