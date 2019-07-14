use std::fmt::{Display, Formatter, Error as FmtError};

/// Describes a netowrk interface.
#[derive(Debug, Clone)]
pub struct InterfaceDescription {
    pub name: String,
    pub description: String
}

impl Display for InterfaceDescription{
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "{}, {}", &self.name, &self.description)
    }
}