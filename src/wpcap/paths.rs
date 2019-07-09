#[cfg(windows)]
pub const DEFAULT_PATHS: [&'static str; 4] = [
    "NPcap\\Packet.dll",
    "Packet.dll",
    "Npcap\\wpcap.dll",
    "wpcap.dll"
];

#[cfg(not(windows))]
pub const DEFAULT_PATHS: [&'static str; 0] = [];