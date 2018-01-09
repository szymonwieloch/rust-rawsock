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

mod pcap;
mod err;

pub use self::pcap::PCap;

pub use self::err::Error;
use dlopen::Error as DlopenError;
use std::io::{Error as IoError, ErrorKind as IoErrorKind};

pub trait RawSock {
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
    //fn open_interface<T>(& self, name: &str) -> T where T: Interface;
}

trait Interface {
    fn send(&self, packet: &[u8]) -> Result<(), Error>;
    fn receive(&self);
    fn flush(&self);
    fn get_ip(&self);
    fn get_mac(&self);
    fn get_default_gateway(&self);
}