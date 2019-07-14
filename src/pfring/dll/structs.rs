use libc::c_long;

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