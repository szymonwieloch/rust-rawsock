mod packet;
mod lib_version;
mod interf_desc;
mod data_link;
mod err;

use crate::traits::Library;
use crate::{wpcap, pcap, pfring};
use std::sync::Arc;


pub use self::lib_version::LibraryVersion;
pub use self::packet::{Packet, OwnedPacket, BorrowedPacket};
pub use self::interf_desc::InterfaceDescription;
pub use self::data_link::DataLink;
pub use self::err::Error;



/**
Opens optimal library available on the platform.

# Example

```no_run
extern crate rawsock;
use rawsock::open_best_library;

fn main(){
    let lib = open_best_library().expect("Could not open any library.");

    // do something with the library
}
```
*/
pub fn open_best_library() -> Result<Box<dyn Library>, Error> {
    if let Ok(l) = pfring::Library::open_default_paths() {
        return Ok(Box::new(l));
    }
    if let Ok(l) = wpcap::Library::open_default_paths() {
        return Ok(Box::new(l));
    }
    match pcap::Library::open_default_paths() {
        Ok(l) => Ok(Box::new(l)),
        Err(e) => Err(e)
    }
}

/// Multi-thread version of open_best_library()
pub fn open_best_library_arc() -> Result<Arc<dyn Library>, Error> {
    if let Ok(l) = pfring::Library::open_default_paths() {
        return Ok(Arc::new(l));
    }
    if let Ok(l) = wpcap::Library::open_default_paths() {
        return Ok(Arc::new(l));
    }
    match pcap::Library::open_default_paths() {
        Ok(l) => Ok(Arc::new(l)),
        Err(e) => Err(e)
    }
}

///Provides library statistics
#[derive(Copy, Clone, Debug)]
pub struct Stats {
    /// Received frames
    pub received: u64,
    ///Dropped frames
    pub dropped: u64
}

