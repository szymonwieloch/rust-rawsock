use crate::{BorrowedPacket, DataLink, traits};
use super::dll::{PFRing, PFRingDll, PFRingPacketHeader};
use crate::Error;
use dlopen::wrapper::Container;
use std::ffi::CString;
use std::mem::uninitialized;
use time::Timespec;
use std::slice::from_raw_parts;
use libc::{c_uint};
use crate::utils::string_from_errno;
use super::string_from_pfring_err_code;

///pfring version of an interface.
pub struct Interface<'a> {
    handle: * mut PFRing,
    dll: & 'a Container<PFRingDll>,
}

impl<'a> Interface<'a>{
    pub fn new(name: &str, dll: &'a Container<PFRingDll>) -> Result<Self, Error> {
        let name = CString::new(name)?;
        let handle = unsafe{dll.pfring_open(name.as_ptr(),1500, 0)};//PF_RING_REENTRANT
        if handle.is_null(){
            return Err(Error::OpeningInterface(string_from_errno()));
        }

        let result = unsafe{dll.pfring_enable_ring(handle)};
        if  result < 0{
            unsafe{dll.pfring_close(handle)};
            return Err(Error::OpeningInterface(string_from_pfring_err_code(result)))
        }

        Ok(Self{
            handle, dll
        })
    }
}

impl<'a> Drop for Interface<'a> {
    fn drop(&mut self) {
        unsafe {self.dll.pfring_close(self.handle)};
    }
}

impl<'a> traits::Interface<'a> for Interface<'a> {
    fn send(&self, packet: &[u8]) -> Result<(), Error> {
        let result = unsafe{self.dll.pfring_send(self.handle, packet.as_ptr(), packet.len() as c_uint, 0)};
        if  result <0 {
            Err(Error::SendingPacket(string_from_pfring_err_code(result)))
        } else {
            Ok(())
        }
    }

    fn receive(& mut self) -> Result<BorrowedPacket, Error> {
        let mut buf: * mut u8 = unsafe{uninitialized()};
        let mut header: PFRingPacketHeader = unsafe{uninitialized()};
        let result = unsafe{self.dll.pfring_recv(self.handle, &mut buf, 0, &mut header, 1)};
        if result != 1 {
            Err(Error::ReceivingPacket(string_from_pfring_err_code(result)))
        } else {
            let packet = unsafe{from_raw_parts(buf, header.caplen as usize)};
            Ok(BorrowedPacket::new(Timespec::new(header.ts.tv_sec as i64, (header.ts.tv_usec*1000) as i32), packet))
        }
    }

    fn flush(&self) {
        //TODO: what about the return value?
        unsafe{self.dll.pfring_flush_tx_packets(self.handle)};
    }

    fn data_link(&self) -> DataLink {
        DataLink::Ethernet
    }
}