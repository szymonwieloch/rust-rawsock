use std::fmt::{Display, Formatter, Error as FmtError};

///Kind of data link - protocol used below the surface.
#[derive(Debug, Copy, Clone)]
pub enum DataLink{
    Ethernet,
    RawIp,
    Other
}

impl Display for DataLink{
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            DataLink::Ethernet => write!(f, "ethernet"),
            DataLink::RawIp => write!(f, "raw IP"),
            DataLink::Other => write!(f, "other")
        }
    }
}