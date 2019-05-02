mod packet;
mod traits;
pub use self::packet::{Packet, OwnedPacket, BorrowedPacket};
pub use self::traits::{Library, Interface, LibraryVersion};
pub use crate::{PFRing, PCap, WPCap};

use super::err::Error;

///Describes a network card device.
#[derive(Debug)]
pub struct InterfaceDescription {
    ///Name of the interface.
    pub name: String,
    ///Description of the interface.
    pub description: String
}

///Kind of data link - protocol used below the surface.
#[derive(Debug, Copy, Clone)]
pub enum DataLink{
    Ethernet,
    RawIp,
    Other
}




pub fn open_best_library() -> Result<Box<dyn Library>, Error> {
    if let Ok(l) = PFRing::open_default_paths() {
        return Ok(Box::new(l));
    }
    if let Ok(l) = WPCap::open_default_paths() {
        return Ok(Box::new(l));
    }
    match PCap::open_default_paths() {
        Ok(l) => Ok(Box::new(l)),
        Err(e) => Err(e)
    }
}
