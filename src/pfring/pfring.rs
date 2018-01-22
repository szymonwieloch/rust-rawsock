use super::super::{RawSock, Interface, Packet, Device, DataLink};
use super::interface::PFRingInterface;
use dlopen::wrapper::Container;
use super::super::err::Error;
use super::dll::PFRingDll;

const POSSIBLE_NAMES: [&'static str; 1] = [
    "libpfring.so"
];

pub struct PFRing {
    dll: Container<PFRingDll>
}



impl<'a> RawSock<'a> for PFRing {

    type Interf = PFRingInterface<'a>;

    fn default_locations() -> &'static [&'static str] {
        &POSSIBLE_NAMES
    }

    fn open(path: &str) -> Result<Self, Error> where Self: Sized {
        let dll: Container<PFRingDll> = unsafe { Container::load(path)}?;
        Ok(Self {
            dll
        })
    }

    fn open_interface(&'a self, name: &str) -> Result<Self::Interf, Error> {
        PFRingInterface::new(name, &self.dll)
    }
}