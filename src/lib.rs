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
mod wpcap;
mod pcap_common;
mod pfring;
mod err;
mod common;

pub use self::pcap::{PCap, PCapInterfaceDescriptionIterator, PCapInterface};
pub use self::wpcap::{WPCap, WPCapDeviceIterator, WPCapInterface};
pub use self::pfring::{PFRing, PFRingInterface};

pub use self::common::{Packet, Interface, Library, InterfaceDescription, BorrowedPacket, OwnedPacket, DataLink, RawLib, RawInterf};
pub use self::err::Error;







