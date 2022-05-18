use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use std::mem::size_of;
// --------------------------------------------------------
// message structs.
// --------------------------------------------------------

#[derive(Debug, Eq, PartialEq, Copy, Clone, FromPrimitive, ToPrimitive)]
#[repr(u8)]
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

#[derive(Debug, Eq, PartialEq, Copy, Clone, FromPrimitive, ToPrimitive)]
#[repr(u8)]
pub enum ResultCode {
    RcOk,
    RcNoMessageReceived,
    RcInvalidMessageType,
    RcInvalidPinNumber,
    RcInvalidMemAddress,
    RcInvalidMemLength,
    RcInvalidReplyMessage,
    RcOperationForbidden,
    RcReplyTimeout,
    RcRhRouterErrorInvalidLength,
    RcRhRouterErrorNoRoute,
    RcRhRouterErrorTimeout,
    RcRhRouterErrorNoReply,
    RcRhRouterErrorUnableToDeliver,
    RcInvalidRhRouterError,
    RcCount, // total number of ResultCodes.
}

// --------------------------------------------------------
// message fns.
// --------------------------------------------------------

pub fn payloadSize(p: &Payload) -> usize {
    match PayloadType::from_u8(p.payload_type) {
        Some(pt) => match pt {
            PayloadType::PtAck => size_of::<u8>(),
            PayloadType::PtFail => size_of::<u8>() + size_of::<u8>(),
            PayloadType::PtPinmode => size_of::<u8>() + size_of::<Pinmode>(),
            PayloadType::PtReadpin => size_of::<u8>() + size_of::<u8>(),
            PayloadType::PtReadpinreply => size_of::<u8>() + size_of::<Pinval>(),
            PayloadType::PtWritepin => size_of::<u8>() + size_of::<Pinval>(),
            PayloadType::PtReadanalog => size_of::<u8>() + size_of::<u8>(),
            PayloadType::PtReadanalogreply => size_of::<u8>() + size_of::<AnalogPinval>(),
            PayloadType::PtReadmem => size_of::<u8>() + size_of::<Readmem>(),
            PayloadType::PtReadmemreply => unsafe {
                size_of::<u8>() + size_of::<u8>() + p.data.readmemreply.length as usize
            },
            PayloadType::PtWritemem => unsafe {
                size_of::<u8>()
                    + size_of::<u16>()
                    + size_of::<u8>()
                    + p.data.writemem.length as usize
            },
            PayloadType::PtReadinfo => size_of::<u8>(),
            PayloadType::PtReadinforeply => size_of::<u8>() + size_of::<RemoteInfo>(),
            PayloadType::PtReadhumidity => size_of::<u8>(),
            PayloadType::PtReadhumidityreply => size_of::<u8>() + size_of::<f32>(),
            PayloadType::PtReadtemperature => size_of::<u8>(),
            PayloadType::PtReadtemperaturereply => size_of::<u8>() + size_of::<f32>(),
            PayloadType::PtCount => 0,
        },
        None => 0,
    }
}

// AutomatoResult arFromReply(Payload &p)
// {
//     if (p.payload_type == PtFail)
//         return AutomatoResult((ResultCode)p.failcode);
//     else if (isReply((PayloadType)p.type))
//         return AutomatoResult(rc_ok);
//     else
//         return AutomatoResult(rc_invalid_reply_message);
// }

pub fn setup_ack(p: &mut Payload) {
    p.payload_type = PayloadType::PtAck as u8;
}

pub fn setup_fail(p: &mut Payload, rc: ResultCode) {
    p.payload_type = PayloadType::PtFail as u8;
    p.data.failcode = rc as u8;
}

pub fn setup_pinmode(p: &mut Payload, pin: u8, mode: u8) {
    p.payload_type = PayloadType::PtPinmode as u8;
    p.data.pinmode.pin = pin;
    p.data.pinmode.mode = mode;
}

pub fn setup_readpin(p: &mut Payload, pin: u8) {
    p.payload_type = PayloadType::PtReadpin as u8;
    p.data.pin = pin;
}

pub fn setup_readpinreply(p: &mut Payload, pin: u8, state: u8) {
    p.payload_type = PayloadType::PtReadpinreply as u8;
    p.data.pinval.pin = pin;
    p.data.pinval.state = state;
}

pub fn setup_writepin(p: &mut Payload, pin: u8, state: u8) {
    p.payload_type = PayloadType::PtWritepin as u8;
    p.data.pinval.pin = pin;
    p.data.pinval.state = state;
}

pub fn setup_readanalog(p: &mut Payload, pin: u8) {
    p.payload_type = PayloadType::PtReadanalog as u8;
    p.data.pin = pin;
}

pub fn setup_readanalogreply(p: &mut Payload, pin: u8, state: u16) {
    p.payload_type = PayloadType::PtReadanalogreply as u8;
    p.data.analogpinval.pin = pin;
    p.data.analogpinval.state = state;
}

pub fn setup_readmem(p: &mut Payload, address: u16, length: u8) {
    p.payload_type = PayloadType::PtReadmem as u8;
    p.data.readmem.address = address;
    p.data.readmem.length = length;
}

pub fn setup_readmemreply(p: &mut Payload, mem: &[u8]) -> ResultCode {
    p.payload_type = PayloadType::PtReadmemreply as u8;
    if (mem.len() <= MAX_READMEM) {
        p.data.readmemreply.length = mem.len() as u8;
        unsafe {
            p.data.readmemreply.data[0..mem.len()].copy_from_slice(&mem);
        }
        ResultCode::RcOk
    } else {
        ResultCode::RcInvalidMemLength
    }
}

pub fn setup_writemem(p: &mut Payload, address: u16, mem: &[u8]) -> ResultCode {
    p.payload_type = PayloadType::PtWritemem as u8;
    if (mem.len() <= MAX_WRITEMEM) {
        p.data.writemem.address = address;
        p.data.writemem.length = mem.len() as u8;
        unsafe {
            p.data.writemem.data[0..mem.len()].copy_from_slice(&mem);
        }
        ResultCode::RcOk
    } else {
        ResultCode::RcInvalidMemLength
    }
}

pub fn setup_readinfo(p: &mut Payload) {
    p.payload_type = PayloadType::PtReadinfo as u8;
}

pub fn setup_readinforeply(p: &mut Payload, protoversion: f32, macAddress: u64, datalen: u16) {
    p.payload_type = PayloadType::PtReadinforeply as u8;
    p.data.remoteinfo.protoversion = protoversion;
    p.data.remoteinfo.macAddress = macAddress;
    p.data.remoteinfo.datalen = datalen;
}

pub fn setup_readhumidity(p: &mut Payload) {
    p.payload_type = PayloadType::PtReadhumidity as u8;
}

pub fn setup_readhumidityreply(p: &mut Payload, humidity: f32) {
    p.payload_type = PayloadType::PtReadhumidityreply as u8;
    p.data.f = humidity;
}

pub fn setup_readtemperature(p: &mut Payload) {
    p.payload_type = PayloadType::PtReadtemperature as u8;
}

pub fn setup_readtemperaturereply(p: &mut Payload, temperature: f32) {
    p.payload_type = PayloadType::PtReadtemperaturereply as u8;
    p.data.f = temperature;
}
