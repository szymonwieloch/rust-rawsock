use libc::{c_long, c_uint, c_char, c_int, time_t};
use super::constants::PFRingChunkType;

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

///Equivalent of the C pfring_card_settings
#[repr(C)]
pub struct PFRingCardSettings{
    pub max_packet_size: u32,
    pub rx_ring_slots: u32,
    pub tx_ring_slots: u32,
}

///Equivalent of the C pfring_stat
#[repr(C)]
pub  struct PFRingStat{
    pub recv: u64,
    pub drop: u64,
    pub shunt: u64
}


///Equivalent of the C pfring_chunk_info
#[repr(C)]
pub struct PFRingChunkInfo{
    pub length: u32,
    pub chunk_type: PFRingChunkType,
}

///Equivalent of the C pfring_bpf_program
#[repr(C)]
pub struct PFRingBpfProgram {
    bf_len: c_uint,
    bf_insns: * const ()
}

/** Bus ID: "%04X:%02X:%02X.%X", slot, bus, device, function */
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct BusId {
    pub slot: c_int,
    pub bus: c_int,
    pub device: c_int,
    pub function: c_int
}

///Equivalent of the C pfring_if_t
#[derive(Debug)]
#[repr(C)]
pub struct PFRingInterface {
    pub name: * const c_char,
    pub system_name: * const c_char,
    pub module: * const c_char,
    pub sn: * const c_char,
    pub mac: [c_char; 6],
    pub bus_id: BusId,
    pub status: c_int, /* 1: up, 0: down*/
    pub license: c_int, /* 1: valid, 0: invalid or not installed */
    pub license_expiration: time_t,
    pub next: * const PFRingInterface
}

///Equivalent of the C pfring_module_info
#[repr(C)]
pub struct PFRingModuleInfo {
    pub name: c_int,
    pub open: unsafe extern "C" fn (* const PFRing) -> c_int,
    pub findalldevs : unsafe extern "C" fn () -> * const PFRingInterface
}


///Equivalent of the C thirdparty_func
#[repr(C)]
pub struct ThirdpartyFunc {
    pub name: * const c_char,  /* Function name */
    pub ptr: unsafe extern "C" fn() -> () /* Function pointer */
}

///Equivalent of the C pfring_stat
#[repr(C)]
pub struct PFRingStats{
    recv: u64,
    drop: u64,
    shunt: u64
}


///Equivalent of the C packet_direction
#[repr(C)]
pub enum PacketDirection {
    RxAndTx = 0,
    RxOnly,
    TxOnly

}




