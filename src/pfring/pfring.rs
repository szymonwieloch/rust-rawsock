use super::super::{RawSock, Interface, Packet, Device, DataLink};
use super::dev_iter::PFRingDeviceIterator;
use super::interface::PFRingInterface;
use dlopen::wrapper::Container;
use super::super::err::Error;

struct PFRing {

}

impl<'a> RawSock<'a> for PFRing {
    fn default_locations() -> &'static [&'static str] {
        unimplemented!()
    }

    fn open(path: &str) -> Result<Self, Error> where Self: Sized {
        unimplemented!()
    }

    fn open_interface(&'a self, name: &str) -> Result<Self::Interf, Error> {
        unimplemented!()
    }

    fn get_devices(&'a self) -> Result<Self::DeviceIterator, Error> {
        unimplemented!()
    }
    type Interf = PFRingInterface<'a>;
    type DeviceIterator = PFRingDeviceIterator;
}