/*!
Common traits for all libraries.
*/

use crate::{BorrowedPacket, DataLink, LibraryVersion, Error, InterfaceDescription, Stats};
use std::iter::IntoIterator;


///Trait for structures representing an opened interface (or network card or network device)
///
/// Interfaces are opened using a concrete library - check the Library trait.
pub trait DynamicInterface<'a>{

    ///Sends a raw packet.
    fn send(&self, packet: &[u8]) -> Result<(), Error>;

    ///Receives a raw packet.
    fn receive<'b>(&'b mut self) -> Result<BorrowedPacket<'b>, Error>;

    ///Flushes a queue
    fn flush(&self);

    ///Provides information about the underlying technology used for this connection.
    fn data_link(&self) -> DataLink;

    ///Provides transmission statistics
    fn stats(&self) -> Result<Stats, Error>;

    ///Breaks previously started loops.
    fn break_loop(&  self);

    /**
    Runs infinite loop and passes received packets via callback.

    Exits when the break_loop() function is called or on error.
    */
    fn loop_infinite_dyn(&self, callback: & dyn FnMut(&BorrowedPacket)) -> Result<(), Error>;

    //TODO
    //loop
    //breakloop
    //bpf filters
    //receive/send with timeout

}

/**
    Contains static part of the interface trait.

    Template functions cannot be used for dynamic dispatch (&dyn) and it was necessary to split the
    interface trait into two parts - static and dynamic, depending how user uses the trait.
    StaticInterface contains only the part of trait that cannot be used in the dynamic way.
*/
pub trait StaticInterface<'a>: DynamicInterface<'a> {

    /**
    Runs infinite loop and passes received packets via callback.

    Exits when the break_loop() function is called or on error.
    */
    fn loop_infinite<F>(& self, callback: F) -> Result<(), Error> where F: FnMut(&BorrowedPacket);

}

/// Trait for structures representing opened packet capture libraries.
///
/// There are several libraries that can be used among different platforms.
/// For example pcap.so, wpcap.dll or pfring.so.
/// This trait provides a consistent interface to all of them.
pub trait Library{

    //const DEFAULT_PATHS: &'static [&'static str];

    ///Opens this library by searching for most common paths and names fro the given platform
    fn open_default_paths() -> Result<Self, Error> where Self: Sized {
        Self::open_paths(Self::default_paths().iter().map(|s|*s))
    }

    ///Returns list of default paths to the library on the given platform.
    fn default_paths() -> &'static[&'static str] where Self: Sized;

    ///Opens library searching in the list of provided paths.
    fn open_paths<'b, T>(paths: T) -> Result<Self, Error> where Self: Sized, T:IntoIterator<Item=&'b str>{
        let mut err = Error::NoPathsProvided;
        for path in paths.into_iter(){
            match Self::open(path) {
                Err(e) => err = e,
                Ok(lib) => return Ok(lib)
            }
        }
        Err(err)
    }

    ///Opens library by checking the provided path to it.
    fn open(path: &str) -> Result<Self, Error> where Self: Sized;

    ///Opens interface (network card or network device) with the provided name.
    ///
    /// You can obtain names of available devices by calling the all_interfaces() function.
    fn open_interface<'a>(&'a self, name: &str) -> Result<Box<dyn DynamicInterface<'a>+'a>, Error>;

    /**
    Obtains list of available network interfaces.

    Each of returned interface names can be further used to open interfaces.

    **Note:** each library may support different set of interfaces.
    This is because different libraries support different network interface types and some of them
    add to the list virtual interfaces (such as pcap "any" interface or pfring "zc:eth0").
    The same function called with pcap library will return different set of interfaces than run with pfring.
    However in both cases the returned interface list will be supported by currently used library.

    # Example

    ```no_run
    extern crate rawsock;
    use rawsock::open_best_library;
    use rawsock::traits::Library;

    fn main(){
        let lib = open_best_library().expect("Could not open any library.");
        let interfs = lib.all_interfaces().expect("Could not obtain interface list");
        for interf in &interfs{
            println!("Found interface: {}", &interf.name);
        }
        let interf = lib.open_interface(&interfs.first().unwrap().name)
            .expect("Could not open interface");
        // do something with the interface
    }
    ```

    */
    fn all_interfaces(&self) -> Result<Vec<InterfaceDescription>, Error>;

    ///Returns library version
    fn version(&self) -> LibraryVersion;
}


