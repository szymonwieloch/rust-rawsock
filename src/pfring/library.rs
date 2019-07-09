use crate::{LibraryVersion, traits};
use super::interface::Interface;
use dlopen::wrapper::Container;
use super::super::err::Error;
use super::dll::PFRingDll;
use super::paths::DEFAULT_PATHS;





///Instance of a opened pfring library.
pub struct Library {
    dll: Container<PFRingDll>
}



impl traits::Library for Library {
    fn default_paths() -> &'static [&'static str] where Self: Sized {
        &DEFAULT_PATHS
    }

    //const DEFAULT_PATHS: &'static [&'static str] = &POSSIBLE_NAMES;

    fn open(path: &str) -> Result<Self, Error> where Self: Sized {
        let dll: Container<PFRingDll> = unsafe { Container::load(path)}?;
        Ok(Self {
            dll
        })
    }

    fn open_interface<'a>(&'a self, name: &str) -> Result<Box<traits::Interface<'a> +'a>, Error> {
        match self.open_interface(name){
            Ok(interf) => Ok(Box::new(interf) as Box<traits::Interface>),
            Err(e) => Err(e)
        }
    }

    fn version(&self) -> LibraryVersion {
        let mut ver: u32 = 0;
        unsafe{self.dll.pfring_version_noring(&mut ver)};
        let major: u8 = (ver >>16) as u8;
        let minor: u8 = (ver >> 8) as u8;
        let release: u8 = ver as u8;
        LibraryVersion::PFRing(format!("{}.{}.{}", major, minor, release))
    }
}

impl Library {
    pub fn open_interface(& self, name: &str) -> Result<Interface, Error> {
        Interface::new(name, &self.dll)
    }
}