use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use std::mem::size_of;
// --------------------------------------------------------
// message structs.
// --------------------------------------------------------

#[derive(Debug, Eq, PartialEq, Copy, Clone, FromPrimitive, ToPrimitive)]
#[repr(C)]
pub enum PayloadType {
    PtAck = 0,
    PtFail = 1,
    PtPinmode = 2,
    PtReadpin = 3,
    PtReadpinreply = 4,
    PtWritepin = 5,
    PtReadmem = 6,
    PtReadmemreply = 7,
    PtWritemem = 8,
    PtReadinfo = 9,
    PtReadinforeply = 10,
    PtReadhumidity = 11,
    PtReadhumidityreply = 12,
    PtReadtemperature = 13,
    PtReadtemperaturereply = 14,
    PtReadanalog = 15,
    PtReadanalogreply = 16,
    PtCount = 17, // not a payload type; just the number of payload types.
}

// pub fn pt_code(pt: PayloadType) -> u8 {
//     let out = match pt {
//         PayloadType::PtAck => 0,
//         PayloadType::PtFail => 1,
//         PayloadType::PtPinmode => 2,
//         PayloadType::PtReadpin => 3,
//         PayloadType::PtReadpinreply => 4,
//         PayloadType::PtWritepin => 5,
//         PayloadType::PtReadmem => 6,
//         PayloadType::PtReadmemreply => 7,
//         PayloadType::PtWritemem => 8,
//         PayloadType::PtReadinfo => 9,
//         PayloadType::PtReadinforeply => 10,
//         PayloadType::PtReadhumidity => 11,
//         PayloadType::PtReadhumidityreply => 12,
//         PayloadType::PtReadtemperature => 13,
//         PayloadType::PtReadtemperaturereply => 14,
//         PayloadType::PtReadanalog => 15,
//         PayloadType::PtReadanalogreply => 16,
//         PayloadType::PtCount => 17, // not a payload type; just the number of payload types.
//     };

//     println!("out: {}", out);
//     out
// }

#[derive(Clone, Copy)]
#[repr(C)]
#[repr(packed)]
pub struct RemoteInfo {
    pub protoversion: f32,
    pub macAddress: u64,
    pub datalen: u16,
}

#[derive(Clone, Copy)]
#[repr(C)]
#[repr(packed)]
pub struct Pinval {
    pub pin: u8,
    pub state: u8,
}

#[derive(Clone, Copy)]
#[repr(C)]
#[repr(packed)]
pub struct AnalogPinval {
    pub pin: u8,
    pub state: u16,
}

#[derive(Clone, Copy)]
#[repr(C)]
#[repr(packed)]
pub struct Pinmode {
    pub pin: u8,
    pub mode: u8,
}

#[derive(Clone, Copy)]
#[repr(C)]
#[repr(packed)]
pub struct Readmem {
    pub address: u16,
    pub length: u8,
}

const RH_RF95_MAX_MESSAGE_LEN: usize = 251; // 255 - 4.

// #define MAX_WRITEMEM RH_RF95_MAX_MESSAGE_LEN - sizeof(u16) - sizeof(u8) - sizeof(u8)
const MAX_WRITEMEM: usize = 247;
// #define MAX_READMEM RH_RF95_MAX_MESSAGE_LEN - sizeof(u8) - sizeof(u8)
const MAX_READMEM: usize = 249;

#[derive(Clone, Copy)]
#[repr(C)]
#[repr(packed)]
pub struct ReadmemReply {
    pub length: u8,
    pub data: [u8; MAX_READMEM],
}

#[derive(Clone, Copy)]
#[repr(C)]
#[repr(packed)]
pub struct Writemem {
    pub address: u16,
    pub length: u8,
    pub data: [u8; MAX_WRITEMEM],
}

#[derive(Clone, Copy)]
#[repr(C)]
#[repr(packed)]
pub union PayloadData {
    pub pinval: Pinval,
    pub pinmode: Pinmode,
    pub analogpinval: AnalogPinval,
    pub readmem: Readmem,
    pub readmemreply: ReadmemReply,
    pub writemem: Writemem,
    pub remoteinfo: RemoteInfo,
    pub failcode: u8,
    pub pin: u8,
    pub f: f32,
}

#[derive(Clone, Copy)]
#[repr(C)]
#[repr(packed)]
pub struct Payload {
    pub payload_type: u8,
    pub data: PayloadData,
}

// used for non-mesh, non-routed comms.
#[repr(C)]
#[repr(packed)]
pub struct Message {
    pub fromid: u8,
    pub toid: u8,
    pub data: Payload,
}

#[repr(C)]
#[repr(packed)]
pub union Msgbuf {
    pub buf: [u8; RH_RF95_MAX_MESSAGE_LEN],
    pub payload: Payload,
}

// --------------------------------------------------------
// message fns.
// --------------------------------------------------------

fn payloadSize(p: &Payload) -> usize {
    match PayloadType::from_u8(p.payload_type) {
        Some(pt) => match pt {
            PayloadType::PtAck => size_of::<u8>(),
            PayloadType::PtFail => size_of::<u8>(),
            PayloadType::PtPinmode => size_of::<u8>() + size_of::<Pinmode>(),
            PayloadType::PtReadpin => size_of::<u8>() + size_of::<u8>(),
            PayloadType::PtReadpinreply => size_of::<u8>() + size_of::<Pinval>(),
            PayloadType::PtWritepin => size_of::<u8>() + size_of::<Pinval>(),
            PayloadType::PtReadanalog => size_of::<u8>() + size_of::<u8>(),
            PayloadType::PtReadanalogreply => size_of::<u8>() + size_of::<AnalogPinval>(),
            PayloadType::PtReadmem => size_of::<u8>() + size_of::<Readmem>(),
            PayloadType::PtReadmemreply => {
                size_of::<u8>() + size_of::<u8>() + p.data.readmemreply.length as usize
            }
            PayloadType::PtWritemem => {
                size_of::<u8>()
                    + size_of::<u16>()
                    + size_of::<u8>()
                    + p.data.readmemreply.length as usize
            }
            PayloadType::PtReadinfo => size_of::<u8>(),
            PayloadType::PtReadinforeply => size_of::<u8>() + size_of::<RemoteInfo>(),
            PayloadType::PtReadhumidity => size_of::<u8>(),
            PayloadType::PtReadhumidityreply => size_of::<u8>() + size_of::<f32>(),
            PayloadType::PtReadtemperature => size_of::<u8>(),
            PayloadType::PtReadtemperaturereply => size_of::<u8>() + size_of::<f32>(),
        },
        None => 0,
    }
}
