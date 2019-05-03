use rawsock::Library;
use rawsock::pcap::PCapLibrary;
use rawsock::wpcap::WPCapLibrary;
use rawsock::pfring::PFRingLibrary;
use interfaces2::Interface;

/*
Tests in this module require correctly setup environment. Therefore they are disabled (ignored)
by default. You can enable them by addding --ignored flag to your cargo testing command.
Some tests also may require administrative privileges.
*/

fn choose_interf() -> Option<String>{
    let interfs = Interface::get_all().expect("Could not obtain interface list");
    if let Some(i) = interfs.first(){
        Some(i.name.clone())
    } else {
        None
    }
}

#[test]
#[ignore]
fn open_pcap() {
    let pcap = PCapLibrary::open_default_paths().expect("Could not open pcap library");
    if let Some(ifname) = choose_interf(){
        let mut interf = pcap.open_interface(&ifname).expect("Could not open interface");
        //on some interfaces there may be no traffic.
    }
}