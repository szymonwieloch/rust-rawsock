extern crate rawsock;
use rawsock::{PFRing, Library, Interface};

fn main(){
    let pfring = PFRing::open_default_paths().expect("Could not open pcap library");
    println!("Devices:");
    //PF Ring does not support obtaining a list of devices - at least for now
    //so you just need to know a device path specific to your platform
    const DEV_NAME: &str = "/dev/eth0";
    let mut dev = pfring.open_interface(DEV_NAME).expect("Could not open interface");
    println!("Device {} is opened", DEV_NAME);
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