/**

Rawsock is a crate that can help you to send and receive raw socket frames.
It simplifies loading one of packet capturing libraries (pcap, wpcap, pfring and libraries with
compatible APIs such as npcap are supported).
It also provides a consistent API for using these libraries, so that the internal complexity is
hidden.

# Main features

* Support of pcap, wpcap (with Windows-specific optimizations), npcap and pfring
* Libraries are loaded in a dynamic manner, so that the library does not havee any direct
    dependency - it's going to work with whatever is available on the given platform.
* Consistent API for all packet capturing libraries.
* Provided wrapper that automatically chooses an implementation available on your platform.

# Example of a direct use of pcap library
```no_run
extern crate rawsock;
use rawsock::{PCap, Library, Interface};

fn main(){
    let lib = PCap::open_default_locations().unwrap();
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
use rawsock::{RawLib, Library, Interface};

fn main(){
    let lib = RawLib::open_default_locations().unwrap();
    let mut interf = lib.open_interface("/dev/eth0").unwrap(); //platform specific
    { //This block is required to limit borrowing scope
        let recv_packet = interf.receive().unwrap();
        println!("Received packet: {:?}", recv_packet);
    }
    let send_packet: [u8; 10] = [0,1,2,3,4,5,6,7,8,9];
    interf.send(&send_packet).unwrap();
}
```

#Example implementation using a template and a trait
```no_run
extern crate rawsock;
use rawsock::{PCap, Library, Interface, Error};

struct Sender<L> where L: for<'a> Library<'a> {
    lib: L
}

impl<L> Sender<L> where L: for<'a> Library<'a> {
    pub fn new() -> Result<Self, Error> {
        Ok(Self{
            lib: L::open_default_locations()?
        })
    }

    pub fn do_something(&self) {
        // do something complex with the library
        let interf = self.lib.open_interface("/dev/eth0").unwrap(); //platform specific
        let packet1: [u8; 10] = [0,1,2,3,4,5,6,7,8,9];
        let packet2: [u8; 10] = [9,8,7,6,5,4,3,2,1,0];
        interf.send(&packet1).unwrap();
        interf.send(&packet2).unwrap();
    }
}

fn main(){
    //you can use WPCap, PFRing or RawLib instead of PCap
    let sender: Sender<PCap> = Sender::new().unwrap();
    sender.do_something();
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
mod pcap_common;

mod err;
mod common;

pub use self::common::{Packet, Interface, Library, BorrowedPacket, OwnedPacket, DataLink, LibraryVersion, open_best_library};
pub use self::err::Error;
