use dlopen::wrapper::WrapperApi;
use libc::{c_char, c_uint, c_int, c_long};

///Raw PF Ring handle - created only to allow construction of pointers.
pub enum PFRing{}

///Equivalent of the C struct timeval_t
#[repr(C)]
pub struct TimeVal {
    pub tv_sec: c_long,
    pub tv_usec: c_long
}

///Equivalent of the C struct pcap_hdr_t
#[repr(C)]
pub struct PFRingPacketHeader {
    pub ts: TimeVal,
    pub caplen: u32,
    pub len: u32,
    /* only filled in if PF_RING_LONG_HEADER set */
    extended_hdr: [u8; 512]
}

///Dynami link library interface of pfring.so
#[derive(WrapperApi)]
pub struct PFRingDll{
    pfring_recv: unsafe extern "C" fn (ring: * const PFRing, buffer: * mut * mut u8, buffer_len: c_uint, hdr: * mut PFRingPacketHeader, wait_for_incoming_packet: u8) -> c_int,
    pfring_send: unsafe extern "C" fn (ring: * const PFRing, pkt: * const u8, pkt_len: c_uint, flush_packet: u8) -> c_int,
    pfring_close: unsafe extern "C" fn (ring: * mut PFRing),
 	pfring_open: unsafe extern "C" fn (device_name: * const c_char, caplen: u32, flags: u32) -> * mut PFRing,
    pfring_flush_tx_packets: unsafe extern "C" fn (ring: * mut PFRing) -> c_int
}