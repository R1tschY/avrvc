extern crate bytes;

use self::bytes::BytesMut;

#[derive(Debug)]
pub enum GdbServerPkt {
    Ack { okay: bool },
    Packet(BytesMut),
    CtrlC
}

