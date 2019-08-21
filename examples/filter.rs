/*!
    The usage of filter.
*/

extern crate rawsock;
mod commons;
use rawsock::{open_best_library, DataLink};

fn main() {
    println!("Opening packet capturing library");
    let lib = open_best_library().expect("Could not open any packet capturing library");
    println!("Library opened, version is {}", lib.version());
    let interf_name = lib.all_interfaces()
        .expect("Could not obtain interface list").first()
        .expect("There are no available interfaces").name.clone();
    println!("Opening the {} interface", &interf_name);
    let mut interf = lib.open_interface(&interf_name).expect("Could not open network interface");
    if let DataLink::Ethernet = interf.data_link() {
        println!("Interface opened, data link: {}", interf.data_link());
    } else {
        println!("data link should be Ethernet, {} found", interf.data_link());
        return
    }

    interf.set_filter("icmp").expect("Could not set filter");

    //receive some packets.
    println!("Receiving 5 packets:");
    for _ in 0..5 {
        let packet = interf.receive().expect("Could not receive packet");
        println!("Received packet: {}", packet);
    }
}
