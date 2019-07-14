//Different platforms have different locations:

/// Default paths (os-specific) where the pcap dynamically loaded library is stored.
#[cfg(any(target_os = "macos", target_os = "ios"))]
pub const DEFAULT_PATHS: [&'static str; 2] = [
    "libpcap.A.dylib",
    "libpcap.dylib"
];


/// Default paths (os-specific) where the pcap dynamically loaded library is stored.
#[cfg(all(unix, not(any(target_os = "macos", target_os = "ios"))))]
pub const DEFAULT_PATHS: [&'static str; 4] = [
    "libpcap.so",
    "libpcap.so.0.9.5",
    "libpcap.so.0.9.4",
    "libpcap.so.0.8"
];

/// Default paths (os-specific) where the pcap dynamically loaded library is stored.
#[cfg(windows)]
pub const DEFAULT_PATHS: [&'static str; 4] = [
    "NPcap\\Packet.dll",
    "Packet.dll",
    "Npcap\\wpcap.dll",
    "wpcap.dll"
];