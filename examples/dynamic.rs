extern crate rawsock;
extern crate interfaces2;
mod commons;
use rawsock::{Library, open_best_library};
use self::commons::{find_first_interface_name, receive_packets};

fn main() {

    /*
    This example shows automatic choosing of the best underlying library available on your system
    and dynamic dispatch of calls to the right implementation.

    For most applications this is the recommended approach.
    */
    println!("Opening packet capturing library");
    let lib = open_best_library().expect("Could not open any packet capturing library");
    println!("Library opened, version is {}", lib.version());
    let interf_name = find_first_interface_name();
    println!("Opening the {} interface", interf_name);
    let mut interf = lib.open_interface(&interf_name).expect("Could not open network interface");

    //send some packets
    println!("Sending 5 packets:");
    for i in 0..5{
        let packet: [u8; 5] = [1*i,2*i,3*i,4*i,5*i];
        eprintln!("Sending packet {:?}", packet);
        interf.send(&packet).expect("Could not send packet");
    }

    //receive some packets.
    for _ in 0..5 {
        let packet = interf.receive().expect("Could not receive packet");
        println!("Received packet: {}", packet);
    }
}