mod packet;

pub use self::packet::{Packet, OwnedPacket, BorrowedPacket};

use dlopen::Error as DlopenError;
use std::io::{Error as IoError, ErrorKind as IoErrorKind};
use super::err::Error;
use std::iter::Iterator;


///Describes a network card device.
#[derive(Debug)]
pub struct Device {
    pub name: String,
    pub description: String
}

///Kind of data link - protocol used below the surface.
#[derive(Debug, Copy, Clone)]
pub enum DataLink{
    Ethernet,
    RawIp,
    Other
}



pub trait Interface<'a>{
    fn send(&self, packet: &[u8]) -> Result<(), Error>;
    fn receive<'b>(&'b mut self) -> Result<BorrowedPacket<'b>, Error>;
    fn flush(&self);
    fn data_link(&self) -> DataLink;
}

pub trait RawSock<'a>{
    type Interf: Interface<'a>;
    type DeviceIterator: Iterator<Item=Device>;
    fn default_locations() -> &'static [&'static str];
    fn open(path: &str) -> Result<Self, Error> where Self: Sized;
    fn open_default_locations() -> Result<Self, Error> where Self: Sized {
        let mut last_err = Error::DllError(DlopenError::OpeningLibraryError(IoError::new(IoErrorKind::Other, "No default locations")));
        for path in Self::default_locations() {
            match Self::open(path) {
                Ok(rawsock) => return Ok(rawsock),
                Err(err) => last_err = err
            }
        }
        Err(last_err)
    }
    fn open_interface(&'a self, name: &str) -> Result<Self::Interf, Error>;

    fn get_devices(&'a self) -> Result<Self::DeviceIterator, Error>;
}