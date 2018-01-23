use super::dll::{PCapDll, SUCCESS, PCapInterface, PCapErrBuf};
use super::super::Error;
use std::ffi::CStr;
use std::ptr::null;
use InterfaceDescription;

///pcap iterator of device descriptions.
pub struct PCapInterfaceDescriptionIterator<'a>{
    first: * const PCapInterface,
    current: * const PCapInterface,
    dll: &'a PCapDll
}

impl<'a> PCapInterfaceDescriptionIterator<'a> {
    pub fn new(dll: &'a PCapDll) -> Result<Self, Error> {
        let mut interf: * const PCapInterface = null();
        let mut errbuf = PCapErrBuf::new();
        if unsafe{dll.pcap_findalldevs(&mut interf, errbuf.buffer())} == SUCCESS {
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

impl<'a> Iterator for PCapInterfaceDescriptionIterator<'a>{
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

impl<'a> Drop for PCapInterfaceDescriptionIterator<'a>{
    fn drop(&mut self) {
        unsafe{self.dll.pcap_freealldevs(self.first)}
    }
}