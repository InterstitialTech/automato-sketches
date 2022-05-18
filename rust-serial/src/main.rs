use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use std::io::{Read, Write};
mod automatomsg;
use automatomsg::{
    AnalogPinval, Message, Payload, PayloadData, PayloadType, Pinmode, Pinval, Readmem,
    ReadmemReply, RemoteInfo, Writemem,
};
use std::fs::File;
use std::mem::size_of;
use std::thread::sleep;
use std::time::Duration;

use num_derive;

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

    let PIN_LED: u8 = 26;

    let mut onmsg = automatomsg::Msgbuf {
        payload: Payload {
            payload_type: automatomsg::PayloadType::PtWritepin.to_u8(),
            data: PayloadData {
                pinval: Pinval {
                    pin: PIN_LED,
                    state: 1,
                },
            },
        },
    };
    let mut offmsg = automatomsg::Msgbuf {
        payload: Payload {
            payload_type: automatomsg::PayloadType::PtWritepin.to_u8(),
            data: PayloadData {
                pinval: Pinval {
                    pin: PIN_LED,
                    state: 0,
                },
            },
        },
    };

    println!("pinvalsz: {}", size_of::<Pinval>());
    println!("PayloadType: {}", size_of::<PayloadType>()); // size 4!

    let sz = size_of::<Pinval>() + size_of::<u8>();

    println!("sz: {}", sz);

    println!("ptCoutn: {}", automatomsg::PayloadType::PtCount.to_u8());

    unsafe {
        println!("payload_type: {}", onmsg.payload.payload_type);
    }

    // write 'on' message to file.
    let mut onfile = File::create("onmsg.bin")?;
    unsafe {
        onfile.write(&onmsg.buf[0..6]);
    }

    let id: u8 = 1;

    // mb.payload.payload_type = automatomsg::pt_code(automatomsg::PayloadType::PtWritepin);
    // mb.payload.data.pin = 20;
    // mb.payload.data.state = 1;
    //
    let mut on = true;

    loop {
        unsafe {
            println!("on {}", on);
            if on {
                port.write(&['m' as u8]);
                port.write(&[id as u8]);
                port.write(&[sz as u8]);
                port.write(&onmsg.buf[0..sz + 1]);
            } else {
                port.write(&['m' as u8]);
                port.write(&[id as u8]);
                port.write(&[sz as u8]);
                port.write(&offmsg.buf[0..sz + 1]);
            }
        }

        on = !on;

        sleep(Duration::from_millis(20));

        let mut monobuf = [0; 1];
        while match port.read_exact(&mut monobuf) {
            Ok(len) => {
                println!("{} '{}'", monobuf[0] as u8, monobuf[0] as char);
                // '\n' == 10
                if monobuf[0] == 10 {
                    println!("{}", s);
                    s = "".to_string();
                } else {
                    s.push(monobuf[0] as char);
                }
                true
            }
            Err(e) => false,
            // println!("err: {}", e),
        } {}
        // println!("onmsg: {}", size_of(onmsg));
        // match port.read_to_end(&mut buf) {
        //     Ok(len) => println!("read: {:?}", buf),
        //     Err(e) => println!("err: {}", e),
        // }
        // match port.read_to_string(&mut s) {
        //     Ok(len) => println!("read: {}", s),
        //     Err(e) => println!("err: {}", e),
        // }
    }
}
