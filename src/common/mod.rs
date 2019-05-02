mod packet;
mod traits;
pub use self::packet::{Packet, OwnedPacket, BorrowedPacket};
pub use self::traits::{Library, Interface, LibraryVersion};

use dlopen::Error as DlopenError;
use std::io::{Error as IoError, ErrorKind as IoErrorKind};
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



/*
fn open_best_library() -> Result<Box<dyn Library>, Error> {
    panic!("Not yet implemented")
}*/
