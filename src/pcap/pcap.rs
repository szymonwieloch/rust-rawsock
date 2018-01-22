use super::super::{RawSock};
use super::dll::{PCapDll, PCapErrBuf};
use dlopen::wrapper::Container;
use super::super::err::Error;
use libc::{c_char};
use super::interface::PCapInterface;
use super::dev_iter::PCapDeviceIterator;



const POSSIBLE_NAMES: [&'static str; 10] = [
    //OSX
    "libpcap.A.dylib",
    "libpcap.dylib",
    //Unix
    "libpcap.so",
    "libpcap.so.0.9.5",
    "libpcap.so.0.9.4",
    "libpcap.so.0.8",
    //Windows
    "NPcap\\Packet.dll",
    "Packet.dll",
    "Npcap\\wpcap.dll",
    "wpcap.dll"
];

pub struct PCap {
    dll: Container<PCapDll>

}

impl<'a> RawSock<'a> for PCap {
    type Interf = PCapInterface<'a>;
    fn default_locations() -> &'static [&'static str] {
        &POSSIBLE_NAMES
    }
    fn open(path: &str) -> Result<Self, Error> {
        let dll: Container<PCapDll> = unsafe { Container::load(path)}?;
        Ok(Self {
            dll: dll
        })
    }

    fn open_interface(&'a self, name: & str) -> Result<PCapInterface<'a>, Error>{
       PCapInterface::new(name, &self.dll)
    }
}

impl PCap {
    pub fn get_devices<'a>(&'a self) -> Result<PCapDeviceIterator, Error> {
        PCapDeviceIterator::new(&self.dll)
    }
}

