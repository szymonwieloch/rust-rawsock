use crate::{LibraryVersion, traits, InterfaceDescription};
use super::interface::Interface;
use dlopen::wrapper::Container;
use crate::Error;
use super::dll::PFRingDll;
use super::paths::DEFAULT_PATHS;
use crate::utils::cstr_to_string;
use std::sync::Arc;


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

    fn open_interface<'a>(&'a self, name: &str) -> Result<Box<traits::DynamicInterface<'a> +'a>, Error> {
        match self.open_interface(name){
            Ok(interf) => Ok(Box::new(interf) as Box<traits::DynamicInterface>),
            Err(e) => Err(e)
        }
    }

    fn open_interface_arc<'a>(&'a self, name: &str) -> Result<Arc<traits::DynamicInterface<'a> + 'a>, Error> {
        match Interface::new(name, &self.dll){
            Ok(interf) => Ok(Arc::new(interf) as Arc<traits::DynamicInterface>),
            Err(e) => Err(e)
        }
    }

    fn all_interfaces(&self) -> Result<Vec<InterfaceDescription>, Error> {
        let interfs = unsafe{self.dll.pfring_findalldevs()};
        let mut curr = interfs;
        let mut result = Vec::new();
        while !curr.is_null() {
            let system_name = cstr_to_string(unsafe{(*curr).system_name});
            let module = cstr_to_string(unsafe{(*curr).module});
            let sn = cstr_to_string(unsafe{(*curr).sn});
            let id = InterfaceDescription{
                name: cstr_to_string(unsafe{(*curr).name}),
                description: format!("{}, {}, {}", &system_name, &module, &sn)
            };
            result.push(id);
            curr=unsafe{(*curr).next};
        }
        unsafe{self.dll.pfring_freealldevs(interfs)};
        Ok(result)
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
    pub fn dll(&self) -> &PFRingDll {
        &self.dll
    }
}