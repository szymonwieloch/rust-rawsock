/*!
Contains definitions of C-structures and functions.
This is basicly the equivalent of C language header.
*/

use dlopen::wrapper::{Container, WrapperApi};
use libc::{c_char, c_void, c_uint, c_int, c_long};
//use std::ffi::CStr;

pub enum PCapHandle {}

pub const ERRBUF_SIZE: usize = 256; //taken from header, is it platform independent?
pub const SUCCESS: c_int = 0;

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
    ts: PCapTimeVal,
    caplen: c_uint,
    len: c_uint,
    #[cfg(target_os="macos")]
    comment: [c_char; 256]
}

#[repr(C)]
pub struct PCapTimeVal {
    tv_sec: c_long,         /* seconds */
    tv_usec: c_long        /* and microseconds */
}

enum PCapDirection {
    InOut    = 0,
    In       = 1,
    Out      = 2,
}

#[derive(WrapperApi)]
pub struct PCapDll{
    pcap_open_live: unsafe extern "C" fn(devicename: *const c_char, snap_length:c_uint,
        is_promiscuous: c_uint, read_timeout: c_uint, errbuf: * mut c_char) -> * const PCapHandle,
    pcap_close: unsafe extern "C" fn(handle: * const PCapHandle),
    pcap_sendpacket: unsafe extern "C" fn (handle: * const PCapHandle, buf: * const u8, size: c_int) -> c_int,
    pcap_geterr: unsafe extern "C" fn (handle: * const PCapHandle) -> * const c_char,
    pcap_next: unsafe extern "C" fn (handle: * const PCapHandle, header: * mut PCapPacketHeader) -> * const u8,

    //devices
    pcap_findalldevs: unsafe extern "C" fn (alldevsp: * const * const PCapInterface, errbuf: * const c_char) -> c_int,
    pcap_freealldevs: unsafe extern "C" fn (alldevs: * const PCapInterface)
}