use libc::c_uint;

///Equivalent of C pcap_stat
#[repr(C)]
pub struct PCapStat {
    pub ps_recv: c_uint,
    pub ps_drop: c_uint,
    pub ps_ifdrop: c_uint,
    pub bs_capt: c_uint
}