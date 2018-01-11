use super::super::{RawSock, Interface, Packet, Device};
use super::dll::{PCapDll, PCapHandle, PCapPacketHeader, PCapInterface as RawInterface, SUCCESS, ERRBUF_SIZE, PCapDirection as RawDirection};
use dlopen::wrapper::Container;
use super::super::err::Error;
use std::ffi::{CStr, CString};
use libc::{c_char};
use std::mem::uninitialized;
use std::ptr::null;
use super::interface::PCapInterface;
use super::dev_iter::PCapDeviceIterator;



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




impl<'a> RawSock<'a> for PCap {
    type Interf = PCapInterface<'a>;
    type DeviceIterator = PCapDeviceIterator<'a>;
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

    fn get_devices(&'a self) -> Result<Self::DeviceIterator, Error> {
        PCapDeviceIterator::new(&self.dll)
    }
}