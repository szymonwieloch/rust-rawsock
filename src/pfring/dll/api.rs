use super::{PFRing, PFRingPacketHeader, PFRingInterface, PFRingStat, PacketDirection};
use libc::{c_char, c_uint, c_int, c_ushort, timespec};
use dlopen::wrapper::WrapperApi;
use super::constants::MAX_NUM_RX_CHANNELS;

///Dynamic link library interface of pfring.so
#[derive(WrapperApi)]
pub struct PFRingDll{
    //startup/shutdown
    pfring_open: unsafe extern "C" fn (device_name: * const c_char, caplen: u32, flags: u32) -> * mut PFRing,
    pfring_open_multichannel: unsafe extern "C" fn (device_name: * const c_char, caplen: u32, flags: u32,  output: * mut [* mut PFRing; MAX_NUM_RX_CHANNELS]) -> u8,
    pfring_enable_ring: unsafe extern "C" fn (ring: * const PFRing) -> c_int,
    pfring_shutdown: unsafe extern "C" fn(ring: * mut PFRing),
    pfring_close: unsafe extern "C" fn (ring: * mut PFRing),
    pfring_bind: unsafe extern "C" fn (ring: * mut PFRing, device_name: * const c_char) -> c_int,

    //configuration
    pfring_config: unsafe extern "C" fn(cpu_percentage: c_ushort),
    pfring_set_application_name: unsafe extern "C" fn(ring: * mut PFRing, name: * const c_char) -> c_int,
    pfring_set_channel_id: unsafe extern "C" fn (ring: * mut PFRing,  channel_id: u32) -> c_int,
    pfring_set_channel_mask: unsafe extern "C" fn (ring: * mut PFRing, channel_mask: u64) -> c_int,
    pfring_set_vlan_id: unsafe extern "C" fn (ring: * mut PFRing, vlan_id: u16) -> c_int,
    pfring_set_sampling_rate: unsafe extern "C" fn (ring: * mut PFRing, rate: u32) -> c_int,
    pfring_set_direction: unsafe extern "C" fn (ring: * mut PFRing, direction: PacketDirection) -> c_int,
    pfring_set_promisc: unsafe extern "C" fn (ring: * mut PFRing, set_promisc: c_int)-> c_int,


    pfring_stats: unsafe extern "C" fn(ring: * mut PFRing, stats: * mut PFRingStat) -> c_int,
    pfring_get_metadata: unsafe extern "C" fn (ring: * mut PFRing, metadata: * mut * mut u8, metadata_len: * mut u32) -> c_int,

    //int pfring_loop(pfring *ring, pfringProcesssPacket looper, const u_char *user_bytes, u_int8_t wait_for_packet);
    pfring_breakloop: unsafe extern "C" fn (ring: * mut PFRing),
    pfring_get_num_rx_channels: unsafe extern "C" fn(ring: * mut PFRing) -> c_int,
    pfring_get_ring_id: unsafe extern "C" fn (ring: * mut PFRing) -> u32,

    pfring_enable_rss_rehash: unsafe extern "C" fn(ring: * mut PFRing) -> c_int,
    pfring_poll: unsafe extern "C" fn (ring: * mut PFRing, wait_duration: c_uint) -> c_int,
    pfring_is_pkt_available: unsafe extern "C" fn (ring: * mut PFRing) -> c_int,
    pfring_next_pkt_time: unsafe extern "C" fn (ring: * mut PFRing, ts: * mut timespec) -> c_int,
    pfring_get_slot_header_len: unsafe extern "C" fn (ring: * mut PFRing) -> u16,
    pfring_get_device_ifindex: unsafe extern "C" fn (ring: * mut PFRing, device_name: * const c_char, if_index: * mut c_int) -> c_int,

    //sending/receiving
    pfring_recv: unsafe extern "C" fn (ring: * const PFRing, buffer: * mut * mut u8, buffer_len: c_uint, hdr: * mut PFRingPacketHeader, wait_for_incoming_packet: u8) -> c_int,
    pfring_recv_parsed: unsafe extern "C" fn (ring: * const PFRing, buffer: * mut * mut u8, buffer_len: c_uint, hdr: * mut PFRingPacketHeader, wait_for_incoming_packet: u8, level: u8 /* 1..4 */, add_timestamp: u8, add_hash: u8) -> c_int,
    pfring_send: unsafe extern "C" fn (ring: * const PFRing, pkt: * const u8, pkt_len: c_uint, flush_packet: u8) -> c_int,
    pfring_send_get_time: unsafe extern "C" fn (ring: * const PFRing, pkt: * const u8, pkt_len: c_uint, ts: * mut timespec) -> c_int,
    pfring_flush_tx_packets: unsafe extern "C" fn (ring: * mut PFRing) -> c_int,


    pfring_version_noring: unsafe extern "C" fn (version: * mut u32),
    pfring_version: unsafe extern "C" fn (ring: * mut PFRing, version: * mut u32) -> c_int,

    pfring_findalldevs: unsafe extern "C" fn() -> * const PFRingInterface,
    pfring_freealldevs: unsafe extern "C" fn(list: * const PFRingInterface)

}