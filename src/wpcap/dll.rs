/*!
Contains definitions of C-structures and functions.
This is basicly the equivalent of C language header.
*/

use dlopen::wrapper::WrapperApi;
use libc::{c_char, c_uint, c_int};
pub use super::super::pcap_common::{SUCCESS, PCapHandle, PCapPacketHeader, TimeVal, PCapInterface};

/// Code hat helps you use the original DLL API.
pub mod helpers {
    pub use crate::pcap_common::PCapErrBuf;
}

///Equivalent of C struct pcap_sendqueue_t
#[repr(C)]
pub struct PCapSendQueue{
    maxlen: c_uint,
    len: c_uint,
    buffer: * mut c_char
}

///Dynamic load library interface for wpcap.dll
#[derive(WrapperApi)]
pub struct WPCapDll {
    pcap_open_live: unsafe extern "C" fn(devicename: *const c_char, snap_length: c_uint,
                                         is_promiscuous: c_uint, read_timeout: c_uint, errbuf: *mut c_char) -> *const PCapHandle,
    pcap_close: unsafe extern "C" fn(handle: *const PCapHandle),
    pcap_sendpacket: unsafe extern "C" fn(handle: *const PCapHandle, buf: *const u8, size: c_int) -> c_int,
    pcap_geterr: unsafe extern "C" fn(handle: *const PCapHandle) -> *const c_char,
    pcap_next: unsafe extern "C" fn(handle: *const PCapHandle, header: *mut PCapPacketHeader) -> *const u8,
    pcap_datalink: unsafe extern "C" fn(handle: *const PCapHandle) -> c_int,
    //devices
    pcap_findalldevs: unsafe extern "C" fn(alldevsp: *mut *const PCapInterface, errbuf: *const c_char) -> c_int,
    pcap_freealldevs: unsafe extern "C" fn(alldevs: *const PCapInterface),
    pcap_lib_version: unsafe extern "C" fn() -> * const c_char,

    //wpcap specific
    pcap_sendqueue_destroy: unsafe extern "C" fn (queue: * mut PCapSendQueue),
    pcap_sendqueue_alloc: unsafe extern "C" fn (memsize: c_uint) -> * mut PCapSendQueue,
    pcap_sendqueue_queue: unsafe extern "C" fn (queue: * mut PCapSendQueue, pkt_head: * const PCapPacketHeader, pkt_data: * const u8) -> c_int,
    pcap_sendqueue_transmit: unsafe extern "C" fn (p: * const PCapHandle, queue: * mut PCapSendQueue, sync: c_int) -> c_uint,
}