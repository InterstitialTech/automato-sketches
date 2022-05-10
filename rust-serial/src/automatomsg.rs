// --------------------------------------------------------
// message structs.
// --------------------------------------------------------

#[repr(C)]
enum PayloadType {
    PtAck,
    PtFail,
    PtPinmode,
    PtReadpin,
    PtReadpinreply,
    PtWritepin,
    PtReadmem,
    PtReadmemreply,
    PtWritemem,
    PtReadinfo,
    PtReadinforeply,
    PtReadhumidity,
    PtReadhumidityreply,
    PtReadtemperature,
    PtReadtemperaturereply,
    PtReadanalog,
    PtReadanalogreply,
    PtCount, // not a payload type; just the number of payload types.
}

#[derive(Clone, Copy)]
#[repr(C)]
#[repr(packed)]
struct RemoteInfo {
    protoversion: f32,
    macAddress: u64,
    datalen: u16,
}

#[derive(Clone, Copy)]
#[repr(C)]
#[repr(packed)]
struct Pinval {
    pin: u8,
    state: u8,
}

#[derive(Clone, Copy)]
#[repr(C)]
#[repr(packed)]
struct AnalogPinval {
    pin: u8,
    state: u16,
}

#[derive(Clone, Copy)]
#[repr(C)]
#[repr(packed)]
struct Pinmode {
    pin: u8,
    mode: u8,
}

#[derive(Clone, Copy)]
#[repr(C)]
#[repr(packed)]
struct Readmem {
    address: u16,
    length: u8,
}

const RH_RF95_MAX_MESSAGE_LEN: usize = 251; // 255 - 4.

// #define MAX_WRITEMEM RH_RF95_MAX_MESSAGE_LEN - sizeof(u16) - sizeof(u8) - sizeof(u8)
const MAX_WRITEMEM: usize = 247;
// #define MAX_READMEM RH_RF95_MAX_MESSAGE_LEN - sizeof(u8) - sizeof(u8)
const MAX_READMEM: usize = 249;

#[derive(Clone, Copy)]
#[repr(C)]
#[repr(packed)]
struct ReadmemReply {
    length: u8,
    data: [u8; MAX_READMEM],
}

#[derive(Clone, Copy)]
#[repr(C)]
#[repr(packed)]
struct Writemem {
    address: u16,
    length: u8,
    data: [u8; MAX_WRITEMEM],
}

#[derive(Clone, Copy)]
#[repr(C)]
#[repr(packed)]
union PayloadData {
    pinval: Pinval,
    pinmode: Pinmode,
    analogpinval: AnalogPinval,
    readmem: Readmem,
    readmemreply: ReadmemReply,
    writemem: Writemem,
    remoteinfo: RemoteInfo,
    failcode: u8,
    pin: u8,
    f: f32,
}

#[derive(Clone, Copy)]
#[repr(C)]
#[repr(packed)]
struct Payload {
    payload_type: u8,
    data: PayloadData,
}

// used for non-mesh, non-routed comms.
#[repr(C)]
#[repr(packed)]
struct Message {
    fromid: u8,
    toid: u8,
    data: Payload,
}

#[repr(C)]
#[repr(packed)]
union Msgbuf {
    buf: [u8; RH_RF95_MAX_MESSAGE_LEN],
    payload: Payload,
}
