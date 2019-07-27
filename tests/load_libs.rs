/*
The purpose of these tests is to make sure that it is possible to load the newest versions
of pfring, wpcap and pcap libraries.

This means that the library is found and all symbols can be loaded from the library.

These tests require correct environmental setup to pass.
*/

use rawsock::traits::Library;
#[allow(unused_imports)]
use rawsock::{pcap, wpcap, pfring};

#[test]
#[ignore]
#[cfg(unix)]
fn load_pcap(){
    let _lib = pcap::Library::open_default_paths().expect("Could not load pcap");
}

#[test]
#[ignore]
#[cfg(windows)]
fn load_wpcap(){
    let _lib = wpcap::Library::open_default_paths().expect("Could not load pcap");
}

#[test]
#[ignore]
#[cfg(all(unix, not(any(target_os = "macos", target_os = "ios"))))]
fn load_pfring(){
    let _lib = pfring::Library::open_default_paths().expect("Could not load pcap");
}