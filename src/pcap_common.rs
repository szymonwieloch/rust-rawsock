use std::ffi::{CStr};
use libc::{c_char, c_void, c_uint, c_int, c_long};

pub const ERRBUF_SIZE: usize = 256; //taken from header, is it platform independent?
pub enum PCapHandle {}
pub const SUCCESS: c_int = 0;


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

#[repr(C)]
pub struct PCapInterface {
    pub next: * const PCapInterface,
    pub name: * const c_char, /* name to hand to "pcap_open_live()" */
    pub description: * const c_char,	/* textual description of interface, or NULL */
    pub addresses: * const c_void,
    pub flags: c_uint	/* PCAP_IF_ interface flags */
}
#[repr(C)]
pub struct PCapPacketHeader {
    pub ts: PCapTimeVal,
    pub caplen: c_uint,
    pub len: c_uint,
    #[cfg(target_os="macos")]
    pub comment: [c_char; 256]
}

#[repr(C)]
pub struct PCapTimeVal {
    pub tv_sec: c_long,         /* seconds */
    pub tv_usec: c_long        /* and microseconds */
}

pub enum PCapDirection {
    InOut    = 0,
    In       = 1,
    Out      = 2,
}