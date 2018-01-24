use {PCap, PCapInterface, WPCap, WPCapInterface, PFRing, PFRingInterface, Library, Interface, Error, BorrowedPacket, DataLink};

///Wrapper over several library implementations that automatically chooses available library
/// and hides the internal complexity of passing calls tot he right implementation.
pub enum RawLib{
    PFRing(PFRing),
    PCap(PCap),
    WPCap(WPCap)
}

impl<'a> Library<'a> for RawLib{
    type Interf = RawInterf<'a>;

    fn open(path: &str) -> Result<Self, Error> where Self: Sized {
        //The order is : PF Ring, WPCap, PCap
        match PFRing::open(path){
            Ok(val) => Ok(RawLib::PFRing(val)),
            Err(..) => match WPCap::open(path) {
                Ok(val) => Ok(RawLib::WPCap(val)),
                Err(..) => match PCap::open(path) {
                    Ok(val) => Ok(RawLib::PCap(val)),
                    //use the last error as a result for the whole call
                    Err(err) => Err(err)
                }
            }
        }
    }

    fn open_interface(&'a self, name: &str) -> Result<Self::Interf, Error> {
        match * self {
            RawLib::PCap(ref pcap) => match pcap.open_interface(name){
                Ok(val) => Ok(RawInterf::PCap(val)),
                Err(err) => Err(err)
            },
            RawLib::WPCap(ref wpcap) => match wpcap.open_interface(name){
                Ok(val) => Ok(RawInterf::WPCap(val)),
                Err(err) => Err(err)
            },
            RawLib::PFRing(ref pfring) => match pfring.open_interface(name) {
                Ok(val) => Ok(RawInterf::PFRing(val)),
                Err(err) => Err(err)
            }
        }
    }

    fn open_default_locations() -> Result<Self, Error> where Self: Sized {
        //The order is : PF Ring, WPCap, PCap
        match PFRing::open_default_locations(){
            Ok(val) => Ok(RawLib::PFRing(val)),
            Err(..) => match WPCap::open_default_locations(){
                Ok(val) => Ok(RawLib::WPCap(val)),
                Err(..) => match PCap::open_default_locations() {
                    Ok(val) => Ok(RawLib::PCap(val)),
                    //use the last error as a result for the whole call
                    Err(err) => Err(err)
                }
            }
        }
    }
    fn version(&self) -> String {
        match *self {
            RawLib::PFRing(ref pfring) => pfring.version(),
            RawLib::WPCap(ref wpcap) => wpcap.version(),
            RawLib::PCap(ref pcap) => pcap.version()
        }
    }
}


impl RawLib {
    ///Returns library name plus library version
    pub fn full_version(&self) -> String {
        match *self{
            RawLib::PCap(ref pcap) => format!("pcap {}", pcap.version()),
            RawLib::WPCap(ref wpcap) => format!("wpcap {}", wpcap.version()),
            RawLib::PFRing(ref pfring) => format!("pfring {}", pfring.version()),
        }
    }
}

///Wrapper over several library-specific interface implementations that hides the internal
///complexity of passing calls tot he right implementation.
pub enum RawInterf<'a> {
    PFRing(PFRingInterface<'a>),
    WPCap(WPCapInterface<'a>),
    PCap(PCapInterface<'a>)
}

impl<'a> Interface<'a> for RawInterf<'a>{
    fn send(&self, packet: &[u8]) -> Result<(), Error> {
        match *self {
            RawInterf::PFRing(ref pfring) => pfring.send(packet),
            RawInterf::WPCap(ref wpcap) => wpcap.send(packet),
            RawInterf::PCap(ref pcap) => pcap.send(packet)
        }
    }

    fn receive<'b>(&'b mut self) -> Result<BorrowedPacket<'b>, Error> {
        match *self {
            RawInterf::PFRing(ref mut pfring) => pfring.receive(),
            RawInterf::WPCap(ref mut wpcap) => wpcap.receive(),
            RawInterf::PCap(ref mut pcap) => pcap.receive()
        }
    }

    fn flush(&self) {
        match *self {
            RawInterf::PFRing(ref pfring) => pfring.flush(),
            RawInterf::WPCap(ref wpcap) => wpcap.flush(),
            RawInterf::PCap(ref pcap) => pcap.flush()
        }
    }

    fn data_link(&self) -> DataLink {
        match *self {
            RawInterf::PFRing(ref pfring) => pfring.data_link(),
            RawInterf::WPCap(ref wpcap) => wpcap.data_link(),
            RawInterf::PCap(ref pcap) => pcap.data_link()
        }
    }
}