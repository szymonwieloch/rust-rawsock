/**
    Static and dispatch in a loop in a worker thread wih break on demand.
*/

extern crate rawsock;
extern crate crossbeam_utils;
use crossbeam_utils::thread;
use rawsock::{open_best_library};
mod commons;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    dynamic_loop();
}

fn dynamic_loop(){
    let lib = open_best_library().expect("Could not open any library");
    let ifname = lib.all_interfaces()
        .expect("Could not obtain interface list").first()
        .expect("There are no available interfaces").name.clone();
    println!("Library version is {}, opening {} interface", lib.version(), &ifname);
    let interf = lib.open_interface(&ifname).expect("Could not open pcap interface");

    //To compile the code we need a thread scope to guarantee that inter2 will not outlive lib
    thread::scope(|s| {
        s.spawn(|_| {
            interf.loop_infinite_dyn(&  |packet|{
                println!("Received packet: {}", packet);
            }).expect("Error when running receiving loop");
        });
        println!("Waiting 5 seconds");
        sleep(Duration::from_secs(5));
        println!("Breaking the loop");
        interf.break_loop();
        println!("Loop is broken");
    }).unwrap();

}