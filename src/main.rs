use sysinfo::{ System, Networks, Disks};
// use std::net::{TcpStream, Shutdown};
// use std::io::prelude::*;

use tungstenite::connect;
use std::thread;
use std::time::Duration;


fn main() {
    let mut sys = System::new_all();
    let mut bandwith: u64 = 0;
    let mut freebandwith: u64 = 0;
    let mut disk_space = 0;
    sys.refresh_all();
    
    let mut networks = Networks::new_with_refreshed_list();
    for (interface_name, network) in &networks {
        bandwith = network.total_transmitted() + network.total_received();
    }

    let disks = Disks::new_with_refreshed_list();
    for disk in disks.list() {
        disk_space = disk.available_space()/1_000_000_000;
        break;
    }
    
    let (mut ws,_) = connect("ws://127.0.0.1:5432").unwrap();
    loop {
        
        networks.refresh();
        for (interface_name, network) in &networks {
            // freebandwith = network.transmitted() + network.received();
            freebandwith = bandwith - (network.transmitted() + network.received());
        }
        let cpu = sys.cpus().get(0).unwrap();
        
        let sysinfo = format!("{},{:.2},{},{},{},{}",
        System::host_name().unwrap(),
        cpu.cpu_usage(),
        sys.used_memory()/1_000_000,
        freebandwith/1_000_000,
        disk_space,
        sys.total_memory()/1_000_000
        );
        sys.refresh_all();

        thread::sleep(Duration::from_secs(1));
        // let sysinfo = format!("{},{},{},{},{},{},{}",
        // System::host_name().unwrap(),
        // cpu.brand().trim(),
        // cpu.frequency(),
        // sys.physical_core_count().unwrap(),
        // sys.total_memory(),
        // System::long_os_version().unwrap(),
        // System::kernel_version().unwrap()
        // );
        println!("{}", sysinfo);
        ws.send(tungstenite::Message::Text(sysinfo));
    }
}
