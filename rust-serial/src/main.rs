use std::io::Read;
mod automatomsg;

use serial::{BaudRate, CharSize, FlowControl, Parity, PortSettings, SerialPort, StopBits};

fn main() {
    match err_main() {
        Ok(()) => (),
        Err(e) => {
            println!("error: {:?}", e)
        }
    }
}

fn err_main() -> Result<(), serial::Error> {
    let mut port = serial::open("/dev/ttyUSB0")?;

    let ps = PortSettings {
        baud_rate: BaudRate::Baud115200,
        char_size: CharSize::Bits8,
        parity: Parity::ParityNone,
        stop_bits: StopBits::Stop1,
        flow_control: FlowControl::FlowSoftware,
    };
    port.configure(&ps);

    let mut s = String::new();
    let mut buf = Vec::<u8>::new();

    loop {
        let mut monobuf = [0; 1];
        match port.read_exact(&mut monobuf) {
            Ok(len) => {
                // '\n' == 10
                if monobuf[0] == 10 {
                    println!("{}", s);
                    s = "".to_string();
                } else {
                    s.push(monobuf[0] as char);
                }
            }
            Err(e) => (),
            // println!("err: {}", e),
        }
        // match port.read_to_end(&mut buf) {
        //     Ok(len) => println!("read: {:?}", buf),
        //     Err(e) => println!("err: {}", e),
        // }
        // match port.read_to_string(&mut s) {
        //     Ok(len) => println!("read: {}", s),
        //     Err(e) => println!("err: {}", e),
        // }
    }
    println!("Hello, world!");
    Ok(())
}
