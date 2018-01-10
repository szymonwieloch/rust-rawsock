use super::super::{RawSock, Interface};
use super::dll::{PCapDll, PCapHandle, PCapPacketHeader, PCapInterface as RawInterface, SUCCESS, ERRBUF_SIZE};
use dlopen::wrapper::Container;
use super::super::err::Error;
use std::ffi::{CStr, CString};
use libc::{c_char, c_void, c_uint, c_int};
//use core::array::FixedSizeArray;
use std::mem::uninitialized;
use std::ptr::null;



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

#[derive(Debug)]
pub enum Direction {
    In, Out, InOut
}

///Wrapper around pcap error buffer
pub struct PCapErrBuf {
    buffer: [c_char; ERRBUF_SIZE]
}

impl PCapErrBuf{
    pub fn as_string(&self) -> String {
        unsafe{
            CStr::from_ptr(self.buffer.as_ptr())
        }.to_string_lossy().into_owned()
    }

    pub fn buffer(&mut self) -> * mut c_char {
        self.buffer.as_mut_ptr()
    }

    pub fn new () -> PCapErrBuf {
        PCapErrBuf {
            buffer: [0; ERRBUF_SIZE] //TODO: can we optimize it be removing initialization?
        }
    }
}

pub struct Device {
    pub name: String,
    pub description: String,
    pub direction: Direction
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
        if unsafe {self.dll.pcap_sendpacket(self.handle, packet.as_ptr(), packet.len() as c_int)} == SUCCESS {
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
        let mut errbuf =  PCapErrBuf::new();
        let handle = unsafe { self.dll.pcap_open_live(
            name.as_ptr(),
            65536,                  /* max packet size */
            8,                      /* promiscuous mode */
            1000,                   /* read timeout in milliseconds */
            errbuf.buffer()
        )};
        if handle.is_null() {
            Err(Error::OpeningInterface(errbuf.as_string()))
        } else {
            Ok(PCapInterface::new(handle, &self.dll))
        }
    }

    pub fn get_devices(&self) -> Result<Vec<Device>, Error> {
        let mut interf: * const RawInterface = null();
        let mut errbuf = PCapErrBuf::new();
         if unsafe{self.dll.pcap_findalldevs(&interf, errbuf.buffer())} == SUCCESS {
             let mut result: Vec<Device> = Vec::new();
             while !interf.is_null() {
                 result.push(Device{
                     name: unsafe{CStr::from_ptr((*interf).name)}.to_string_lossy().into_owned(),
                     description: unsafe{CStr::from_ptr((*interf).description)}.to_string_lossy().into_owned(),
                     direction: Direction::In
                 });
                 interf = unsafe{&*interf}.next;
             }
             Ok(result)
         } else {
             Err(Error::GettingDeviceList(errbuf.as_string()))
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