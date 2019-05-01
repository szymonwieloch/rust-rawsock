use rawsock::{PCap, Library,  InterfaceDescription};

/*
These tests assume:
- You have the relevant library available in the operating system.
*/

#[test]
fn open_devices() {
    let pcap = PCap::open_default_locations().expect("Could not open pcap library");
    let devices: Vec<InterfaceDescription> = pcap.get_devices().expect("Could not get devices").collect();
}

#[test]
#[ignore]
fn find_devices(){
    let pcap = PCap::open_default_locations().expect("Could not open pcap library");
    let devices: Vec<InterfaceDescription> = pcap.get_devices().expect("Could not get devices").collect();
    //funny fact: on travis Linux devices do not have any configured devices but MacOS have
    //This is why this assertion cannot be checked by default
    assert!(!devices.is_empty());
}