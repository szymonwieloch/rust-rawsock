use rawsock::traits::Library;
use rawsock::pcap;

/*
Tests in this module require correctly setup environment. Therefore they are disabled (ignored)
by default. You can enable them by addding --ignored flag to your cargo testing command.
Some tests also may require administrative privileges.
*/

fn choose_interf(lib: &pcap::Library) -> Option<String>{
   match lib.all_interfaces() {
       Ok(i) => match i.first(){
           Some(j) => Some(j.name.clone()),
           None => None
       },
       Err(_) => None
   }
}

#[test]
#[ignore]
fn open_pcap() {
    let pcap = pcap::Library::open_default_paths().expect("Could not open pcap library");
    if let Some(ifname) = choose_interf(&pcap){
        let mut _interf = pcap.open_interface(&ifname).expect("Could not open interface");
        //on some interfaces there may be no traffic.
    }
}