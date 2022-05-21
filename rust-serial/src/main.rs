use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use std::io::{Read, Write};
mod automatomsg;
use automatomsg::{
    AnalogPinval, Message, Payload, PayloadData, PayloadType, Pinmode, Pinval, Readmem,
    ReadmemReply, RemoteInfo, ResultCode, Writemem,
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
    port.configure(&ps)?;

    // let mut s = String::new();
    // let mut buf = Vec::<u8>::new();

    let PIN_LED: u8 = 26;

    let onmsg = automatomsg::Msgbuf {
        payload: Payload {
            payload_type: automatomsg::PayloadType::PtWritepin as u8, // .to_u8(),
            data: PayloadData {
                pinval: Pinval {
                    pin: PIN_LED,
                    state: 1,
                },
            },
        },
    };
    let offmsg = automatomsg::Msgbuf {
        payload: Payload {
            payload_type: automatomsg::PayloadType::PtWritepin as u8,
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

    // println!("ptCoutn: {}", automatomsg::PayloadType::PtCount as u8);

    unsafe {
        println!("payload_type: {}", onmsg.payload.payload_type);
    }

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

    let mut retmsg = offmsg.clone();

    loop {
        // unsafe {
        //     println!("on {}", on);
        //     if on {
        //         port.write(&['m' as u8]);
        //         port.write(&[id as u8]);
        //         port.write(&[sz as u8]);
        //         port.write(&onmsg.buf[0..sz + 1]);
        //     } else {
        //         port.write(&['m' as u8]);
        //         port.write(&[id as u8]);
        //         port.write(&[sz as u8]);
        //         port.write(&offmsg.buf[0..sz + 1]);
        //     }
        // }
        unsafe {
            if on {
                writeMessage(&mut port, &onmsg, id)?;
            } else {
                writeMessage(&mut port, &offmsg, id)?;
            }

            let mut fromid: u8 = 0;
            readMessage(&mut port, &mut retmsg, &mut fromid);

            println!("payloadsize: {}", automatomsg::payloadSize(&retmsg.payload));
        }

        on = !on;

        sleep(Duration::from_millis(20));

        // readmsg function.
        // read 'm'
        // read id?
        // read len
        // read rest.

        /*
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
        */
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
    Ok(())
}

unsafe fn writeMessage(
    port: &mut serial::SystemPort,
    msg: &automatomsg::Msgbuf,
    toid: u8,
) -> Result<(), serial::Error> {
    let sz = automatomsg::payloadSize(&msg.payload);

    port.write(&['m' as u8])?;
    port.write(&[toid as u8])?;
    port.write(&[sz as u8])?;
    port.write(&msg.buf[0..sz + 1])?;

    Ok(())
}

unsafe fn readMessage(
    port: &mut serial::SystemPort,
    msg: &mut automatomsg::Msgbuf,
    fromid: &mut u8,
) -> Result<bool, serial::Error> {
    let mut monobuf = [0; 1];
    port.read_exact(&mut monobuf)?;
    if monobuf[0] as char != 'm' {
        return Ok(false);
    }
    port.read_exact(&mut monobuf)?;
    *fromid = monobuf[0];

    port.read_exact(&mut monobuf)?;
    let sz = monobuf[0] as usize;

    port.read_exact(&mut msg.buf[0..sz])?;

    Ok(true)
}

fn writeMessageFiles() -> Result<(), serial::Error> {
    let mut mutmsg = automatomsg::Msgbuf {
        payload: Payload {
            payload_type: automatomsg::PayloadType::PtAck as u8,
            data: PayloadData { pin: 0 },
        },
    };

    unsafe {
        automatomsg::setup_ack(&mut mutmsg.payload);
        let mut onfile = File::create("ack.bin")?;
        onfile.write(&mutmsg.buf[0..automatomsg::payloadSize(&mutmsg.payload)])?;

        println!("setup_ack: {}", automatomsg::payloadSize(&mutmsg.payload));
    }

    unsafe {
        automatomsg::setup_fail(&mut mutmsg.payload, ResultCode::RcInvalidRhRouterError);
        let mut onfile = File::create("fail.bin")?;
        onfile.write(&mutmsg.buf[0..automatomsg::payloadSize(&mutmsg.payload)])?;

        println!("setup_fail: {}", automatomsg::payloadSize(&mutmsg.payload));
    }

    unsafe {
        automatomsg::setup_pinmode(&mut mutmsg.payload, 26, 2);
        let mut onfile = File::create("pinmode.bin")?;
        onfile.write(&mutmsg.buf[0..automatomsg::payloadSize(&mutmsg.payload)])?;

        println!(
            "setup_pinmode: {}",
            automatomsg::payloadSize(&mutmsg.payload)
        );
    }

    unsafe {
        automatomsg::setup_readpin(&mut mutmsg.payload, 22);
        let mut onfile = File::create("readpin.bin")?;
        onfile.write(&mutmsg.buf[0..automatomsg::payloadSize(&mutmsg.payload)])?;

        println!(
            "setup_readpin: {}",
            automatomsg::payloadSize(&mutmsg.payload)
        );
    }

    unsafe {
        automatomsg::setup_readpinreply(&mut mutmsg.payload, 26, 1);
        let mut onfile = File::create("readpinreply.bin")?;
        onfile.write(&mutmsg.buf[0..automatomsg::payloadSize(&mutmsg.payload)])?;

        println!(
            "setup_readpinreply: {}",
            automatomsg::payloadSize(&mutmsg.payload)
        );
    }

    unsafe {
        automatomsg::setup_writepin(&mut mutmsg.payload, 15, 1);
        let mut onfile = File::create("writepin.bin")?;
        onfile.write(&mutmsg.buf[0..automatomsg::payloadSize(&mutmsg.payload)])?;

        println!(
            "setup_writepin: {}",
            automatomsg::payloadSize(&mutmsg.payload)
        );
    }

    unsafe {
        automatomsg::setup_readanalog(&mut mutmsg.payload, 27);
        let mut onfile = File::create("readanalog.bin")?;
        onfile.write(&mutmsg.buf[0..automatomsg::payloadSize(&mutmsg.payload)])?;

        println!(
            "setup_readanalog: {}",
            automatomsg::payloadSize(&mutmsg.payload)
        );
    }

    unsafe {
        automatomsg::setup_readanalogreply(&mut mutmsg.payload, 6, 500);
        let mut onfile = File::create("readanalogreply.bin")?;
        onfile.write(&mutmsg.buf[0..automatomsg::payloadSize(&mutmsg.payload)])?;

        println!(
            "setup_readanalogreply: {}",
            automatomsg::payloadSize(&mutmsg.payload)
        );
    }

    unsafe {
        automatomsg::setup_readmem(&mut mutmsg.payload, 1500, 75);
        let mut onfile = File::create("readmem.bin")?;
        onfile.write(&mutmsg.buf[0..automatomsg::payloadSize(&mutmsg.payload)])?;

        println!(
            "setup_readmem: {}",
            automatomsg::payloadSize(&mutmsg.payload)
        );
    }

    unsafe {
        let test = [1, 2, 3, 4, 5];
        automatomsg::setup_readmemreply(&mut mutmsg.payload, &test);
        let mut onfile = File::create("readmemreply.bin")?;
        onfile.write(&mutmsg.buf[0..automatomsg::payloadSize(&mutmsg.payload)])?;

        println!(
            "setup_readmemreply: {}",
            automatomsg::payloadSize(&mutmsg.payload)
        );
    }

    unsafe {
        let test = [5, 4, 3, 2, 1];
        automatomsg::setup_writemem(&mut mutmsg.payload, 5678, &test);
        let mut onfile = File::create("writemem.bin")?;
        onfile.write(&mutmsg.buf[0..automatomsg::payloadSize(&mutmsg.payload)])?;

        println!(
            "setup_writemem: {}",
            automatomsg::payloadSize(&mutmsg.payload)
        );
    }

    unsafe {
        automatomsg::setup_readinfo(&mut mutmsg.payload);
        let mut onfile = File::create("readinfo.bin")?;
        onfile.write(&mutmsg.buf[0..automatomsg::payloadSize(&mutmsg.payload)])?;

        println!(
            "setup_readinfo: {}",
            automatomsg::payloadSize(&mutmsg.payload)
        );
    }

    unsafe {
        automatomsg::setup_readinforeply(&mut mutmsg.payload, 1.1, 5678, 5000);
        let mut onfile = File::create("readinforeply.bin")?;
        onfile.write(&mutmsg.buf[0..automatomsg::payloadSize(&mutmsg.payload)])?;

        println!(
            "setup_readinforeply: {}",
            automatomsg::payloadSize(&mutmsg.payload)
        );
    }

    unsafe {
        automatomsg::setup_readhumidity(&mut mutmsg.payload);
        let mut onfile = File::create("readhumidity.bin")?;
        onfile.write(&mutmsg.buf[0..automatomsg::payloadSize(&mutmsg.payload)])?;

        println!(
            "setup_readhumidity: {}",
            automatomsg::payloadSize(&mutmsg.payload)
        );
    }

    unsafe {
        automatomsg::setup_readhumidityreply(&mut mutmsg.payload, 45.7);
        let mut onfile = File::create("readhumidityreply.bin")?;
        onfile.write(&mutmsg.buf[0..automatomsg::payloadSize(&mutmsg.payload)])?;

        println!(
            "setup_readhumidityreply: {}",
            automatomsg::payloadSize(&mutmsg.payload)
        );
    }

    unsafe {
        automatomsg::setup_readtemperature(&mut mutmsg.payload);
        let mut onfile = File::create("readtemperature.bin")?;
        onfile.write(&mutmsg.buf[0..automatomsg::payloadSize(&mutmsg.payload)])?;

        println!(
            "setup_readtemperature: {}",
            automatomsg::payloadSize(&mutmsg.payload)
        );
    }

    unsafe {
        automatomsg::setup_readtemperaturereply(&mut mutmsg.payload, 98.6);
        let mut onfile = File::create("readtemperaturereply.bin")?;
        onfile.write(&mutmsg.buf[0..automatomsg::payloadSize(&mutmsg.payload)])?;

        println!(
            "setup_readtemperaturereply: {}",
            automatomsg::payloadSize(&mutmsg.payload)
        );
    }

    Ok(())
}
