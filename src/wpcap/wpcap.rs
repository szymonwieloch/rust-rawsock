use super::super::Library;
use super::dll::WPCapDll;
use dlopen::wrapper::Container;
use super::super::err::Error;
use super::interface::WPCapInterface;
use super::dev_iter::WPCapDeviceDescriptionIterator;
use common::open_locations;
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

impl<'a> Library<'a> for WPCap {
    type Interf = WPCapInterface<'a>;

    fn open(path: &str) -> Result<Self, Error> {
        let dll: Container<WPCapDll> = unsafe { Container::load(path)}?;
        Ok(Self {
            dll: dll
        })
    }

    fn open_interface(&'a self, name: & str) -> Result<WPCapInterface<'a>, Error>{
        WPCapInterface::new(name, &self.dll)
    }
    fn open_default_locations() -> Result<Self, Error> where Self: Sized {
        open_locations(&POSSIBLE_NAMES)
    }
    fn version(&self) -> String {
        unsafe{CStr::from_ptr(self.dll.pcap_lib_version())}.to_string_lossy().into_owned()
    }
}

impl WPCap {
    pub fn get_devices<'a>(&'a self) -> Result<WPCapDeviceDescriptionIterator, Error> {
        WPCapDeviceDescriptionIterator::new(&self.dll)
    }
}