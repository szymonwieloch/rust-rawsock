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
        const DO_NOT_STRIP_FCS          = 1 << 21;
    }
}

pub const MAX_CAPLEN: c_int = 65535;
pub const PAGE_SIZE: c_int = 4096;
pub const DEFAULT_POLL_DURATION: c_int = 500;
pub const POLL_SLEEP_STEP: c_int = 10; /* ns = 0.1 ms */
pub const POLL_SLEEP_MIN: c_int = POLL_SLEEP_STEP;
pub const POLL_SLEEP_MAX: c_int = 1000; /* ns */
pub const POLL_QUEUE_MIN_LEN:c_int = 500; /* # packets */
pub const MAX_NUM_RX_CHANNELS: usize = 64;

pub const SUCCESS: c_int = 0;

/// Equivalent of enum filtering_mode
#[repr(C)]
pub enum FilteringMode{
    HardwareAndSoftware = 0,
    HardwareOnly,
    SoftwareOnly
}

/// Equivalent of enum pfring_chunk_type
#[repr(C)]
pub enum PFRingChunkType {
    PcapChunk,
    PcapNsecChunk,
    UnknownChunkType
}


/// Equivalent of enum packet_slicing_level
#[repr(C)]
pub enum PacketSlicingLevel {
    FullPacketSlicing = 0,
    L2Slicing = 2,
    L3Slicing = 3,
    L4Slicing = 4
}

/// Equivalent of enum socket_mode
#[repr(C)]
pub enum SocketMode {
    SendAndRecv = 0,
    SendOnly,
    RecvOnly,
}

/// Equivalent of enum cluster_type
#[repr(C)]
pub enum ClusterType {
    PerFlow = 0,              /* 6-tuple: <src ip, src port, dst ip, dst port, proto, vlan>  */
    RoundRobin,
    PerFlow2Tuple,          /* 2-tuple: <src ip,           dst ip                       >  */
    PerFlow4Tuple,          /* 4-tuple: <src ip, src port, dst ip, dst port             >  */
    PerFlow5Tuple,          /* 5-tuple: <src ip, src port, dst ip, dst port, proto      >  */
    PerFlowTcp5Tuple,      /* 5-tuple only with TCP, 2 tuple with all other protos        */
    /* same as above, computing on tunnel content when present */
    PerInnerFlow,            /* 6-tuple: <src ip, src port, dst ip, dst port, proto, vlan>  */
    PerInnerFlow2Tuple,    /* 2-tuple: <src ip,           dst ip                       >  */
    PerInerFlow4Tuple,    /* 4-tuple: <src ip, src port, dst ip, dst port             >  */
    PerInnerFlow5Tuple,    /* 5-tuple: <src ip, src port, dst ip, dst port, proto      >  */
    PerInnerFlowTcp5Tuple,/* 5-tuple only with TCP, 2 tuple with all other protos        */
    /* new types, for L2-only protocols */
    PerFlowIp5Tuple,       /* 5-tuple only with IP, 2 tuple with non-IP <src mac, dst mac> */
    PerInnerFlowIp5Tuple, /* 5-tuple only with IP, 2 tuple with non-IP <src mac, dst mac> */
    PerFlowIpWithDupTuple,    /* 1-tuple: <src ip> and <dst ip> with duplication              */
}