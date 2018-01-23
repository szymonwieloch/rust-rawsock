use super::super::{Library};
use super::dll::PCapDll;
use dlopen::wrapper::Container;
use super::super::err::Error;
use super::interface::PCapInterface;
use super::dev_iter::PCapInterfaceDescriptionIterator;
use common::open_locations;


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

impl<'a> Library<'a> for PCap {
    type Interf = PCapInterface<'a>;
    fn open_default_locations() -> Result<Self, Error> {
        open_locations(&POSSIBLE_NAMES)
    }
    fn open(path: &str) -> Result<Self, Error> {
        let dll: Container<PCapDll> = unsafe { Container::load(path)}?;
        Ok(Self {
            dll
        })
    }

    fn open_interface(&'a self, name: & str) -> Result<PCapInterface<'a>, Error>{
       PCapInterface::new(name, &self.dll)
    }
}

impl PCap {
    pub fn get_devices<'a>(&'a self) -> Result<PCapInterfaceDescriptionIterator, Error> {
        PCapInterfaceDescriptionIterator::new(&self.dll)
    }
}

