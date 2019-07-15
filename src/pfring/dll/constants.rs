use libc::c_int;

/// Equivalent of pfring macros with constants.
bitflags! {
    pub struct PFRingFlags: c_int {
        const ZC_SYMMETRIC_RSS          = 1 << 0;
        const DNA_SYMMETRIC_RSS         = Self::ZC_SYMMETRIC_RSS.bits;
        const REENTRANT                 = 1 << 1;
        const LONG_HEADER               = 1 << 2;
        const PROMISC                   = 1 << 3;
        const TIMESTAMP                 = 1 << 4;
        const HW_TIMESTAMP              = 1 << 5;
        const RX_PACKET_BOUNCE          = 1 << 6;
        const ZC_FIXED_RSS_Q_0          = 1 << 7;
        const DNA_FIXED_RSS_Q_0         = Self::ZC_FIXED_RSS_Q_0.bits;
        const STRIP_HW_TIMESTAMP        = 1 << 8;
        const DO_NOT_PARSE              = 1 << 9;
        const DO_NOT_TIMESTAMP          = 1 << 10;
        const CHUNK_MODE                = 1 << 11;
        const IXIA_TIMESTAMP            = 1 << 12;
        const USERSPACE_BPF             = 1 << 13;
        const ZC_NOT_REPROGRAM_RSS      = 1 << 14;
        const VSS_APCON_TIMESTAMP       = 1 << 15;
        const ZC_IPONLY_RSS             = 1 << 16;
        const FLOW_OFFLOAD              = 1 << 17;
        const FLOW_OFFLOAD_NOUPDATES    = 1 << 18;
        const FLOW_OFFLOAD_NORAWDATA    = 1 << 19;
        const L7_FILTERING              = 1 << 20;
    }
}

pub const MAX_CAPLEN: c_int = 65535;
pub const PAGE_SIZE: c_int = 4096;
pub const DEFAULT_POLL_DURATION: c_int = 500;
pub const POLL_SLEEP_STEP: c_int = 10; /* ns = 0.1 ms */
pub const POLL_SLEEP_MIN: c_int = POLL_SLEEP_STEP;
pub const POLL_SLEEP_MAX: c_int = 1000; /* ns */
pub const POLL_QUEUE_MIN_LEN:c_int = 500; /* # packets */

/// Equivalent of enum filtering_mode
#[repr(C)]
pub enum FilteringMode{
    HardwareAndSoftware = 0,
    HardwareOnly,
    SoftwareOnly
}

/// Equivalent of enum packet_slicing_level
#[repr(C)]
pub enum PacketSlicingLevel {
    FullPacketSlicing = 0,
    L2Slicing,
    L3Slicing,
    L4Slicing,
}

/// Equivalent of enum pfring_chunk_type
#[repr(C)]
pub enum PFRingChunkType {
    PcapChunk,
    PcapNsecChunk,
    UnknownChunkType
}