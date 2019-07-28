/*!
Contains definitions of C-structures and functions.
This is basicly the equivalent of C language header.
*/

use dlopen::wrapper::WrapperApi;
use libc::{c_char, c_uint, c_int, c_uchar, c_long, FILE};
pub use crate::pcap_common::{PCapHandle, PCapPacketHeader, TimeVal, PCapInterface, PCapDumper, PCapHandler, BpfProgram};
pub use crate::pcap_common::constants::SUCCESS;
pub use super::structs::PCapStat;

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
    pcap_open_live: unsafe extern "C" fn(devicename: *const c_char, snap_length: c_uint, is_promiscuous: c_uint, read_timeout: c_uint, errbuf: *mut c_char) -> *const PCapHandle,
    pcap_open_offline: unsafe extern "C" fn (fname: * const c_char, errbuf: * mut c_char) -> * const PCapHandle,
    pcap_open_dead: unsafe extern "C" fn (linktype: c_int, snaplen: c_int) -> * const PCapHandle,
    pcap_dump_open: unsafe extern "C" fn (handle: * const PCapHandle, fname: * const c_char) -> * mut PCapDumper,
    pcap_close: unsafe extern "C" fn(handle: *const PCapHandle),


    pcap_setnonblock: unsafe extern "C" fn (handle: * const PCapHandle, nonblock: c_int, errbuf: * mut c_char) -> c_int,
    pcap_getnonblock: unsafe extern "C" fn (handle: * const PCapHandle, errbuf: * mut c_char) -> c_int,
    pcap_lookupnet: unsafe extern "C" fn (device: * const c_char, netp: * mut u32, maskp: * mut u32, errbuf: * mut c_char) -> c_int,
    pcap_sendpacket: unsafe extern "C" fn(handle: *const PCapHandle, buf: *const u8, size: c_int) -> c_int,
    pcap_geterr: unsafe extern "C" fn(handle: *const PCapHandle) -> *const c_char,
    pcap_next: unsafe extern "C" fn(handle: *const PCapHandle, header: *mut PCapPacketHeader) -> *const u8,
    pcap_next_ex: unsafe extern "C" fn (handle: * const PCapHandle, pkt_header: *mut * mut PCapPacketHeader, pkt_data: * mut * const c_uchar) -> c_int,
    pcap_datalink: unsafe extern "C" fn(handle: *const PCapHandle) -> c_int,
    pcap_dump: unsafe extern "C" fn (user: * mut c_uchar, h: * mut PCapPacketHeader, sp: * mut c_uchar),
    pcap_dump_ftell: unsafe extern "C" fn (p: * mut PCapDumper) -> c_long,
    //devices
    pcap_findalldevs: unsafe extern "C" fn(alldevsp: *mut *const PCapInterface, errbuf: *const c_char) -> c_int,
    pcap_freealldevs: unsafe extern "C" fn(alldevs: *const PCapInterface),
    pcap_lib_version: unsafe extern "C" fn() -> * const c_char,
    pcap_compile: unsafe extern "C" fn (handle: * const PCapHandle, fp: * mut BpfProgram, str: * const c_char, optimize: c_int, netmask: u32) -> c_int,
    pcap_setfilter: unsafe extern "C" fn (handle: * const PCapHandle, fp: * mut BpfProgram) -> c_int,
    pcap_freecode: unsafe extern "C" fn (bpf_program: * mut BpfProgram),

    pcap_list_datalinks: unsafe extern "C" fn (p: * const PCapHandle, dlt_buf: * mut * mut c_int) -> c_int,
    pcap_free_datalinks: unsafe extern "C" fn (dlt_list: * mut c_int),
    pcap_set_datalink: unsafe extern "C" fn (p: * const PCapHandle, dlt: c_int) -> c_int,
    pcap_datalink_val_to_name: unsafe extern "C" fn (dlt: c_int) -> * const c_char,
    pcap_datalink_val_to_description: unsafe extern "C" fn (dlt: c_int)  -> * const c_char,
    pcap_snapshot: unsafe extern "C" fn (p: * const PCapHandle) -> c_int,
    pcap_is_swapped: unsafe extern "C" fn (p: * const PCapHandle) -> c_int,

    pcap_major_version: unsafe extern "C" fn (p: * const PCapHandle) -> c_int,
    pcap_minor_version: unsafe extern "C" fn (p: * const PCapHandle) -> c_int,

    pcap_dump_file: unsafe extern "C" fn (p: * mut PCapDumper) -> * mut FILE,
    pcap_dump_flush: unsafe extern "C" fn (p: * mut PCapDumper) -> c_int,
    pcap_dump_close: unsafe extern "C" fn (p: * mut PCapDumper),


    pcap_stats: unsafe extern "C" fn (handle: * const PCapHandle, ps: * mut PCapStat) -> c_int,

    pcap_loop: unsafe extern "C" fn (handle: * const PCapHandle, cnt: c_int, callback: PCapHandler, user: * mut c_uchar) -> c_int,
    pcap_breakloop: unsafe extern "C" fn (andle: * const PCapHandle),
    pcap_dispatch: unsafe extern "C" fn (handle: * const PCapHandle, cnt: c_int, callback: PCapHandler, user: * mut c_uchar) -> c_int,
    pcap_file: unsafe extern "C" fn (p: * const PCapHandle) -> * mut FILE,

    //wpcap specific
    pcap_sendqueue_destroy: unsafe extern "C" fn (queue: * mut PCapSendQueue),
    pcap_sendqueue_alloc: unsafe extern "C" fn (memsize: c_uint) -> * mut PCapSendQueue,
    pcap_sendqueue_queue: unsafe extern "C" fn (queue: * mut PCapSendQueue, pkt_head: * const PCapPacketHeader, pkt_data: * const u8) -> c_int,
    pcap_sendqueue_transmit: unsafe extern "C" fn (p: * const PCapHandle, queue: * mut PCapSendQueue, sync: c_int) -> c_uint,
}