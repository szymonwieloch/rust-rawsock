use dlopen::wrapper::{Container, WrapperApi};
use libc::{c_char, c_void, c_uint, c_int, c_long};

pub enum PCapHandle {}

#[repr(C)]
pub struct PCapInterface {
    next: * const PCapInterface,
    name: * const c_char, /* name to hand to "pcap_open_live()" */
    description: * const c_char,	/* textual description of interface, or NULL */
    addresses: * const c_void,
    flags: c_uint	/* PCAP_IF_ interface flags */
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

#[derive(WrapperApi)]
pub struct PCapDll{
    pcap_open_live: unsafe extern "C" fn(devicename: *const c_char, snap_length:c_uint,
        is_promiscuous: c_uint, read_timeout: c_uint, errbuf: * mut c_char) -> * const PCapHandle,
    pcap_close: unsafe extern "C" fn(handle: * const PCapHandle),
    pcap_sendpacket: unsafe extern "C" fn (handle: * const PCapHandle, buf: * const u8, size: c_int) -> c_int,
    pcap_geterr: unsafe extern "C" fn (handle: * const PCapHandle) -> * const c_char,
    pcap_next: unsafe extern "C" fn (handle: * const PCapHandle, header: * mut PCapPacketHeader) -> * const u8,
}