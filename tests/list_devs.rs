use rawsock::Library;
use rawsock::pcap::PCapLibrary;
//use rawsock::wpcap::WPCapLibrary;
//use rawsock::pfring::PFRingLibrary;
use pnet::datalink;

/*
Tests in this module require correctly setup environment. Therefore they are disabled (ignored)
by default. You can enable them by addding --ignored flag to your cargo testing command.
Some tests also may require administrative privileges.
*/

fn choose_interf() -> Option<String>{
    match datalink::interfaces().first() {
        Some(i) => Some(i.name.clone()),
        None => None
    }
}

#[test]
#[ignore]
fn open_pcap() {
    let pcap = PCapLibrary::open_default_paths().expect("Could not open pcap library");
    if let Some(ifname) = choose_interf(){
        let mut _interf = pcap.open_interface(&ifname).expect("Could not open interface");
        //on some interfaces there may be no traffic.
    }
}