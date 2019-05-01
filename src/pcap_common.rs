use std::ffi::{CStr};
use libc::{c_char, c_void, c_uint, c_int, c_long};
use std::mem::uninitialized;

pub const ERRBUF_SIZE: usize = 256; //taken from header, is it platform independent?
pub enum PCapHandle {}
pub const SUCCESS: c_int = 0;


///Wrapper around pcap error buffer
pub struct PCapErrBuf {
    buffer: [c_char; ERRBUF_SIZE]
}

///Wrapper over unsafe pcap error buffer
impl PCapErrBuf{
    ///Converts current content to a string
    pub fn as_string(&self) -> String {
        unsafe{
            CStr::from_ptr(self.buffer.as_ptr())
        }.to_string_lossy().into_owned()
    }

    ///Returns pointer to the underlying buffer.
    pub fn buffer(&mut self) -> * mut c_char {
        self.buffer.as_mut_ptr()
    }

    ///Creates a new instance.
    pub fn new () -> PCapErrBuf {
        PCapErrBuf {
            buffer: unsafe{uninitialized()}
        }
    }
}

///Equivalent of pcap_interf_t
#[repr(C)]
pub struct PCapInterface {
    pub next: * const PCapInterface,
    pub name: * const c_char, /* name to hand to "pcap_open_live()" */
    pub description: * const c_char,	/* textual description of interface, or NULL */
    pub addresses: * const c_void,
    pub flags: c_uint	/* PCAP_IF_ interface flags */
}

///Equivalent of C struct pcap_pkthdr
#[repr(C)]
pub struct PCapPacketHeader {
    pub ts: PCapTimeVal,
    pub caplen: c_uint,
    pub len: c_uint,
    //some documentation suggest that this additional field should be present on MAC os.
    //but the main documentation seems to state something different.
    //#[cfg(target_os="macos")]
    //pub comment: [c_char; 256]
}

///Equivalent of C struct timeval_t
#[repr(C)]
pub struct PCapTimeVal {
    pub tv_sec: c_long,         /* seconds */
    pub tv_usec: c_long        /* and microseconds */
}
/* probably this his not needed
pub enum PCapDirection {
    InOut    = 0,
    In       = 1,
    Out      = 2,
}*/