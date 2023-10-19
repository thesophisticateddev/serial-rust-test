use std::io::{self, Write};
use std::time::Duration;

use serialport::SerialPortInfo;

fn handle_port(select_port: serialport::SerialPortInfo) {
    println!("Port name {}", select_port.port_name);
    let mut port = serialport::new(select_port.port_name, 115_200)
        .timeout(Duration::from_millis(1000))
        .open()
        .expect("Failed to open port");

    println!("Port opened!");
    let start_command = "s".as_bytes();

    port.write_all(start_command)
        .expect("Failed to write buffer to serial port");
    io::stdout()
        .write_all(start_command)
        .expect("Failed to write to stdout");
    port.flush();
    println!("Start command sent!");
    loop {
        let mut serial_buf: Vec<u8> = vec![0; 32];
        let rtn = port.read(&mut serial_buf);
        println!("Rtn {:?}", rtn);
        println!("Buffer read {:?}", serial_buf);
        let bytes_to_string = std::str::from_utf8(&serial_buf);
        println!("String val: {:?}", bytes_to_string);
    }
}

fn get_all_ports() -> Vec<SerialPortInfo> {
    let all_serial_ports = serialport::available_ports().expect("No ports detected");
    all_serial_ports
}

fn select_port(ports: Vec<SerialPortInfo>) -> Result<SerialPortInfo, ()> {
    for each_port in ports {
        let p_name: &str = &each_port.port_name;
        match p_name {
            "/dev/ttyACM0" => return Ok(each_port),
            _ => println!("Port not selected {}", p_name),
        }
    }
    Err(())
}

fn main() {
    let serial_ports = get_all_ports();
    let selected_port = select_port(serial_ports).expect("Port not selected");
    handle_port(selected_port);
    println!("Hello, world!");
}
