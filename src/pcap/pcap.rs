use super::super::{Library, Interface};
use super::dll::PCapDll;
use dlopen::wrapper::Container;
use super::super::err::Error;
use super::interface::PCapInterface;
use super::dev_iter::PCapInterfaceDescriptionIterator;
use std::ffi::CStr;


//Different platforms have different locations:

#[cfg(any(target_os = "macos", target_os = "ios"))]
const POSSIBLE_NAMES: [&'static str; 2] = [
    "libpcap.A.dylib",
    "libpcap.dylib"
];

#[cfg(all(unix, not(any(target_os = "macos", target_os = "ios"))))]
const POSSIBLE_NAMES: [&'static str; 4] = [
    "libpcap.so",
    "libpcap.so.0.9.5",
    "libpcap.so.0.9.4",
    "libpcap.so.0.8"
];

#[cfg(windows)]
const POSSIBLE_NAMES: [&'static str; 4] = [
    "NPcap\\Packet.dll",
    "Packet.dll",
    "Npcap\\wpcap.dll",
    "wpcap.dll"
];

///Instance of a opened pcap library.
pub struct PCap {
    dll: Container<PCapDll>
}

impl Library for PCap {

    const DEFAULT_PATHS: &'static [&'static str] = &POSSIBLE_NAMES;

    fn open(path: &str) -> Result<Self, Error> {
        let dll: Container<PCapDll> = unsafe { Container::load(path)}?;
        Ok(Self {
            dll
        })
    }

    fn open_interface<'a>(&'a self, name: &str) -> Result<Box<Interface<'a> +'a>, Error> {
        match self.open_interface(name){
            Ok(interf) => Ok(Box::new(interf) as Box<Interface>),
            Err(e) => Err(e)
        }
    }

    fn version(&self) -> String {
        unsafe{CStr::from_ptr(self.dll.pcap_lib_version())}.to_string_lossy().into_owned()
    }
}

impl PCap {
    pub fn get_devices(& self) -> Result<PCapInterfaceDescriptionIterator, Error> {
        PCapInterfaceDescriptionIterator::new(&self.dll)
    }

    fn open_interface(&self, name: & str) -> Result<PCapInterface, Error>{
       PCapInterface::new(name, &self.dll)
    }

}

