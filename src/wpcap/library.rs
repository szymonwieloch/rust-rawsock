use crate::{LibraryVersion, traits};
use super::dll::WPCapDll;
use dlopen::wrapper::Container;
use crate::Error;
use super::interface::Interface;
use super::paths::DEFAULT_PATHS;
use crate::pcap_common::{PCapErrBuf, PCapInterface};
use crate::pcap_common::constants::SUCCESS;
use std::ptr::null;
use crate::common::InterfaceDescription;
use crate::pcap_common::interface_data_from_pcap_list;
use crate::utils::cstr_to_string;
use std::sync::Arc;


///Instance of a opened wpcap library.
pub struct Library {
    dll: Container<WPCapDll>
}

impl traits::Library for Library {
    fn default_paths() -> &'static [&'static str] where Self: Sized {
        &DEFAULT_PATHS
    }

    //const DEFAULT_PATHS: &'static [&'static str] = &POSSIBLE_NAMES;

    fn open(path: &str) -> Result<Self, Error> {
        let dll: Container<WPCapDll> = unsafe { Container::load(path) }?;
        Ok(Self {
            dll
        })
    }

    fn open_interface<'a>(&'a self, name: &str) -> Result<Box<dyn traits::DynamicInterface<'a> + 'a>, Error> {
        match self.open_interface(name) {
            Ok(interf) => Ok(Box::new(interf) as Box<dyn traits::DynamicInterface>),
            Err(e) => Err(e)
        }
    }

    fn version(&self) -> LibraryVersion {
        LibraryVersion::WPCap(cstr_to_string(unsafe{self.dll.pcap_lib_version()}))
    }

    fn all_interfaces(&self) -> Result<Vec<InterfaceDescription>, Error> {
        let mut interfs: *const PCapInterface = null();
        let mut errbuf = PCapErrBuf::new();
        if SUCCESS != unsafe { self.dll.pcap_findalldevs(&mut interfs, errbuf.buffer()) } {
            return Err(Error::GettingDeviceDescriptionList(errbuf.as_string()))
        }
        let interf_datas = interface_data_from_pcap_list(interfs);

        unsafe { self.dll.pcap_freealldevs(interfs) }
        Ok(interf_datas)
    }

    fn open_interface_arc<'a>(&'a self, name: &str) -> Result<Arc<dyn traits::DynamicInterface<'a> +'a>, Error> {
        match Interface::new(name, &self.dll) {
            Ok(interf) => Ok(Arc::new(interf) as Arc<dyn traits::DynamicInterface>),
            Err(e) => Err(e)
        }
    }
}

impl Library {

    pub fn open_interface(& self, name: & str) -> Result<Interface, Error>{
        Interface::new(name, &self.dll)
    }
    pub fn dll(&self) -> &WPCapDll {
        &self.dll
    }
}