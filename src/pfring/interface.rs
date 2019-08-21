use crate::{BorrowedPacket, DataLink, traits, Stats};
use super::dll::{PFRing, PFRingDll, PFRingPacketHeader, PFRingStat, SUCCESS};
use crate::Error;
use dlopen::wrapper::Container;
use std::ffi::{CString, CStr};
use std::mem::uninitialized;
use libc::{c_uint, c_int, c_uchar};
use crate::utils::string_from_errno;
use super::dll::helpers::{string_from_pfring_err_code, borrowed_packet_from_header};
use std::mem::transmute;

///pfring version of an interface.
pub struct Interface<'a> {
    handle: * mut PFRing,
    dll: & 'a Container<PFRingDll>,
}

unsafe impl<'a> Sync for Interface<'a> {}
unsafe impl<'a> Send for Interface<'a> {}


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
            handle,
            dll,
        })
    }

    fn int_to_err(&self, err: c_int) -> Error {
        Error::LibraryError(string_from_pfring_err_code(err))
    }
}

impl<'a> Drop for Interface<'a> {
    fn drop(&mut self) {
        unsafe {self.dll.pfring_close(self.handle)};
    }
}

impl<'a> traits::DynamicInterface<'a> for Interface<'a> {
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
            Ok(borrowed_packet_from_header(&header, buf))
        }
    }

    fn flush(&self) {
        //TODO: what about the return value?
        unsafe{self.dll.pfring_flush_tx_packets(self.handle)};
    }

    fn data_link(&self) -> DataLink {
        DataLink::Ethernet
    }

    fn stats(&self) -> Result<Stats, Error> {
        let mut stats:PFRingStat = unsafe{uninitialized()};
        let result = unsafe{self.dll.pfring_stats(self.handle, &mut stats)};
        if result == SUCCESS {
            Ok(Stats {
                received: stats.recv as u64,
                dropped: stats.drop as u64
            })
        } else {
            Err(self.int_to_err(result))
        }
    }

    fn break_loop(& self) {
        unsafe{self.dll.pfring_breakloop(self.handle)};
    }

    fn loop_infinite_dyn(&self, callback: & dyn FnMut(&BorrowedPacket)) -> Result<(), Error> {
        let result = unsafe{self.dll.pfring_loop(self.handle, on_received_packet_dynamic, transmute(& callback), 1)};
        // This is super strange but although pfring_loop specification states that this function
        // should only return 0, it also returns 1. It happens when it finishes successfully after
        // a pfring_breakloop() call.
        if result == SUCCESS || result == 1 {
            Ok(())
        } else {
            Err(self.int_to_err(result))
        }
    }

    fn set_filter_cstr(&mut self, filter: &CStr) -> Result<(), Error> {
        let result = unsafe { self.dll.pfring_set_bpf_filter(self.handle, filter.as_ptr() as *mut i8) };
        if result == SUCCESS {
            Ok(())
        } else {
            Err(self.int_to_err(result))
        }
    }

    fn remove_filter(&mut self) -> Result<(), Error> {
        let result = unsafe { self.dll.pfring_remove_bpf_filter(self.handle) };
        if result == SUCCESS {
            Ok(())
        } else {
            Err(self.int_to_err(result))
        }
    }
}

extern "C" fn on_received_packet_static<F>(h: * const PFRingPacketHeader, p: * const c_uchar, user_bytes: * const c_uchar) where F: FnMut(&BorrowedPacket) {
    let callback: &mut F = unsafe{transmute(user_bytes)};

    let packet = borrowed_packet_from_header(unsafe{&*h}, p);
    callback(&packet)
}

extern "C" fn on_received_packet_dynamic(h: * const PFRingPacketHeader, p: * const c_uchar, user_bytes: * const c_uchar){
    let callback: &mut & mut dyn FnMut(&BorrowedPacket) = unsafe{transmute(user_bytes)};

    let packet = borrowed_packet_from_header(unsafe{&*h}, p);
    callback(&packet)
}

impl<'a> traits::StaticInterface<'a> for Interface<'a> {
    fn loop_infinite<F>(& self, callback: F) -> Result<(), Error> where F: FnMut(&BorrowedPacket) {
        let result = unsafe{self.dll.pfring_loop(self.handle, on_received_packet_static::<F>, transmute(& callback), 1)};
        if result == SUCCESS {
            Ok(())
        } else {
            Err(self.int_to_err(result))
        }
    }
}