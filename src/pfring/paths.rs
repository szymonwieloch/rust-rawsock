#[cfg(all(unix, not(any(target_os = "macos", target_os = "ios"))))]
pub const DEFAULT_PATHS: [&'static str; 2] = [
    "libpfring.so",
    "libpfring.so.1"
];

#[cfg(any(windows, target_os = "macos", target_os = "ios"))]
pub const DEFAULT_PATHS: [&'static str; 0] = [];