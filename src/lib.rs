/**

# Overview
**rawsock** is a Rust library that highly simplifies use of packet capturing libraries
such as **pcap**, **wpcap** or **pf_ring** and also libraries with a compatible API, such as **npcap**.
It can help you to send and receive raw socket frames using one consistent API for all those libraries,
so that the internal complexity is hidden.

Main advantage: you write code using one simple API - rawsock. But when your application is run,
the best available library on user's machine is chosen and used in the background.
Platform-specific extensions (such as WinPcap ```pcap_sendqueue_transmit()```) are also used in optimal way.

# Main features

* One consistent API for all packet capturing libraries.
* Support of pcap, wpcap (with Windows-specific optimizations), npcap and pfring
* Supports all main platforms: tested on Windows, Linux, Mac. Many more should work too
* Libraries are loaded in a dynamic manner, so that the library does not have any direct
    dependency - it's going to work with whatever is available on the given platform.
* Libraries are checked in the order of effectiveness. The best found library is loaded.
* It is also possible to load a specific library when using the rawsock API
or directly use the API of the given dynamically loaded library - for more advanced use cases.

# Quick example

```no_run
extern crate rawsock;
use rawsock::open_best_library;

const ICMP_PACKET: [u8; 84] = [
0x45, 0x00, 0x00, 0x54, 0xee, 0x96, 0x40, 0x00, 0x40, 0x01, 0x79, 0xf0, 0xc0, 0xa8, 0x01, 0x6a,
0x08, 0x08, 0x08, 0x08, 0x08, 0x00, 0x2f, 0x08, 0x66, 0xc2, 0x00, 0x12, 0x82, 0xaa, 0xcc, 0x5c,
0x00, 0x00, 0x00, 0x00, 0x51, 0x49, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x11, 0x12, 0x13,
0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23,
0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33,
0x34, 0x35, 0x36, 0x37];

fn main() {

    /*
    This example shows automatic choosing of the best underlying library available on your system
    and dynamic dispatch of calls to the right implementation.

    For most applications this is the recommended approach.
    */
    println!("Opening packet capturing library");
    let lib = open_best_library().expect("Could not open any packet capturing library");
    println!("Library opened, version is {}", lib.version());
    let interf_name = "eth0"; //replace with whatever is available on your platform
    println!("Opening the {} interface", interf_name);
    let mut interf = lib.open_interface(&interf_name).expect("Could not open network interface");
    println!("Interface opened, data link: {}", interf.data_link());

    //send some packets
    println!("Sending 5 packets:");
    for i in 0..5{
        println!("Sending ICMP ping packet no {}",i);
        interf.send(&ICMP_PACKET).expect("Could not send packet");
    }

    //receive some packets.
    println!("Receiving 5 packets:");
    for _ in 0..5 {
        let packet = interf.receive().expect("Could not receive packet");
        println!("Received packet: {}", packet);
    }
}
```

*/

extern crate dlopen;
#[macro_use]
extern crate dlopen_derive;
#[macro_use]
extern crate bitflags;
extern crate libc;
extern crate time;

pub mod pcap;
pub mod wpcap;
pub mod pfring;
pub mod traits;
mod pcap_common;
mod utils;
mod common;

pub use self::common::{Packet, BorrowedPacket, OwnedPacket, DataLink, LibraryVersion, open_best_library, open_best_library_arc,InterfaceDescription, Error, Stats};
