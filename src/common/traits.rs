use crate::err::Error;
use super::{BorrowedPacket, DataLink};

///Trait for structures representing an opened interface (or network card or network device)
///
/// Interfaces are opened using a concrete library - check the Library trait.
pub trait Interface<'a>{
    ///Sends a raw packet.
    fn send(&self, packet: &[u8]) -> Result<(), Error>;
    ///Receives a raw packet.
    fn receive<'b>(&'b mut self) -> Result<BorrowedPacket<'b>, Error>;
    ///Flushes a queue
    fn flush(&self);
    ///Provides information about the underlying technology used for this connection.
    fn data_link(&self) -> DataLink;
}

///Trait for structures representing opened packet capture libraries.
///
/// There are several libraries that can be used among different platforms.
/// For example pcap.so, wpcap.dll or pfring.so.
/// This trait provides a consistent interface to all of them.
pub trait Library<'a>{
    ///Type of interface that gets opened by this library
    type Interf: Interface<'a>;
    ///Opens this library by searching for most comon paths and names fro the given platform
    fn open_default_locations() -> Result<Self, Error> where Self: Sized;
    ///Opens library by checking the provided path to it.
    fn open(path: &str) -> Result<Self, Error> where Self: Sized;
    ///Opens interface (network card or network device) with the provided name.
    /// Name depends on the platform, for example on linux this is a path to the file representing
    /// a device, on Windows this is a GUID of the device.
    fn open_interface(&'a self, name: &str) -> Result<Self::Interf, Error>;

    ///Returns library version
    fn version(&self) -> String;
}
