/*!
Contains definitions of C-structures and functions.
This is basicly the equivalent of C language header.
*/

use dlopen::wrapper::WrapperApi;
use libc::{c_char, c_uint, c_int};

/// Code hat helps you use the original DLL API.
pub mod helpers {
    pub use crate::pcap_common::PCapErrBuf;
}

pub use crate::pcap_common::{SUCCESS, PCapHandle, PCapPacketHeader, TimeVal, PCapInterface};

///Dynamic link library interface for pcap.so
#[derive(WrapperApi)]
pub struct PCapDll{
    pcap_open_live: unsafe extern "C" fn(devicename: *const c_char, snap_length:c_uint,
        is_promiscuous: c_uint, read_timeout: c_uint, errbuf: * mut c_char) -> * const PCapHandle,
    pcap_close: unsafe extern "C" fn(handle: * const PCapHandle),
    pcap_sendpacket: unsafe extern "C" fn (handle: * const PCapHandle, buf: * const u8, size: c_int) -> c_int,
    pcap_geterr: unsafe extern "C" fn (handle: * const PCapHandle) -> * const c_char,
    pcap_next: unsafe extern "C" fn (handle: * const PCapHandle, header: * mut PCapPacketHeader) -> * const u8,
    pcap_datalink: unsafe extern "C" fn (handle: * const PCapHandle) -> c_int,
    pcap_lib_version: unsafe extern "C" fn() -> * const c_char,
    //devices
    pcap_findalldevs: unsafe extern "C" fn (alldevsp: * mut * const PCapInterface, errbuf: * const c_char) -> c_int,
    pcap_freealldevs: unsafe extern "C" fn (alldevs: * const PCapInterface),

}