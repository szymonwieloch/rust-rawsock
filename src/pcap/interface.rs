use std::ffi::{CStr, CString};
use crate::{Error,  BorrowedPacket, DataLink, traits, Stats};
use super::dll::{PCapHandle, PCapDll, PCapPacketHeader};
use super::dll::helpers::PCapErrBuf;
use super::structs::PCapStat;
use libc::{ c_int};
use std::mem::{uninitialized, transmute};
use crate::utils::cstr_to_string;
use std::sync::Mutex;

use crate::pcap_common::helpers::{borrowed_packet_from_header, on_received_packet_static, on_received_packet_dynamic};
use crate::pcap_common::constants::{SUCCESS, PCAP_ERROR_BREAK, PCAP_EMPTY_FILTER_STR};
use crate::pcap_common::BpfProgram;

lazy_static! {
	static ref COMPILE_GUARD : Mutex<()> = Mutex::new(());
}

///pcap version of interface.
pub struct Interface<'a> {
    handle: * const PCapHandle,
    dll: & 'a PCapDll,
    datalink: DataLink,
}

unsafe impl<'a> Sync for Interface<'a> {}
unsafe impl<'a> Send for Interface<'a> {}


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
            datalink,
        })
    }

    fn last_error(&self) -> Error {
        let cerr = unsafe{self.dll.pcap_geterr(self.handle)};
        Error::LibraryError(cstr_to_string(cerr))
    }
}

impl<'a> Drop for Interface<'a> {
    fn drop(&mut self) {
        unsafe {
            self.dll.pcap_close(self.handle);
        }
    }
}

impl<'a> traits::DynamicInterface<'a> for Interface<'a> {
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
            Ok(borrowed_packet_from_header(& header, data))
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

    fn break_loop(& self) {
        unsafe{self.dll.pcap_breakloop(self.handle)}
    }

    fn loop_infinite_dyn(&self, callback: & dyn FnMut(&BorrowedPacket)) -> Result<(), Error> {
        let result = unsafe { self.dll.pcap_loop(self.handle, -1, on_received_packet_dynamic, transmute(&callback)) };
        if result == SUCCESS || result == PCAP_ERROR_BREAK {
            Ok(())
        } else {
            Err(self.last_error())
        }
    }

    fn set_filter_cstr(&mut self, filter: &CStr) -> Result<(), Error> {
        let mut bpf_filter: BpfProgram = unsafe {uninitialized()};
        // before pcap 1.8, pcap_compile was not thread safe (https://www.tcpdump.org/manpages/pcap_compile.3pcap.html)
        let result = {
            let _lock = COMPILE_GUARD.lock().unwrap();
            unsafe { self.dll.pcap_compile(self.handle, &mut bpf_filter, filter.as_ptr(), 1, 0) }
        };
        if result != SUCCESS {
            return Err(self.last_error())
        }
        let result = unsafe { self.dll.pcap_setfilter(self.handle, &mut bpf_filter) };
        unsafe { self.dll.pcap_freecode(&mut bpf_filter) };
        if result == SUCCESS {
            Ok(())
        } else {
            Err(self.last_error())
        }
    }

    fn remove_filter(&mut self) -> Result<(), Error> {
        self.set_filter_cstr(unsafe {
            CStr::from_bytes_with_nul_unchecked(PCAP_EMPTY_FILTER_STR)
        })
    }
}


impl<'a> traits::StaticInterface<'a> for Interface<'a> {
    fn loop_infinite<F>(& self, callback: F) -> Result<(), Error> where F: FnMut(&BorrowedPacket) {
        let result = unsafe { self.dll.pcap_loop(self.handle, -1, on_received_packet_static::<F>, transmute(&callback)) };
        if result == SUCCESS || result == PCAP_ERROR_BREAK {
            Ok(())
        } else {
            Err(self.last_error())
        }
    }
}
