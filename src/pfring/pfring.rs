use super::super::{Library, Interface, Packet, InterfaceDescription, DataLink};
use super::interface::PFRingInterface;
use dlopen::wrapper::Container;
use super::super::err::Error;
use super::dll::PFRingDll;
use common::open_locations;

const POSSIBLE_NAMES: [&'static str; 1] = [
    "libpfring.so"
];

pub struct PFRing {
    dll: Container<PFRingDll>
}



impl<'a> Library<'a> for PFRing {

    type Interf = PFRingInterface<'a>;

    fn open(path: &str) -> Result<Self, Error> where Self: Sized {
        let dll: Container<PFRingDll> = unsafe { Container::load(path)}?;
        Ok(Self {
            dll
        })
    }

    fn open_interface(&'a self, name: &str) -> Result<Self::Interf, Error> {
        PFRingInterface::new(name, &self.dll)
    }
    fn open_default_locations() -> Result<Self, Error> where Self: Sized {
        open_locations(&POSSIBLE_NAMES)
    }
}