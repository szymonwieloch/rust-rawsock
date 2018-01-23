use super::dll::{WPCapDll, SUCCESS, PCapInterface, PCapErrBuf};
use super::super::Error;
use std::ffi::CStr;
use std::ptr::null;
use InterfaceDescription;

pub struct WPCapDeviceIterator<'a>{
    first: * const PCapInterface,
    current: * const PCapInterface,
    dll: &'a WPCapDll
}

impl<'a> WPCapDeviceIterator<'a> {
    pub fn new(dll: &'a WPCapDll) -> Result<Self, Error> {
        let mut interf: * const PCapInterface = null();
        let mut errbuf = PCapErrBuf::new();
        if unsafe{dll.pcap_findalldevs(&interf, errbuf.buffer())} == SUCCESS {
            Ok(Self{
                dll,
                current: interf,
                first: interf
            })
        } else {
            Err(Error::GettingDeviceDescriptionList(errbuf.as_string()))
        }
    }
}

impl<'a> Iterator for WPCapDeviceIterator<'a>{
    type Item = InterfaceDescription;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_null(){
            None
        } else {
            let tmp = self.current;
            self.current = unsafe { &*self.current }.next;
            Some(InterfaceDescription {
                name: unsafe{CStr::from_ptr((*tmp).name)}.to_string_lossy().into_owned(),
                description: unsafe{CStr::from_ptr((*tmp).description)}.to_string_lossy().into_owned()
            })
        }
    }
}

impl<'a> Drop for WPCapDeviceIterator<'a>{
    fn drop(&mut self) {
        unsafe{self.dll.pcap_freealldevs(self.first)}
    }
}