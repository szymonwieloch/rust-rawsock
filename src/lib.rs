/**

Rawsock is a crate that can help you to send and receive raw socket frames.
It simplifies loading one of packet capturing libraries (pcap, wpcap, pfring and libraries with
compatible APIs such as npcap are supported).
It also provides a consistent API for using these libraries, so that the internal complexity is
hidden.

# Main features

* Support of pcap, wpcap (with Windows-specific optimizations), npcap and pfring
* Libraries are loaded in a dynamic manner, so that the library does not have any direct
    dependency - it's going to work with whatever is available on the given platform.
* Consistent API for all packet capturing libraries.
* Provided wrapper that automatically chooses an implementation available on your platform.

# Example of a direct use of pcap library
```no_run
extern crate rawsock;
use rawsock::pcap;
use rawsock::traits::{Library, Interface};

fn main(){
    let lib = pcap::Library::open_default_paths().unwrap();
    let mut interf = lib.open_interface("/dev/eth0").unwrap(); //platform specific
    { //This block is required to limit borrowing scope
        let recv_packet = interf.receive().unwrap();
        println!("Received packet: {:?}", recv_packet);
    }
    let send_packet: [u8; 10] = [0,1,2,3,4,5,6,7,8,9];
    interf.send(&send_packet).unwrap();
}
```
#Example use of a automatic wrapper
```no_run
extern crate rawsock;
use rawsock::open_best_library;

fn main(){
    let lib = open_best_library().unwrap();
    let mut interf = lib.open_interface("/dev/eth0").unwrap(); //platform specific
    { //This block is required to limit borrowing scope
        let recv_packet = interf.receive().unwrap();
        println!("Received packet: {:?}", recv_packet);
    }
    let send_packet: [u8; 10] = [0,1,2,3,4,5,6,7,8,9];
    interf.send(&send_packet).unwrap();
}
```

*/

extern crate dlopen;
#[macro_use]
extern crate dlopen_derive;
extern crate libc;
extern crate time;

pub mod pcap;
pub mod wpcap;
pub mod pfring;
pub mod traits;
mod pcap_common;
mod utils;
mod err;
mod common;

pub use self::common::{Packet, BorrowedPacket, OwnedPacket, DataLink, LibraryVersion, open_best_library, InterfaceData};
pub use self::err::Error;
