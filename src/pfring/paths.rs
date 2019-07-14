/// Default paths (os-specific) where the pfring dynamically loaded library is stored.
#[cfg(all(unix, not(any(target_os = "macos", target_os = "ios"))))]
pub const DEFAULT_PATHS: [&'static str; 2] = [
    "libpfring.so",
    "libpfring.so.1"
];

/// Default paths (os-specific) where the pfring dynamically loaded library is stored.
#[cfg(any(windows, target_os = "macos", target_os = "ios"))]
pub const DEFAULT_PATHS: [&'static str; 0] = [];