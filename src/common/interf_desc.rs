use std::fmt::{Display, Formatter, Error as FmtError};

/// Describes a network interface.
#[derive(Debug, Clone)]
pub struct InterfaceDescription {
    ///Interface name that can be used as an argument for open_interface() function.
    pub name: String,
    /// Human friendly interface description.
    pub description: String
}

impl Display for InterfaceDescription{
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "{}, {}", &self.name, &self.description)
    }
}