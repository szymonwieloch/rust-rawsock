extern crate rawsock;
use rawsock::{PCap, RawSock, Interface};
//use core::array::FixedSizeArray;

//This device works on my machine, you need to replace it with whatever is available on yours.
const DEVICE_NAME: &str = "\\Device\\NPF_{EFA814F6-3590-4637-AFBF-F0507CB8567D}";

fn main(){
    let pcap = PCap::open_default_locations().expect("Could not open pcap library");
    let mut dev = pcap.open_interface(DEVICE_NAME).expect("Could not open interface");
    println!("Device {} is opened", DEVICE_NAME);
    for _ in 0..10 {
        let packet = dev.receive().expect("Could not receive packet");
        println!("Packet: {:?}", packet);
    }
    {
        let packet1 = dev.receive().unwrap();
        //let packet2 = dev.receive().unwrap(); This won't compile because of multable borrow
    }

    let packet: [u8; 10] = [0,1,2,3,4,5,6,7,8,9];
    dev.send(&packet).expect("Could not send packet");
    println!("Packet was sent");
}