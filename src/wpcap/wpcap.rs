use super::super::{Library, Interface};
use super::dll::WPCapDll;
use dlopen::wrapper::Container;
use super::super::err::Error;
use super::interface::WPCapInterface;
use super::dev_iter::WPCapDeviceDescriptionIterator;
use std::ffi::CStr;



const POSSIBLE_NAMES: [&'static str; 4] = [
    "NPcap\\Packet.dll",
    "Packet.dll",
    "Npcap\\wpcap.dll",
    "wpcap.dll"
];

pub struct WPCap {
    dll: Container<WPCapDll>
}

impl Library for WPCap {

    const DEFAULT_PATHS: &'static [&'static str] = &POSSIBLE_NAMES;

    fn open(path: &str) -> Result<Self, Error> {
        let dll: Container<WPCapDll> = unsafe { Container::load(path)}?;
        Ok(Self {
            dll: dll
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

impl WPCap {
    pub fn get_devices<'a>(&'a self) -> Result<WPCapDeviceDescriptionIterator, Error> {
        WPCapDeviceDescriptionIterator::new(&self.dll)
    }

    fn open_interface(& self, name: & str) -> Result<WPCapInterface, Error>{
        WPCapInterface::new(name, &self.dll)
    }
}