//Different platforms have different locations:

#[cfg(any(target_os = "macos", target_os = "ios"))]
pub const DEFAULT_PATHS: [&'static str; 2] = [
    "libpcap.A.dylib",
    "libpcap.dylib"
];

#[cfg(all(unix, not(any(target_os = "macos", target_os = "ios"))))]
pub const DEFAULT_PATHS: [&'static str; 4] = [
    "libpcap.so",
    "libpcap.so.0.9.5",
    "libpcap.so.0.9.4",
    "libpcap.so.0.8"
];

#[cfg(windows)]
pub const DEFAULT_PATHS: [&'static str; 4] = [
    "NPcap\\Packet.dll",
    "Packet.dll",
    "Npcap\\wpcap.dll",
    "wpcap.dll"
];