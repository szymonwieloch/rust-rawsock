/*!
Contains definitions of C-structures and functions.
This is basicly the equivalent of C language header.
*/

use dlopen::wrapper::WrapperApi;
use libc::{c_char, c_uint, c_int, FILE, c_uchar, c_long};
use super::structs::PCapStat;

/// Code hat helps you use the original DLL API.
pub mod helpers {
    pub use crate::pcap_common::PCapErrBuf;
}

pub use crate::pcap_common::{PCapHandle, PCapPacketHeader, PCapInterface, PCapHandler, BpfProgram, PCapDirection, PCapDumper};
pub use crate::pcap_common::constants::SUCCESS;


///Dynamic link library interface for pcap.so
#[derive(WrapperApi)]
pub struct PCapDll{
    pcap_open_live: unsafe extern "C" fn(devicename: *const c_char, snap_length:c_uint, is_promiscuous: c_uint, read_timeout: c_uint, errbuf: * mut c_char) -> * const PCapHandle,
    pcap_open_offline_with_tstamp_precision: unsafe extern "C" fn (fname: * const c_char, precision: c_uint, errbuf: * mut c_char) -> * const PCapHandle,
    pcap_open_offline: unsafe extern "C" fn (fname: * const c_char, errbuf: * mut c_char) -> * const PCapHandle,
    pcap_fopen_offline: unsafe extern "C" fn (fp: * mut FILE, errbuf: * mut c_char) -> * const PCapHandle,
    pcap_fopen_offline_with_tstamp_precision: unsafe extern "C" fn (fp: * mut FILE, precision: c_uint, errbuf: * mut c_char) -> * const PCapHandle,
    pcap_open_dead: unsafe extern "C" fn (linktype: c_int, snaplen: c_int) -> * const PCapHandle,
    pcap_open_dead_with_tstamp_precision: unsafe extern "C" fn (linktype: c_int, snaplen: c_int, precision: c_uint) -> * const PCapHandle,
    pcap_close: unsafe extern "C" fn(handle: * const PCapHandle),
    pcap_activate: unsafe extern "C" fn (handle: * const PCapHandle) -> c_int,
    pcap_file: unsafe extern "C" fn (p: * const PCapHandle) -> * mut FILE,


    //configuration
    pcap_set_snaplen: unsafe extern "C" fn(p: * const PCapHandle, snaplen: c_int) -> c_int,
    pcap_snapshot: unsafe extern "C" fn (p: * const PCapHandle) -> c_int,
    pcap_set_promisc: unsafe extern "C" fn(p: * const PCapHandle, promisc: c_int) -> c_int,
    pcap_can_set_rfmon: unsafe extern "C" fn (p: * const PCapHandle) -> c_int,
    pcap_set_rfmon: unsafe extern "C" fn (p: * const PCapHandle, rfmon: c_int) -> c_int,
    pcap_set_timeout: unsafe extern "C" fn(p: * const PCapHandle, to_ms: c_int)-> c_int,
    pcap_set_buffer_size: unsafe extern "C" fn (p: * const PCapHandle, buffer_size: c_int) -> c_int,
    pcap_set_tstamp_type: unsafe extern "C" fn (p: * const PCapHandle, tstamp_type: c_int) -> c_int,
    pcap_list_tstamp_types: unsafe extern "C" fn (p: * const PCapHandle, tstamp_typesp: * mut * mut c_int) -> c_int,
    pcap_free_tstamp_types: unsafe extern "C" fn (tstamp_types: * mut c_int),
    pcap_tstamp_type_val_to_name: unsafe extern "C" fn (tstamp_type: c_int) -> * const c_char,
    pcap_tstamp_type_val_to_description: unsafe extern "C" fn (tstamp_type: c_int) -> * const c_char,
    pcap_tstamp_type_name_to_val: unsafe extern "C" fn (name: * const c_char) -> c_int,
    pcap_set_tstamp_precision: unsafe extern "C" fn (p: * const PCapHandle, tstamp_precision: c_int) -> c_int,
    pcap_get_tstamp_precision: unsafe extern "C" fn (p: * const PCapHandle) -> c_int,


    pcap_is_swapped: unsafe extern "C" fn (p: * const PCapHandle) -> c_int,
    pcap_geterr: unsafe extern "C" fn (handle: * const PCapHandle) -> * const c_char,

    // sending/receiving
    pcap_sendpacket: unsafe extern "C" fn (handle: * const PCapHandle, buf: * const u8, size: c_int) -> c_int,
    pcap_inject: unsafe extern "C" fn (handle: * const PCapHandle, buf: * const (), size: isize) -> c_int,
    pcap_next: unsafe extern "C" fn (handle: * const PCapHandle, header: * mut PCapPacketHeader) -> * const u8,
    pcap_next_ex: unsafe extern "C" fn (handle: * const PCapHandle, pkt_header: *mut * mut PCapPacketHeader, pkt_data: * mut * const c_uchar) -> c_int,
    pcap_breakloop: unsafe extern "C" fn (andle: * const PCapHandle),
    pcap_setnonblock: unsafe extern "C" fn (handle: * const PCapHandle, nonblock: c_int, errbuf: * mut c_char) -> c_int,
    pcap_getnonblock: unsafe extern "C" fn (handle: * const PCapHandle, errbuf: * mut c_char) -> c_int,
    pcap_get_selectable_fd: unsafe extern "C" fn (handle: * const PCapHandle) -> c_int,

    pcap_loop: unsafe extern "C" fn (handle: * const PCapHandle, cnt: c_int, callback: PCapHandler, user: * mut c_uchar) -> c_int,
    pcap_dispatch: unsafe extern "C" fn (handle: * const PCapHandle, cnt: c_int, callback: PCapHandler, user: * mut c_uchar) -> c_int,

    //filtering
    pcap_compile: unsafe extern "C" fn (handle: * const PCapHandle, fp: * mut BpfProgram, str: * const c_char, optimize: c_int, netmask: u32) -> c_int,
    pcap_freecode: unsafe extern "C" fn (bpf_program: * mut BpfProgram),
    pcap_setfilter: unsafe extern "C" fn (handle: * const PCapHandle, fp: * mut BpfProgram) -> c_int,
    pcap_lookupnet: unsafe extern "C" fn (device: * const c_char, netp: * mut u32, maskp: * mut u32, errbuf: * mut c_char) -> c_int,
    pcap_offline_filter: unsafe extern "C" fn (fp: * const BpfProgram, h: * const PCapPacketHeader, pkt: * const c_uchar) -> c_int,



    pcap_datalink: unsafe extern "C" fn (handle: * const PCapHandle) -> c_int,
    pcap_setdirection: unsafe extern "C" fn (p: * const PCapHandle, d: PCapDirection) -> c_int,

    //dump
    pcap_dump_open: unsafe extern "C" fn (handle: * const PCapHandle, fname: * const c_char) -> * mut PCapDumper,
    pcap_dump_open_append: unsafe extern "C" fn (handle: * const PCapHandle, fname: * const c_char) -> * mut PCapDumper,
    pcap_dump_fopen: unsafe extern "C" fn (handle: * const PCapHandle, fp: * mut FILE) -> * mut PCapDumper,
    pcap_dump_close: unsafe extern "C" fn (p: * mut PCapDumper),
    pcap_dump_file: unsafe extern "C" fn (p: * mut PCapDumper) -> * mut FILE,
    pcap_dump: unsafe extern "C" fn (user: * mut c_uchar, h: * mut PCapPacketHeader, sp: * mut c_uchar),
    pcap_dump_flush: unsafe extern "C" fn (p: * mut PCapDumper) -> c_int,
    pcap_dump_ftell: unsafe extern "C" fn (p: * mut PCapDumper) -> c_long,

    pcap_lib_version: unsafe extern "C" fn() -> * const c_char,
    pcap_major_version: unsafe extern "C" fn (p: * const PCapHandle) -> c_int,
    pcap_minor_version: unsafe extern "C" fn (p: * const PCapHandle) -> c_int,

    //datalinks
    pcap_list_datalinks: unsafe extern "C" fn (p: * const PCapHandle, dlt_buf: * mut * mut c_int) -> c_int,
    pcap_free_datalinks: unsafe extern "C" fn (dlt_list: * mut c_int),
    pcap_set_datalink: unsafe extern "C" fn (p: * const PCapHandle, dlt: c_int) -> c_int,
    pcap_datalink_val_to_name: unsafe extern "C" fn (dlt: c_int) -> * const c_char,
    pcap_datalink_val_to_description: unsafe extern "C" fn (dlt: c_int)  -> * const c_char,

    pcap_datalink_name_to_val: unsafe extern "C" fn (name: * const c_char) -> c_int,

    pcap_stats: unsafe extern "C" fn (handle: * const PCapHandle, ps: * mut PCapStat) -> c_int,
    pcap_statustostr: unsafe extern "C" fn (error: c_int) -> * const c_char,
    //devices
    pcap_findalldevs: unsafe extern "C" fn (alldevsp: * mut * const PCapInterface, errbuf: * const c_char) -> c_int,
    pcap_freealldevs: unsafe extern "C" fn (alldevs: * const PCapInterface),

    // this is not yet available in the publicly available pcap library
    //pcap_get_required_select_timeout: unsafe extern "C" fn (handle: * const PCapHandle) -> * mut TimeVal,
    //pcap_dump_ftell64: unsafe extern "C" fn (p: * mut PCapDumper) -> i64,
    //pcap_datalink_val_to_description_or_dlt: unsafe extern "C" fn (dlt: c_int)  -> * const c_char,

}