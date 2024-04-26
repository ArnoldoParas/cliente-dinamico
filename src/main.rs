use sysinfo::System;
use std::net::{TcpStream, Shutdown};
use std::io::prelude::*;


fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:5432").unwrap();




    let mut sys = System::new_all();
    sys.refresh_all();

    let cpu = sys.cpus().get(0).unwrap();

    // println!("Modelo del procesador: {}", cpu.brand());
    // println!("Frecuencia del procesador: {}", cpu.frequency());
    // println!("Numero de procesadores logicos: {}", sys.cpus().len());
    // println!("Numero de nucleos fisicos: {}", sys.physical_core_count().unwrap());
    // println!("Memoria total: {} bytes", sys.total_memory());
    // println!("Version del sistema operativo: {}, version: {}", 
    //     System::long_os_version().unwrap(),
    //     System::kernel_version().unwrap());
    let sysinfo = format!("{},{},{},{},{},{},{}",
        System::host_name().unwrap(),
        cpu.brand().trim(),
        cpu.frequency(),
        sys.physical_core_count().unwrap(),
        sys.total_memory(),
        System::long_os_version().unwrap(),
        System::kernel_version().unwrap()
    );
    println!("{}", sysinfo);
    stream.write_all(sysinfo.as_bytes()).unwrap();
    stream.shutdown(Shutdown::Both).expect("fail to shut down write");
}
