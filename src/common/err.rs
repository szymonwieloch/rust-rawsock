use std::error::Error as ErrorTrait;
use std::fmt::{Display, Result as FmtResult, Formatter};
use std::convert::From;
use dlopen::Error as DlopenError;
use std::ffi::NulError;

/// Error enumeration returned by this crate.
#[derive(Debug)]
pub enum Error {
    DllError(DlopenError),
    ///Provided string could not be coverted into `std::ffi::CString` because it contained null
    /// character.
    NullCharacter(NulError),
    ///The interface could not be opened.
    OpeningInterface(String),
    ///Receiving raw packet failed.
    ReceivingPacket(String),
    ///Sending raw packet failed.
    SendingPacket(String),
    ///Obtaining device description list failed.
    GettingDeviceDescriptionList(String),
    ///No paths were provided by the user
    NoPathsProvided,
    LibraryError(String)
}

impl ErrorTrait for Error {
    fn description(&self) -> &str {
        match *self {
            Error::DllError(ref err) => err.description(),
            Error::NullCharacter(ref err) => err.description(),
            Error::OpeningInterface(ref txt) => txt,
            Error::ReceivingPacket(ref txt) => txt,
            Error::SendingPacket(ref txt) => txt,
            Error::GettingDeviceDescriptionList(ref txt) => txt,
            Error::NoPathsProvided => "No library paths were provided.",
            Error::LibraryError(ref txt) => txt
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match * self {
            Error::DllError(ref err) => err.fmt(f),
            Error::NullCharacter(ref err) => err.fmt(f),
            Error::OpeningInterface(ref txt)=> f.write_str(txt),
            Error::ReceivingPacket(ref txt) => f.write_str(txt),
            Error::SendingPacket(ref txt) => f.write_str(txt),
            Error::GettingDeviceDescriptionList(ref txt) => f.write_str(txt),
            Error::NoPathsProvided => f.write_str("No library paths were provided."),
            Error::LibraryError(ref txt) => f.write_str(txt)
        }
    }
}

impl From<::dlopen::Error> for Error {
    fn from(err: DlopenError) -> Error{
        Error::DllError(err)
    }
}

impl From<NulError> for Error {
    fn from(err: NulError) -> Error{
        Error::NullCharacter(err)
    }
}