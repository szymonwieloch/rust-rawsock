extern crate rawsock;
use rawsock::traits::{Library, PcapLibrary};
use rawsock::{wpcap, pcap};
use rawsock::InterfaceDescription;

fn open_library<T>() -> T where T: Library {
    let lib = T::open_default_paths().expect("Could not open library");
    println!("Library opened, version is {}", lib.version());
    lib
}

fn print_interfaces(interfs: Vec<InterfaceDescription>){
    println!("Found interfaces:");
    for (num, interf) in interfs.iter().enumerate() {
        println!("{}: {}, {}", num, interf.name, interf.description);
    }
}



fn main () {
    run_pcap();
    run_wpcap();
}

fn run_pcap(){
    let lib = open_library::<pcap::Library>();
    let interfs = lib.all_interfaces().expect("Couldnot obtain interface list");
    print_interfaces(interfs);
}

fn run_wpcap() {
    let lib = open_library::<wpcap::Library>();
    let interfs = lib.all_interfaces().expect("Couldnot obtain interface list");
    print_interfaces(interfs);
}