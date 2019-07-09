use crate::{LibraryVersion, traits};
use super::dll::PCapDll;
use dlopen::wrapper::Container;
use super::super::err::Error;
use super::interface::Interface;
use std::ffi::CStr;
use super::paths::DEFAULT_PATHS;




///Instance of a opened pcap library.
pub struct Library {
    dll: Container<PCapDll>
}

impl traits::Library for Library {
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

    fn open_interface<'a>(&'a self, name: &str) -> Result<Box<traits::Interface<'a> +'a>, Error> {
        match self.open_interface(name){
            Ok(interf) => Ok(Box::new(interf) as Box<traits::Interface>),
            Err(e) => Err(e)
        }
    }

    fn version(&self) -> LibraryVersion {
        LibraryVersion::PCap(unsafe{CStr::from_ptr(self.dll.pcap_lib_version())}.to_string_lossy().into_owned())
    }
}

impl Library {
    pub fn open_interface(&self, name: & str) -> Result<Interface, Error>{
       Interface::new(name, &self.dll)
    }

}

