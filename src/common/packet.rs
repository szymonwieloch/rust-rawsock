use time::Timespec;
use std::ops::Deref;

///Trait for obtained packets - common part between borrowed and owned versions.
pub trait Packet : Deref<Target=[u8]>{
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
    ///Converts Borrowed packet into owned version.
    pub fn as_owned(&self) -> OwnedPacket {
        OwnedPacket::new(self.packet, self.when_received)
    }
    ///Converts Borrowed packet into owned version.
    pub fn into_owned(self) -> OwnedPacket {
        OwnedPacket::new(self.packet, self.when_received)
    }
}

impl<'a> Deref for BorrowedPacket<'a> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.packet
    }
}

impl<'a> Packet for BorrowedPacket<'a> {

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

    fn when(&self) -> Timespec {
        self.when_received
    }
}

impl Deref for OwnedPacket {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.packet
    }
}

impl OwnedPacket {
    ///Creates owned packet from provided data.
    pub fn new (data: &[u8], when: Timespec) -> Self{
        OwnedPacket {
            packet: data.into(),
            when_received: when
        }
    }

    ///Returns its borrowed version.
    pub fn as_borrowed(&self) -> BorrowedPacket {
        BorrowedPacket::new(self.when_received, &self.packet)
    }
}