/*!
    Sometimes you don't know the interfaces available on the given machine
    and you need a list of them.
*/

extern crate rawsock;
use rawsock::traits::Library;
use rawsock::{open_best_library, InterfaceDescription, pcap};

fn print_interfaces(interfs: Vec<InterfaceDescription>){
    println!("Found interfaces:");
    for (num, interf) in interfs.iter().enumerate() {
        println!("{}: {}, {}", num, interf.name, interf.description);
    }
}



fn main () {

    // this is generic version that work with the current library:
    let lib = open_best_library().expect("Could not open any library");
    let interfs = lib.all_interfaces().expect("Could not obtain interface list");
    print_interfaces(interfs);

    //this is a version with concrete library type
    //pcap and pfring support different subsets of interfaces and add some virtual interfaces
    //although "real" interfaces should be available in both of them
    let lib = pcap::Library::open_default_paths().expect("Could not open pcap library");
    let interfs = lib.all_interfaces().expect("Could not obtain interface list");
    print_interfaces(interfs);
}