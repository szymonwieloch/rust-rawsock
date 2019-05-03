use interfaces2::Interface as NetInterf;
extern crate rawsock;
use rawsock::Interface;

pub fn find_first_interface_name() -> String {
    let interfaces = NetInterf::get_all().expect("Could not get list of interfaces");
    println!{"You have the following interfaces available on your platform:"}
    for interf in &interfaces {
        println!("- {}", interf.name)
    }
    let first = interfaces.first().expect("There are no network interfaces in your system").name.clone();
    println!("Chosen first interface is: {}", first);
    first
}

pub fn receive_packets<'a, T>(interf: &mut T) where T: Interface<'a>{
    println!("Receiving packets:");
    for i in 1..6{
        let packet = interf.receive().expect("Error while receiving packet");
        println!("Packet {} is {}", i, packet);
    }
}