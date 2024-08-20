use std::io::{self, Read, Write};
use std::time::Duration;

fn main() {
    let mut port = serialport::new("/dev/tty.usbserial-10", 115_200)
        .timeout(Duration::from_millis(6000))
        .open()
        .expect("Error: ");
    port.write_data_terminal_ready(true);
    let mut buffer = String::new();
    port.write(b"AT\r");
    let mut buffer: [u8; 1] = [0; 1];
    loop {
        match port.read(&mut buffer) {
            Ok(bytes) => {
                if bytes == 1 {
                    println!("Received: {:?} {}", std::str::from_utf8(&buffer), buffer[0]);
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => println!("{:?}", e),
        }
    }
}
