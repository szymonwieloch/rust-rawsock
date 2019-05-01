use rawsock::{PCap, Library,  InterfaceDescription};

/*
These tests assume:
- You have the relevant library available in the operating system.
- You have at least one network interface.
*/

#[test]
fn open_devices() {
    let pcap = PCap::open_default_locations().expect("Could not open pcap library");
    let devices: Vec<InterfaceDescription> = pcap.get_devices().expect("Could not get devices").collect();
    assert!(! devices.is_empty());
}