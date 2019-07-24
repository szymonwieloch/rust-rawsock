use super::{PFRing, PFRingPacketHeader, PFRingInterface, PFRingStat, PacketDirection, PacketSlicingLevel, SocketMode, ClusterType, PFRingProcessPacket, PFRingChunkInfo, PFRingBpfProgram, ThirdpartyFunc};
use libc::{c_char, c_uint, c_int, c_ushort, timespec, c_uchar, c_double};
use dlopen::wrapper::WrapperApi;
use super::constants::MAX_NUM_RX_CHANNELS;

///Dynamic link library interface of pfring.so
#[derive(WrapperApi)]
pub struct PFRingDll{
    //startup/shutdown
    pfring_open: unsafe extern "C" fn (device_name: * const c_char, caplen: u32, flags: u32) -> * mut PFRing,
    pfring_open_multichannel: unsafe extern "C" fn (device_name: * const c_char, caplen: u32, flags: u32,  output: * mut [* mut PFRing; MAX_NUM_RX_CHANNELS]) -> u8,
    pfring_enable_ring: unsafe extern "C" fn (ring: * const PFRing) -> c_int,
    pfring_disable_ring: unsafe extern "C" fn (ring: * mut PFRing) -> c_int,
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
    pfring_set_filtering_sampling_rate: unsafe extern "C" fn (ring: * mut PFRing, rate : u32) -> c_int,
    pfring_set_direction: unsafe extern "C" fn (ring: * mut PFRing, direction: PacketDirection) -> c_int,
    pfring_set_promisc: unsafe extern "C" fn (ring: * mut PFRing, set_promisc: c_int)-> c_int,
    pfring_set_poll_watermark: unsafe extern "C" fn(ring: * mut PFRing, watermark: u16) -> c_int,
    pfring_set_poll_watermark_timeout: unsafe extern "C" fn (ring: * mut PFRing, poll_watermark_timeout: u16) -> c_int,
    pfring_set_poll_duration: unsafe extern "C" fn (ring: * mut PFRing, duration: c_uint) -> c_int,
    pfring_set_tx_watermark: unsafe extern "C" fn (ring: * mut PFRing, watermark: u16) -> c_int,
    pfring_get_appl_stats_file_name: unsafe extern "C" fn (ring: PFRing, path: * mut c_char, path_len: c_uint) -> c_int,
    pfring_set_packet_slicing: unsafe extern "C" fn(ring: * mut PFRing, packet_level: PacketSlicingLevel, additional_bytes: u32) -> c_int,
    pfring_set_socket_mode: unsafe extern "C" fn (ring: * mut PFRing, mode: SocketMode) -> c_int,
    pfring_set_cluster: unsafe extern "C" fn (ring: * mut PFRing, cluster_idd: c_uint, the_type: ClusterType) -> c_int,
    pfring_remove_from_cluster: unsafe extern "C" fn (ring: * mut PFRing) -> c_int,
    pfring_set_master_id: unsafe extern "C" fn (ring: * mut PFRing, master_id: u32) -> c_int,
    pfring_set_master: unsafe extern "C" fn (ring: * mut PFRing, master: * mut PFRing) -> c_int,
    pfring_remove_hw_rule: unsafe extern "C" fn (ring: * mut PFRing, rule_id: u16) -> c_int,
    pfring_set_reflector_device: unsafe extern "C" fn (ring: * mut PFRing, device_name: * mut c_char) -> c_int,
    pfring_set_bound_dev_name: unsafe extern "C" fn (ring: * mut PFRing, custom_dev_name: * mut c_char) -> c_int,

    // rules
    pfring_remove_filtering_rule: unsafe extern "C" fn (ring: * mut PFRing, rule_id: u16) -> c_int,
    pfring_set_application_stats: unsafe extern "C" fn (ring: * mut PFRing, stats: * mut c_char) -> c_int,
    pfring_purge_idle_hash_rules: unsafe extern "C" fn(ring: * mut PFRing, inactivity_sec: u16)-> c_int,
    pfring_purge_idle_rules: unsafe extern "C" fn (ring: PFRing, inactivity_sec: u16) -> c_int,
    pfring_get_filtering_rule_stats: unsafe extern "C" fn(ring: * mut PFRing, rule_id: u16, stats: * mut char, stats_len: * mut c_uint) -> c_int,
    pfring_toggle_filtering_policy: unsafe extern "C" fn (ring: * mut PFRing, rules_default_accept_policy: u8)-> c_int,
    pfring_remove_bpf_filter: unsafe extern "C" fn (ring: * mut PFRing) -> c_int,

    // the following functions require making pfring-sys compilable
    //int pfring_add_hw_rule(pfring *ring, hw_filtering_rule *rule);
    //int pfring_handle_hash_filtering_rule(pfring *ring, hash_filtering_rule* rule_to_add, u_char add_rule);
    //int pfring_add_filtering_rule(pfring *ring, filtering_rule* rule_to_add);
    //int pfring_get_hash_filtering_rule_stats(pfring *ring, hash_filtering_rule* rule, char *stats, u_int *stats_len);
    //int pfring_set_virtual_device(pfring *ring, virtual_filtering_device_info *info);
    //int pfring_set_filtering_mode(pfring *ring, filtering_mode mode);
    //int pfring_get_card_settings(pfring *ring, pfring_card_settings *settings);



    pfring_stats: unsafe extern "C" fn(ring: * mut PFRing, stats: * mut PFRingStat) -> c_int,
    pfring_get_metadata: unsafe extern "C" fn (ring: * mut PFRing, metadata: * mut * mut u8, metadata_len: * mut u32) -> c_int,

    pfring_get_num_rx_channels: unsafe extern "C" fn(ring: * mut PFRing) -> c_int,
    pfring_get_ring_id: unsafe extern "C" fn (ring: * mut PFRing) -> u32,
    pfring_get_num_queued_pkts: unsafe extern "C" fn (ring: * mut PFRing) -> u32,
    pfring_get_bound_device_address: unsafe extern "C" fn (ring: * mut PFRing, mac_address: * mut c_uchar) -> c_int,
    pfring_get_bound_device_ifindex: unsafe extern "C" fn (ring: * mut PFRing, if_index: * mut c_int) -> c_int,
    pfring_get_device_clock: unsafe extern "C" fn (ring: * mut PFRing, ts: * mut timespec) -> c_int,
    pfring_set_device_clock: unsafe extern "C" fn (ring: * mut PFRing, ts: * mut timespec) -> c_int,
    pfring_adjust_device_clock: unsafe extern "C" fn (ring: * mut PFRing, offset: * mut timespec, sign: i8) -> c_int,
    pfring_get_link_status: unsafe extern "C" fn (ring: * mut PFRing) -> c_int,

    pfring_enable_rss_rehash: unsafe extern "C" fn(ring: * mut PFRing) -> c_int,
    pfring_poll: unsafe extern "C" fn (ring: * mut PFRing, wait_duration: c_uint) -> c_int,
    pfring_is_pkt_available: unsafe extern "C" fn (ring: * mut PFRing) -> c_int,
    pfring_next_pkt_time: unsafe extern "C" fn (ring: * mut PFRing, ts: * mut timespec) -> c_int,
    pfring_get_slot_header_len: unsafe extern "C" fn (ring: * mut PFRing) -> u16,
    pfring_get_device_ifindex: unsafe extern "C" fn (ring: * mut PFRing, device_name: * const c_char, if_index: * mut c_int) -> c_int,
    pfring_get_selectable_fd: unsafe extern "C" fn (ring: * mut PFRing) -> c_int,
    pfring_next_pkt_raw_timestamp: unsafe extern "C" fn (ring: * mut PFRing, timestamp_ns: * mut u64) -> c_int,

    pfring_loopback_test: unsafe extern "C" fn (ring: * mut PFRing, buffer: * mut c_char, buffer_len: c_uint, test_len: c_uint) -> c_int,

    //sending/receiving
    pfring_recv: unsafe extern "C" fn (ring: * const PFRing, buffer: * mut * mut u8, buffer_len: c_uint, hdr: * mut PFRingPacketHeader, wait_for_incoming_packet: u8) -> c_int,
    pfring_recv_parsed: unsafe extern "C" fn (ring: * const PFRing, buffer: * mut * mut u8, buffer_len: c_uint, hdr: * mut PFRingPacketHeader, wait_for_incoming_packet: u8, level: u8 /* 1..4 */, add_timestamp: u8, add_hash: u8) -> c_int,
    pfring_send: unsafe extern "C" fn (ring: * const PFRing, pkt: * const u8, pkt_len: c_uint, flush_packet: u8) -> c_int,
    pfring_send_get_time: unsafe extern "C" fn (ring: * const PFRing, pkt: * const u8, pkt_len: c_uint, ts: * mut timespec) -> c_int,
    pfring_flush_tx_packets: unsafe extern "C" fn (ring: * mut PFRing) -> c_int,
    pfring_loop: unsafe extern "C" fn (ring: * mut PFRing, looper: PFRingProcessPacket, user_bytes: * const c_uchar, wait_for_packet: u8) -> c_int,
    pfring_breakloop: unsafe extern "C" fn (ring: * mut PFRing),
    pfring_send_last_rx_packet: unsafe extern "C" fn(ring: * mut PFRing, tx_interface_id: c_int) -> c_int,
    pfring_recv_chunk: unsafe extern "C" fn (ring: * mut PFRing, chunk: * mut * mut (), chunk_info: * mut PFRingChunkInfo, wait_for_incoming_chunk: u8) -> c_int,




    pfring_version_noring: unsafe extern "C" fn (version: * mut u32),
    pfring_version: unsafe extern "C" fn (ring: * mut PFRing, version: * mut u32) -> c_int,

    pfring_findalldevs: unsafe extern "C" fn() -> * const PFRingInterface,
    pfring_freealldevs: unsafe extern "C" fn(list: * const PFRingInterface),

    pfring_sync_indexes_with_kernel: unsafe extern "C" fn (ring: * mut PFRing),
    pfring_search_payload: unsafe extern "C" fn (ring: * mut PFRing, string_to_search: * mut c_char) -> c_int,

    pfring_parse_pkt: unsafe extern "C" fn (pkt: * mut c_uchar, hdr: * mut PFRingPacketHeader, level : u8, add_timestamp: u8, add_hash: u8) -> c_int,

    pfring_set_if_promisc: unsafe extern "C" fn (device: * const c_char, set_promisc: c_int) -> c_int,

    pfring_format_numbers: unsafe extern "C" fn (val: c_double, buf: * mut c_char, buf_len: c_uint, add_decimals: u8) -> * mut c_char,
    pfring_enable_hw_timestamp: unsafe extern "C" fn (ring: * mut PFRing, device_name: * mut c_char, enable_rx: u8, enable_tx: u8) -> c_int,
    pfring_get_mtu_size: unsafe extern "C" fn (ring: * mut PFRing) -> c_int,

    pfring_print_parsed_pkt: unsafe extern "C" fn (buff: * mut c_char, buff_len: c_uint, p: * const c_uchar, h: * const PFRingPacketHeader) -> c_int,
    pfring_print_pkt: unsafe extern "C" fn (buff: * mut c_char, buff_len: c_uint, p: * const c_uchar, len: c_uint, caplen: c_uint) -> c_int,

    pfring_read_ixia_hw_timestamp: unsafe extern "C" fn (buffer: * mut c_uchar, buffer_len: u32, ts: * mut timespec) -> c_int,
    pfring_handle_ixia_hw_timestamp: unsafe extern "C" fn (buffer: * mut c_uchar, hdr: * mut PFRingPacketHeader),
    pfring_read_vss_apcon_hw_timestamp: unsafe extern "C" fn (buffer: * mut c_uchar, buffer_len: u32, ts: * mut timespec) -> c_int,
    pfring_handle_vss_apcon_hw_timestamp: unsafe extern "C" fn (buffer: * mut c_uchar, hdr: * mut PFRingPacketHeader),
    pfring_get_interface_speed: unsafe extern "C" fn (ring: * mut PFRing) -> u32,
    pfring_parse_bpf_filter: unsafe extern "C" fn (filter_buffer: * mut c_char, caplen: c_uint, filter: * mut PFRingBpfProgram) -> c_int,
    pfring_free_bpf_filter: unsafe extern "C" fn(filter: * mut PFRingBpfProgram),
    pfring_bpf_filter: unsafe extern "C" fn (bpf_insn: * mut (), buffer: * mut c_uchar, caplen: u32, len: u32)-> u32,
    pfring_thirdparty_lib_init: unsafe extern "C" fn (thirdparty_lib_name: * const c_char, thirdparty_function_ptr: * mut ThirdpartyFunc)


}