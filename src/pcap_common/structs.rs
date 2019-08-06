use std::ffi::{CStr};
use libc::{c_char, c_void, c_uint, c_uchar, c_ushort, timeval};
use std::mem::uninitialized;
use crate::common::InterfaceDescription;
use crate::utils::cstr_to_string;
use super::constants::ERRBUF_SIZE;


///Raw PCap handle - created only to allow construction of pointers.
pub enum PCapHandle {}

///Raw PCap duper handle - created only to allow construction of pointers.
pub enum PCapDumper {}


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
#[derive(Copy, Clone)]
pub struct PCapPacketHeader {
    pub ts: timeval,
    pub caplen: c_uint,
    pub len: c_uint,
    /*
    Although the official documentation does not mention this, pcap headers on Apple devices
    have additional field and without this definition there is stack corruption in some cases.
    */
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    pub comment: [c_char; 256]
}

///Equivalent of pcap_direction_t
/// #[repr(u32)]
pub enum PCapDirection {
    InOut    = 0,
    In       = 1,
    Out      = 2,
}

pub type PCapHandler=extern "C" fn(user: * mut c_uchar, h: * const PCapPacketHeader, bytes: * const c_uchar);

pub fn interface_data_from_pcap_list(interfs: * const PCapInterface) -> Vec<InterfaceDescription> {
    let mut interfs_descr = Vec::new();
    let mut curr = interfs;
    while !curr.is_null() {
        let id = InterfaceDescription {
            name: cstr_to_string(unsafe{(*curr).name}),
            description: cstr_to_string(unsafe{(*curr).description})
        };
        interfs_descr.push(id);
        curr = unsafe{(*curr).next};
    }
    interfs_descr
}

/// Equivalent of the bpf_insn C struct
pub struct BpfInsn {
    pub code: c_ushort,
    pub jt: c_uchar,
    pub jf: c_uchar,
    pub k: u32,
}

///Equivalent of the bpf_program C struct
pub struct BpfProgram {
    pub bf_len: c_uint,
    pub bf_insns: *mut BpfInsn,
}