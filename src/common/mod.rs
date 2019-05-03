mod packet;
mod traits;
pub use self::packet::{Packet, OwnedPacket, BorrowedPacket};
pub use self::traits::{Library, Interface};
use crate::pcap::PCapLibrary;
use crate::pfring::PFRingLibrary;
use crate::wpcap::WPCapLibrary;
use std::fmt::{Display, Formatter, Error as FmtError};

use super::err::Error;

///Kind of data link - protocol used below the surface.
#[derive(Debug, Copy, Clone)]
pub enum DataLink{
    Ethernet,
    RawIp,
    Other
}

///Kind of library and its version.
#[derive(Debug, Clone)]
pub enum LibraryVersion{
    PCap(String),
    WPCap(String),
    PFRing(String)
}

impl Display for LibraryVersion {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            LibraryVersion::PCap(ver) => write!(f, " pcap {}", ver),
            LibraryVersion::WPCap(ver) => write!(f, "wpcap {}", ver),
            LibraryVersion::PFRing(ver) => write!(f, "pfring {}", ver)
        }
    }
}


pub fn open_best_library() -> Result<Box<dyn Library>, Error> {
    if let Ok(l) = PFRingLibrary::open_default_paths() {
        return Ok(Box::new(l));
    }
    if let Ok(l) = WPCapLibrary::open_default_paths() {
        return Ok(Box::new(l));
    }
    match PCapLibrary::open_default_paths() {
        Ok(l) => Ok(Box::new(l)),
        Err(e) => Err(e)
    }
}
