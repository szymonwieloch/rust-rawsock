use super::super::{RawSock, Interface};
use super::dll::{PCapDll, PCapHandle, PCapPacketHeader};
use dlopen::wrapper::Container;
use super::super::err::Error;
use std::ffi::{CStr, CString};
use libc::{c_char, c_void, c_uint, c_int};
use std::mem::uninitialized;

const PCAP_ERRBUF_SIZE: usize = 256; //taken from header, is it platform independent?
const PCAP_SUCCESS: c_int = 0;

const POSSIBLE_NAMES: [&'static str; 10] = [
    //OSX
    "libpcap.A.dylib",
    "libpcap.dylib",
    //Unix
    "libpcap.so",
    "libpcap.so.0.9.5",
    "libpcap.so.0.9.4",
    "libpcap.so.0.8",
    //Windows
    "NPcap\\Packet.dll",
    "Packet.dll",
    "Npcap\\wpcap.dll",
    "wpcap.dll"
];



pub struct PCap {
    dll: Container<PCapDll>

}

pub struct PCapInterface<'a> {
    handle: * const PCapHandle,
    dll: & 'a PCapDll
}

impl<'a> PCapInterface<'a> {
    fn new(handle: * const PCapHandle, dll: &'a PCapDll) ->Self {
        PCapInterface{
            dll: dll,
            handle: handle
        }
    }
}

impl<'a> Drop for PCapInterface<'a> {
    fn drop(&mut self) {
        unsafe { self.dll.pcap_close(self.handle) }
    }
}

impl<'a> Interface for PCapInterface<'a> {
    fn send(&self, packet: &[u8]) -> Result<(), Error> {
        if unsafe {self.dll.pcap_sendpacket(self.handle, packet.as_ptr(), packet.len() as c_int)} == PCAP_SUCCESS {
            Ok(())
        } else {
            let txt = unsafe {CStr::from_ptr(self.dll.pcap_geterr(self.handle))}.to_string_lossy().into_owned();
            Err(Error::SendingPacket(txt))
        }
    }

    fn receive(&self) {
        let mut header: PCapPacketHeader = unsafe {uninitialized()};
        let data = unsafe { self.dll.pcap_next(self.handle, &mut header)};
    }

    fn flush(&self) {
        unimplemented!()
    }

    fn get_ip(&self) {
        unimplemented!()
    }

    fn get_mac(&self) {
        unimplemented!()
    }

    fn get_default_gateway(&self) {
        unimplemented!()
    }
}

impl PCap {
    pub fn open_interface(&self, name: &str) -> Result<PCapInterface, Error> {
        let name = CString::new(name)?;
        let mut errbuf: [c_char; PCAP_ERRBUF_SIZE] = [0;PCAP_ERRBUF_SIZE];
        let handle = unsafe { self.dll.pcap_open_live(
            name.as_ptr(),
            65536,                  /* max packet size */
            8,                      /* promiscuous mode */
            1000,                   /* read timeout in milliseconds */
            errbuf.as_mut_ptr()
        )};
        if handle.is_null() {
            let msg = unsafe {CStr::from_ptr(errbuf.as_ptr())}.to_string_lossy().into_owned();
            Err(Error::OpeningInterface(msg))
        } else {
            Ok(PCapInterface::new(handle, &self.dll))
        }
    }
}

impl RawSock for PCap {
    fn default_locations() -> &'static [&'static str] {
        &POSSIBLE_NAMES
    }
    fn open(path: &str) -> Result<Self, Error> {
        let dll: Container<PCapDll> = unsafe { Container::load(path)}?;
        Ok(Self {
            dll: dll
        })
    }

}