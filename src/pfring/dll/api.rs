use super::{PFRing, PFRingPacketHeader};
use libc::{c_char, c_uint, c_int};
use dlopen::wrapper::WrapperApi;

///Dynamic link library interface of pfring.so
#[derive(WrapperApi)]
pub struct PFRingDll{
    pfring_recv: unsafe extern "C" fn (ring: * const PFRing, buffer: * mut * mut u8, buffer_len: c_uint, hdr: * mut PFRingPacketHeader, wait_for_incoming_packet: u8) -> c_int,
    pfring_send: unsafe extern "C" fn (ring: * const PFRing, pkt: * const u8, pkt_len: c_uint, flush_packet: u8) -> c_int,
    pfring_close: unsafe extern "C" fn (ring: * mut PFRing),
    pfring_open: unsafe extern "C" fn (device_name: * const c_char, caplen: u32, flags: u32) -> * mut PFRing,
    pfring_flush_tx_packets: unsafe extern "C" fn (ring: * mut PFRing) -> c_int,
    pfring_version_noring: unsafe extern "C" fn (version: * mut u32),
    pfring_enable_ring: unsafe extern "C" fn (ring: * const PFRing) -> c_int
}