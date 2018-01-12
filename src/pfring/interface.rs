use {Interface, Error, BorrowedPacket, DataLink};

pub struct PFRingInterface<'a> {
    _fake: &'a u32
}

impl<'a> Interface<'a> for PFRingInterface<'a> {
    fn send(&self, packet: &[u8]) -> Result<(), Error> {
        unimplemented!()
    }

    fn receive<'b>(&'b mut self) -> Result<BorrowedPacket<'b>, Error> {
        unimplemented!()
    }

    fn flush(&self) {
        unimplemented!()
    }

    fn data_link(&self) -> DataLink {
        unimplemented!()
    }
}