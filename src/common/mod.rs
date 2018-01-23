mod packet;
mod rawsock;
mod traits;

pub use self::packet::{Packet, OwnedPacket, BorrowedPacket};
pub use self::rawsock::{RawInterf, RawLib};
pub use self::traits::{Library, Interface};

use dlopen::Error as DlopenError;
use std::io::{Error as IoError, ErrorKind as IoErrorKind};
use super::err::Error;
use std::iter::Iterator;


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

///Used internally by several implementations of the Library trait.
pub fn open_locations<'a, T>(locations: &[&'static str]) -> Result<T, Error> where T: Sized+Library<'a> {
    let mut last_err = Error::DllError(DlopenError::OpeningLibraryError(IoError::new(IoErrorKind::Other, "No default locations")));
    for path in locations {
        match T::open(path) {
            Ok(lib) => return Ok(lib),
            Err(err) => last_err = err
        }
    }
    Err(last_err)
}