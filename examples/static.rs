/**
    Some application may require direct use of a concrete library.
    In tis case you should declare type of your library directly.
*/

extern crate rawsock;
use rawsock::traits::{DynamicInterface, Library};
use rawsock::{wpcap, pcap, pfring};
mod commons;
use self::commons::ICMP_PACKET;

pub fn open_library<T>() -> T where T: Library {
    let lib = T::open_default_paths().expect("Could not open library");
    println!("Library opened, version is {}", lib.version());
    lib
}

pub fn receive_packets<'a, T>(interf: &mut T) where T: DynamicInterface<'a>{
    println!("Receiving 5 packets:");
    for i in 1..6{
        let packet = interf.receive().expect("Error while receiving packet");
        println!("Packet {} is {}", i, packet);
    }
}

pub fn send_packets<'a, T>(interf: &mut T) where T: DynamicInterface<'a> {
    println!("Data link: {}", interf.data_link());
    println!("Sending 5 ICMP ping packets:");
    for i in 1..6 {
        interf.send(&ICMP_PACKET).expect("Errow while sending packet");
        println!("Packet {} was sent", i);
    }
}

fn main () {
    run_pcap();
    run_pfring();
    run_wpcap();
}

fn run_pcap(){
    let lib = open_library::<pcap::Library>();
    let ifname = lib.all_interfaces()
        .expect("Could not obtain interface list").first()
        .expect("There are no available interfaces").name.clone();
    let mut interf = lib.open_interface(&ifname).expect("Could not open pcap interface");
    send_packets(&mut interf);
    receive_packets(&mut interf);
}

fn run_wpcap() {
    let lib = open_library::<wpcap::Library>();
    let ifname = lib.all_interfaces()
        .expect("Could not obtain interface list").first()
        .expect("There are no available interfaces").name.clone();
    let mut interf = lib.open_interface(&ifname).expect("Could not open wpcap interface");
    send_packets(&mut interf);
    receive_packets(&mut interf);
}

fn run_pfring() {
    let lib = open_library::<pfring::Library>();
    let ifname = lib.all_interfaces()
        .expect("Could not obtain interface list").first()
        .expect("There are no available interfaces").name.clone();
    let mut interf = lib.open_interface(&ifname).expect("Could not open pfring interface");
    send_packets(&mut interf);
    receive_packets(&mut interf);
}