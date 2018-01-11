use super::super::{RawSock, Interface, Packet, Device};
use super::dll::{PCapDll, PCapHandle, PCapPacketHeader, PCapInterface as RawInterface, SUCCESS, ERRBUF_SIZE, PCapDirection as RawDirection};
use dlopen::wrapper::Container;
use super::super::err::Error;
use std::ffi::{CStr, CString};
use libc::{c_char, c_void, c_uint, c_int};
//use core::array::FixedSizeArray;
use std::mem::uninitialized;
use std::ptr::null;
use std::marker::PhantomData;
use std::slice::from_raw_parts;
use time::Timespec;
use super::interface::PCapInterface;



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

impl PCap {
    pub fn get_devices(&self) -> Result<Vec<Device>, Error> {
        let mut interf: * const RawInterface = null();
        let mut errbuf = PCapErrBuf::new();
         if unsafe{self.dll.pcap_findalldevs(&interf, errbuf.buffer())} == SUCCESS {
             let mut result: Vec<Device> = Vec::new();
             while !interf.is_null() {
                 result.push(Device{
                     name: unsafe{CStr::from_ptr((*interf).name)}.to_string_lossy().into_owned(),
                     description: unsafe{CStr::from_ptr((*interf).description)}.to_string_lossy().into_owned()
                 });
                 interf = unsafe{&*interf}.next;
             }
             Ok(result)
         } else {
             Err(Error::GettingDeviceList(errbuf.as_string()))
         }
    }
}

impl<'a> RawSock<'a, PCapInterface<'a>> for PCap {
    fn default_locations() -> &'static [&'static str] {
        &POSSIBLE_NAMES
    }
    fn open(path: &str) -> Result<Self, Error> {
        let dll: Container<PCapDll> = unsafe { Container::load(path)}?;
        Ok(Self {
            dll: dll
        })
    }

    fn open_interface(&'a self, name: & str) -> Result<PCapInterface<'a>, Error>{
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
}