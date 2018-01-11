/**
This module suppors loading of dynamic link libraries that allow capturing and sending packets.
There are two main libraries that can be used for this purpose: **libpcap** and **pfring**.
They are loaded in a dynamic manner so that this library does not have direct dependency on
any of them and can be configured in a runtime.
*/

extern crate dlopen;
#[macro_use]
extern crate dlopen_derive;
extern crate libc;
extern crate time;

mod pcap;
mod err;
mod common;

pub use self::pcap::PCap;

pub use self::common::{Packet, Interface, RawSock, Device, BorrowedPacket, OwnedPacket};
pub use self::err::Error;







