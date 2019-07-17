/*!
Code for managing the *pcap* library.
*/

mod library;
pub mod dll;
mod interface;
mod structs;
mod paths;


pub use self::paths::DEFAULT_PATHS;
pub use self::library::Library;
pub use self::interface::Interface;