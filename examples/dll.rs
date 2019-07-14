/*!
    In rare cases you may want to access the dynamically load library API directly.
    This is not recommended since it is library-specific and may increase complexity of your
    application. But it may be the only way if you need to do something that is very specific
    to the given library.
    Rawsock provides internal API and C-like structure definitions that allows you to get full
    functionality from your libraries.
*/

extern crate rawsock;
extern crate dlopen;
use rawsock::pcap::dll::{SUCCESS, PCapDll, PCapInterface};
use rawsock::pcap::dll::helpers::PCapErrBuf;
use rawsock::pcap::DEFAULT_PATHS;
use dlopen::wrapper::Container;
use std::ptr::null;
use std::ffi::CStr;

fn open_library() -> Container<PCapDll>{
    for path in &DEFAULT_PATHS{
        match unsafe { Container::load(path)} {
            Err(_)=> (),
            Ok(lib) => return lib
        }
    }
    panic!("Could not open the library")
}

fn main(){
    let dll = open_library();

    //example: obtain list of devices directly (although rawsock API supports it too):
    let mut interfs: * const PCapInterface = null();
    let mut errbuf = PCapErrBuf::new();
    assert_eq!(SUCCESS, unsafe {dll.pcap_findalldevs(&mut interfs, errbuf.buffer())});
    let mut curr = interfs;
    while !curr.is_null() {
        let name = unsafe { CStr::from_ptr((*curr).name)}.to_string_lossy().into_owned();
        println!("Found interface: {}", &name);
        curr = unsafe{(*curr).next};
    }
    unsafe {dll.pcap_freealldevs(interfs)}
}