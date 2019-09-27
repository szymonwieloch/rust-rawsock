use std::net::SocketAddr;
use std::fmt::{Display, Formatter, Error as FmtError};

/// Describes the addresses of a network interface.
#[derive(Debug, Clone)]
pub struct AddressDescription {
    pub address: Option<SocketAddr>,
    pub netmask: Option<SocketAddr>,
    pub broadcase_address: Option<SocketAddr>,
    pub dest_address: Option<SocketAddr>,
}

/// Describes a network interface.
#[derive(Debug, Clone)]
pub struct InterfaceDescription {
    ///Interface name that can be used as an argument for open_interface() function.
    pub name: String,
    /// Human friendly interface description.
    pub description: String,
    /// Network addresses of the interface.
    /// *Only provided in pcap.*
    pub addresses: Option<Vec<AddressDescription>>
}

impl Display for InterfaceDescription{
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "{}, {}", &self.name, &self.description)
    }
}