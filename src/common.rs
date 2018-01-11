use dlopen::Error as DlopenError;
use std::io::{Error as IoError, ErrorKind as IoErrorKind};
use super::err::Error;
use time::Timespec;
use std::marker::PhantomData;

pub trait Packet{
    ///Returns content of the packet
    fn data(&self) -> &[u8];
    ///Returns the time when this packet was received.
    fn when(&self) -> Timespec;
}

///Structure representing obtained raw packet.
#[derive(Debug)]
pub struct BorrowedPacket<'a> {
    when_received: Timespec,
    packet: &'a[u8],
    //_marker: PhantomData<&'a mut u32>
}

impl<'a> Packet for BorrowedPacket<'a> {
    fn data(&self) -> &[u8] {
        self.packet
    }

    fn when(&self) -> Timespec {
        self.when_received
    }
}

pub struct OwnedPacket {
    when_received: Timespec,
    packet: Vec<u8>
}

impl Packet for OwnedPacket{
    fn data(&self) -> &[u8] {
        &self.packet
    }

    fn when(&self) -> Timespec {
        self.when_received
    }
}

impl OwnedPacket {
    pub fn new (data: &[u8], when: Timespec) -> Self{
        OwnedPacket {
            packet: data.into(),
            when_received: when
        }
    }
}

impl<'a> BorrowedPacket<'a> {
    ///Creates a new Packet instance.
    pub fn new(when_received: Timespec, data: &'a[u8]) -> BorrowedPacket<'a> {
        BorrowedPacket{
            when_received,
            packet: data,
            //_marker: PhantomData
        }
    }
    pub fn into_owned(self) -> OwnedPacket {
        OwnedPacket::new(self.packet, self.when_received)
    }
}

///Describes a network card device.
#[derive(Debug)]
pub struct Device {
    pub name: String,
    pub description: String
}

pub trait Interface<'a>{
    fn send(&self, packet: &[u8]) -> Result<(), Error>;
    fn receive<'b>(&'b mut self) -> Result<BorrowedPacket<'b>, Error>;
    fn flush(&self);
    fn get_ip(&self);
    fn get_mac(&self);
    fn get_default_gateway(&self);
}

pub trait RawSock<'a, I> where I: Interface<'a>{
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
    fn open_interface(&'a self, name: &str) -> Result<I, Error>;
}