mod packet;
pub use self::packet::{Packet, OwnedPacket, BorrowedPacket};
pub use crate::traits::{Library, Interface};
use crate::{wpcap, pcap, pfring};
use std::fmt::{Display, Formatter, Error as FmtError};

use super::err::Error;

///Kind of data link - protocol used below the surface.
#[derive(Debug, Copy, Clone)]
pub enum DataLink{
    Ethernet,
    RawIp,
    Other
}

impl Display for DataLink{
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            DataLink::Ethernet => write!(f, "ethernet"),
            DataLink::RawIp => write!(f, "raw IP"),
            DataLink::Other => write!(f, "other")
        }
    }
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
    if let Ok(l) = pfring::Library::open_default_paths() {
        return Ok(Box::new(l));
    }
    if let Ok(l) = wpcap::Library::open_default_paths() {
        return Ok(Box::new(l));
    }
    match pcap::Library::open_default_paths() {
        Ok(l) => Ok(Box::new(l)),
        Err(e) => Err(e)
    }
}

pub struct InterfaceData {
    pub name: String,
    pub description: String
}
