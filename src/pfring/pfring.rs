use crate::{Library, Interface, LibraryVersion};
use super::interface::PFRingInterface;
use dlopen::wrapper::Container;
use super::super::err::Error;
use super::dll::PFRingDll;

#[cfg(all(unix, not(any(target_os = "macos", target_os = "ios"))))]
pub const DEFAULT_PATHS: [&'static str; 2] = [
    "libpfring.so",
    "libpfring.so.1"
];

#[cfg(any(windows, target_os = "macos", target_os = "ios"))]
pub const DEFAULT_PATHS: [&'static str; 0] = [];



///Instance of a opened pfring library.
pub struct PFRingLibrary {
    dll: Container<PFRingDll>
}



impl Library for PFRingLibrary {
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

    fn open_interface<'a>(&'a self, name: &str) -> Result<Box<Interface<'a> +'a>, Error> {
        match self.open_interface(name){
            Ok(interf) => Ok(Box::new(interf) as Box<Interface>),
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

impl PFRingLibrary {
    pub fn open_interface(& self, name: &str) -> Result<PFRingInterface, Error> {
        PFRingInterface::new(name, &self.dll)
    }
}