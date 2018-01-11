use time::Timespec;

///Trait for obtained packets - common part between borrowed and owned versions.
pub trait Packet{
    ///Returns content of the packet
    fn data(&self) -> &[u8];
    ///Returns the time when this packet was received.
    fn when(&self) -> Timespec;
}

///Structure representing obtained raw packet - borrowed version.
#[derive(Debug)]
pub struct BorrowedPacket<'a> {
    when_received: Timespec,
    packet: &'a[u8],
    //_marker: PhantomData<&'a mut u32>
}

impl<'a> BorrowedPacket<'a> {
    ///Creates a new Packet instance.
    pub fn new(when_received: Timespec, data: &'a[u8]) -> BorrowedPacket<'a> {
        BorrowedPacket{
            when_received,
            packet: data,
            //_marker: PhantomData
        }
    }
    pub fn as_owned(&self) -> OwnedPacket {
        OwnedPacket::new(self.packet, self.when_received)
    }
    pub fn into_owned(self) -> OwnedPacket {
        OwnedPacket::new(self.packet, self.when_received)
    }
}

impl<'a> Packet for BorrowedPacket<'a> {
    fn data(&self) -> &[u8] {
        self.packet
    }

    fn when(&self) -> Timespec {
        self.when_received
    }
}

///Structure representing obtained raw packet - owned version.
pub struct OwnedPacket {
    when_received: Timespec,
    packet: Vec<u8>
}

impl Packet for OwnedPacket{
    fn data(&self) -> &[u8] {
        &self.packet
    }

    fn when(&self) -> Timespec {
        self.when_received
    }
}

impl OwnedPacket {
    pub fn new (data: &[u8], when: Timespec) -> Self{
        OwnedPacket {
            packet: data.into(),
            when_received: when
        }
    }

    pub fn as_borrowed(&self) -> BorrowedPacket {
        BorrowedPacket::new(self.when_received, &self.packet)
    }
}