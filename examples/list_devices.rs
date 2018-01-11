extern crate rawsock;

use rawsock::{PCap, RawSock};

fn main(){
    let pcap = PCap::open_default_locations().expect("Could not open pcap library");
    for dev in pcap.get_devices().expect("Could not get devices") {
        println!("name: {}, descr: {}", dev.name, dev.description)
    }
}