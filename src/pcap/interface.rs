use std::ffi::{CStr, CString};
use crate::{Error,  BorrowedPacket, DataLink, traits, Stats};
use super::dll::{PCapHandle, PCapDll, SUCCESS, PCapPacketHeader};
use super::dll::helpers::PCapErrBuf;
use super::structs::PCapStat;
use libc::{ c_int};
use std::mem::uninitialized;
use time::Timespec;
use std::slice::from_raw_parts;
use crate::utils::cstr_to_string;

///pcap version of interface.
pub struct Interface<'a> {
    handle: * const PCapHandle,
    dll: & 'a PCapDll,
    datalink: DataLink
}


impl<'a> Interface<'a> {
    pub fn new(name: &str, dll: &'a PCapDll) ->Result<Self, Error> {
        let name = CString::new(name)?;
        let mut errbuf =  PCapErrBuf::new();
        let handle = unsafe { dll.pcap_open_live(
            name.as_ptr(),
            65536,                  /* max packet size */
            8,                      /* promiscuous mode */
            1000,                   /* read timeout in milliseconds */
            errbuf.buffer()
        )};
        if handle.is_null() {
            return Err(Error::OpeningInterface(errbuf.as_string()))
        }
        let datalink = match unsafe{dll.pcap_datalink(handle)}{
            1 => DataLink::Ethernet,
            12 => DataLink::RawIp,
            _=> DataLink::Other
        };

        Ok(Interface {
            dll,
            handle,
            datalink
        })
    }

    fn last_error(&self) -> Error {
        let cerr = unsafe{self.dll.pcap_geterr(self.handle)};
        Error::LibraryError(cstr_to_string(cerr))
    }
}

impl<'a> Drop for Interface<'a> {
    fn drop(&mut self) {
        unsafe { self.dll.pcap_close(self.handle) }
    }
}

impl<'a> traits::Interface<'a> for Interface<'a> {
    fn send(&self, packet: &[u8]) -> Result<(), Error> {
        if unsafe {self.dll.pcap_sendpacket(self.handle, packet.as_ptr(), packet.len() as c_int)} == SUCCESS {
            Ok(())
        } else {
            let txt = unsafe {CStr::from_ptr(self.dll.pcap_geterr(self.handle))}.to_string_lossy().into_owned();
            Err(Error::SendingPacket(txt))
        }
    }

    fn receive(& mut self) -> Result<BorrowedPacket, Error>{
        let mut header: PCapPacketHeader = unsafe {uninitialized()};
        //TODO: replace pcap_next with pcap_next_ex to obtain more error information
        let data = unsafe { self.dll.pcap_next(self.handle, &mut header)};
        if data.is_null() {
                Err(Error::ReceivingPacket("Unknown error when obtaining packet".into()))
        } else {
            Ok(
                unsafe {
                    BorrowedPacket::new(Timespec::new(header.ts.tv_sec as i64, (header.ts.tv_usec * 1000) as i32), from_raw_parts(data, header.caplen as usize))
                })
        }
    }

    fn flush(&self) {
        //pcap does not flush its packets - ignore
    }

    fn data_link(&self) -> DataLink {
        self.datalink
    }

    fn stats(&self) -> Result<Stats, Error> {
        let mut stats: PCapStat = unsafe{uninitialized()};
        if SUCCESS == unsafe{self.dll.pcap_stats(self.handle, &mut stats)}{
            Ok(Stats{
                received: stats.ps_recv as u64,
                dropped: (stats.ps_drop + stats.ps_ifdrop) as u64
            })
        } else {
            Err(self.last_error())
        }
    }
}