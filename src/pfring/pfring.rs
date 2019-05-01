use super::super::Library;
use super::interface::PFRingInterface;
use dlopen::wrapper::Container;
use super::super::err::Error;
use super::dll::PFRingDll;
use crate::common::open_locations;

const POSSIBLE_NAMES: [&'static str; 1] = [
    "libpfring.so"
];

///Instance of a opened pfring library.
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
    fn version(&self) -> String {
        let mut ver: u32 = 0;
        unsafe{self.dll.pfring_version_noring(&mut ver)};
        let major: u8 = (ver >>16) as u8;
        let minor: u8 = (ver >> 8) as u8;
        let release: u8 = ver as u8;
        format!("{}.{}.{}", major, minor, release)
    }
}