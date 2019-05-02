extern crate rawsock;
use rawsock::{WPCap, Library, InterfaceDescription};

fn main(){
    let pcap = WPCap::open_default_paths().expect("Could not open wpcap library");
    println!("Devices:");
    let devices: Vec<InterfaceDescription> = pcap.get_devices().expect("Could not get devices").collect();
    for dev in devices.iter()  {
        println!("{}, description: {}", dev.name, dev.description)
    }
    //open the last one
    let last = devices.last().expect("No devices found");
    let mut dev = pcap.open_interface(&last.name).expect("Could not open interface");
    println!("Device {} is opened", last.name);
    for _ in 0..10 {
        let packet = dev.receive().expect("Could not receive packet");
        println!("Packet: {:?}", packet);
    }
    {
        let packet1 = dev.receive().unwrap();
        println!("Received packet: {:?}", packet1);
        //let packet2 = dev.receive().unwrap(); This won't compile because of mutable borrow
    }

    let packet: [u8; 10] = [0,1,2,3,4,5,6,7,8,9];
    dev.send(&packet).expect("Could not send packet");
    dev.flush();
    println!("Packet was sent");
}