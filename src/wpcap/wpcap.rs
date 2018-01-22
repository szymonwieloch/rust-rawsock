use super::super::{RawSock, Interface, Packet, Device, DataLink};
use super::dll::{WPCapDll, PCapErrBuf};
use dlopen::wrapper::Container;
use super::super::err::Error;
use std::ffi::{CStr, CString};
use libc::{c_char};
use super::interface::WPCapInterface;
use super::dev_iter::WPCapDeviceIterator;



const POSSIBLE_NAMES: [&'static str; 4] = [
    "NPcap\\Packet.dll",
    "Packet.dll",
    "Npcap\\wpcap.dll",
    "wpcap.dll"
];

pub struct WPCap {
    dll: Container<WPCapDll>
}

impl<'a> RawSock<'a> for WPCap {
    type Interf = WPCapInterface<'a>;
    fn default_locations() -> &'static [&'static str] {
        &POSSIBLE_NAMES
    }
    fn open(path: &str) -> Result<Self, Error> {
        let dll: Container<WPCapDll> = unsafe { Container::load(path)}?;
        Ok(Self {
            dll: dll
        })
    }

    fn open_interface(&'a self, name: & str) -> Result<WPCapInterface<'a>, Error>{
        WPCapInterface::new(name, &self.dll)
    }
}

impl WPCap {
    pub fn get_devices<'a>(&'a self) -> Result<WPCapDeviceIterator, Error> {
        WPCapDeviceIterator::new(&self.dll)
    }
}