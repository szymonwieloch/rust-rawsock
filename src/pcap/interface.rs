use std::ffi::CStr;
use super::super::{Error, Packet, BorrowedPacket, Interface, RawSock};
use super::dll::{PCapHandle, PCapDll, SUCCESS, PCapPacketHeader};
use libc::{c_char, c_void, c_uint, c_int};
use std::mem::uninitialized;
use time::Timespec;
use std::slice::from_raw_parts;

pub struct PCapInterface<'a> {
    handle: * const PCapHandle,
    dll: & 'a PCapDll
}


impl<'a> PCapInterface<'a> {
    pub fn new(handle: * const PCapHandle, dll: &'a PCapDll) ->Self {
        PCapInterface{
            dll: dll,
            handle: handle
        }
    }
}

impl<'a> Drop for PCapInterface<'a> {
    fn drop(&mut self) {
        unsafe { self.dll.pcap_close(self.handle) }
    }
}

impl<'a> Interface<'a> for PCapInterface<'a> {
    fn send(&self, packet: &[u8]) -> Result<(), Error> {
        if unsafe {self.dll.pcap_sendpacket(self.handle, packet.as_ptr(), packet.len() as c_int)} == SUCCESS {
            Ok(())
        } else {
            let txt = unsafe {CStr::from_ptr(self.dll.pcap_geterr(self.handle))}.to_string_lossy().into_owned();
            Err(Error::SendingPacket(txt))
        }
    }

    fn receive<'b>(&'b mut self) -> Result<BorrowedPacket<'b>, Error>{
        let mut header: PCapPacketHeader = unsafe {uninitialized()};
        //TODO: replace pcap_next with pcap_next_ex to obtain more error information
        let data = unsafe { self.dll.pcap_next(self.handle, &mut header)};
        if data.is_null() {
                Err(Error::ReceivingPacket("Unknown error when obtaining packet".into()))
        } else {
            Ok(
                unsafe {
                    BorrowedPacket::new(Timespec::new(header.ts.tv_sec as i64, header.ts.tv_usec * 1000), from_raw_parts(data, header.caplen as usize))
                })
        }
    }

    fn flush(&self) {
        unimplemented!()
    }

    fn get_ip(&self) {
        unimplemented!()
    }

    fn get_mac(&self) {
        unimplemented!()
    }

    fn get_default_gateway(&self) {
        unimplemented!()
    }
}