/*!
Code hat helps you use the original DLL API.
*/

use libc::c_int;
use std::convert::TryFrom;
use std::mem::transmute;

/// Safe wrapper around error codes returned by pfring API.
#[allow(dead_code)]
#[repr(i32)]
pub enum PFRingErrCode{
    Generic = -1,
    InvalidArgument = -2,
    NoPacketsAvailable = -3,
    NoTxSlotsAvailable = -4,
    WrongConfiguration = -5,
    EndOfDemoMode = -6,
    NotSupported = -7,
    InvalidLibVersion = -8,
    UnknownAdapter = -9,
    NotEnoughMemory = -10,
    InvalidStatus = -11,
    RingNotEnabled = -12
}

impl TryFrom<c_int> for PFRingErrCode{
    type Error = ();

    fn try_from(value: c_int) -> Result<Self, <Self as TryFrom<c_int>>::Error> {
        if PFRingErrCode::Generic as c_int >= value && PFRingErrCode::RingNotEnabled as c_int <= value {
            Ok(unsafe{transmute(value as i32)})
        } else {
            Err(())
        }
    }
}

impl PFRingErrCode {
    pub fn to_description(&self) -> &'static str {
        use self::PFRingErrCode::*;
        match *self {
            Generic => "Generic",
            InvalidArgument => "Invalid argument",
            NoPacketsAvailable => "No packets available",
            NoTxSlotsAvailable => "No TX slots available",
            WrongConfiguration => "Wront configuration",
            EndOfDemoMode => "End of demo mode",
            NotSupported => "Not supported",
            InvalidLibVersion => "Invalid library version",
            UnknownAdapter => "Unknown adapter",
            NotEnoughMemory => "Not enough memory",
            InvalidStatus => "Invalid status",
            RingNotEnabled => "Ring not enabled"
        }
    }
}

/// Converts pfring error code into human-friendly text.
pub fn string_from_pfring_err_code(err_code: c_int) -> String {
    if let Ok(err) = PFRingErrCode::try_from(err_code){
        String::from(err.to_description())
    } else {
        format!("Unknown PF Ring error code: {}", err_code)
    }
}