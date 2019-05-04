use super::super::{Library, Interface, LibraryVersion};
use super::dll::PCapDll;
use dlopen::wrapper::Container;
use super::super::err::Error;
use super::interface::PCapInterface;
use std::ffi::CStr;


//Different platforms have different locations:

#[cfg(any(target_os = "macos", target_os = "ios"))]
pub const DEFAULT_PATHS: [&'static str; 2] = [
    "libpcap.A.dylib",
    "libpcap.dylib"
];

#[cfg(all(unix, not(any(target_os = "macos", target_os = "ios"))))]
pub const DEFAULT_PATHS: [&'static str; 4] = [
    "libpcap.so",
    "libpcap.so.0.9.5",
    "libpcap.so.0.9.4",
    "libpcap.so.0.8"
];

#[cfg(windows)]
pub const DEFAULT_PATHS: [&'static str; 4] = [
    "NPcap\\Packet.dll",
    "Packet.dll",
    "Npcap\\wpcap.dll",
    "wpcap.dll"
];

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

