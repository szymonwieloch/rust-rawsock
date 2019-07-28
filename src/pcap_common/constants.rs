#![allow(dead_code)]
use libc::c_int;

pub const ERRBUF_SIZE: usize = 256; //taken from header, is it platform independent?

/// Value returned by pcap library to indicate successful operation.
pub const SUCCESS: c_int = 0;
pub const PCAP_ERROR: c_int = -1;
pub const PCAP_ERROR_BREAK: c_int = -2;
pub const PCAP_ERROR_NOT_ACTIVATED: c_int = -3;
pub const PCAP_ERROR_ACTIVATED: c_int = -4;
pub const PCAP_ERROR_NO_SUCH_DEVICE: c_int = -5;
pub const PCAP_ERROR_RFMON_NOTSUP: c_int = -6;
pub const PCAP_ERROR_NOT_RFMON: c_int = -7;
pub const PCAP_ERROR_PERM_DENIED: c_int = -8;
pub const PCAP_ERROR_IFACE_NOT_UP: c_int = -9;
pub const PCAP_ERROR_CANTSET_TSTAMP_TYPE: c_int = -10;
pub const PCAP_ERROR_PROMISC_PERM_DENIED: c_int = -11;
pub const PCAP_ERROR_TSTAMP_PRECISION_NOTSUP: c_int = -12;

