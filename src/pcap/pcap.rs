use super::super::{Library, Interface, LibraryVersion};
use super::dll::PCapDll;
use dlopen::wrapper::Container;
use super::super::err::Error;
use super::interface::PCapInterface;
use std::ffi::CStr;
use super::paths::DEFAULT_PATHS;




///Instance of a opened pcap library.
pub struct PCapLibrary {
    dll: Container<PCapDll>
}

impl Library for PCapLibrary {
    fn default_paths() -> &'static [&'static str] where Self: Sized {
        &DEFAULT_PATHS
    }

    //const DEFAULT_PATHS: &'static [&'static str] = &POSSIBLE_NAMES;

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

    fn version(&self) -> LibraryVersion {
        LibraryVersion::PCap(unsafe{CStr::from_ptr(self.dll.pcap_lib_version())}.to_string_lossy().into_owned())
    }
}

impl PCapLibrary {
    pub fn open_interface(&self, name: & str) -> Result<PCapInterface, Error>{
       PCapInterface::new(name, &self.dll)
    }

}

