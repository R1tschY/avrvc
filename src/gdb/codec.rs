extern crate bytes;
extern crate tokio_io;

use self::bytes::{BufMut, BytesMut};
use self::tokio_io::codec::{Encoder, Decoder};
use std::io;
use std::str;
use gdb::GdbServerPkt;

/// `Codec` for `GdbServerPackage`s
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct GdbServerCodec(());

impl GdbServerCodec {
    /// Creates a new `GdbServerCodec`.
    pub fn new() -> Self { GdbServerCodec(())  }


}

fn check_checksum(packet: &[u8]) -> Result<(), io::Error> {

    let checksum_hex = str::from_utf8(&packet[packet.len() - 2 ..]).map_err(|_| io::Error::new(
        io::ErrorKind::InvalidData,
        format!("invalid checksum: {:x} {:x}", packet[packet.len() - 2], packet[packet.len() - 3])))?;

    let checksum = u8::from_str_radix(checksum_hex, 16).map_err(|_| io::Error::new(
        io::ErrorKind::InvalidData,
        format!("invalid checksum: {}", checksum_hex)))?;

    let data = &packet[1 .. packet.len() - 3];
    let sum = data.iter().fold(0u8, |acc, &x| ((acc as u16 + x as u16) & 0xFFu16) as u8);
    if sum == checksum {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("checksum error: 0x{:x} != 0x{:x}", checksum, sum)))
    }
}


impl Decoder for GdbServerCodec {
    type Item = GdbServerPkt;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let len = buf.len();
        if len == 0 {
            Ok(None)
        } else {
            match buf[0] {
                b'+' | b'-' => {
                    let ack_char = buf[0];
                    buf.advance(1);
                    Ok(Some(GdbServerPkt::Ack { okay: ack_char == b'+' } ))
                },
                b'$' => {
                    let hashtag_pos = buf.iter()
                        .position(|&byte| byte == b'#');
                    match hashtag_pos {
                        Some(pos) if pos + 2 < buf.len() => {
                            let mut packet = buf.split_to(pos + 3).freeze();
                            check_checksum(&packet)?;
                            packet.truncate(pos);
                            packet.advance(1);
                            Ok(Some(GdbServerPkt::Packet(packet)))
                        },
                        _ =>
                            Ok(None)
                    }
                },
                b'\x03' => {
                    buf.advance(1);
                    Ok(Some(GdbServerPkt::CtrlC))
                },
                _ => Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("invalid message start: 0x{:x}", buf[0])))
            }
        }
    }
}

impl Encoder for GdbServerCodec {
    type Item = GdbServerPkt;
    type Error = io::Error;

    fn encode(&mut self, data: GdbServerPkt, buf: &mut BytesMut) -> Result<(), io::Error> {
        match data {
            GdbServerPkt::Packet(data) => {
                buf.reserve(data.len() + 4);
                buf.put(b'$');
                buf.put(data);
                buf.put(b'#');
                buf.put(b'0'); // TODO
                buf.put(b'0');
            },
            GdbServerPkt::Ack { okay } => {
                buf.put(if okay { b'+' } else { b'-' })

            },
            GdbServerPkt::CtrlC => buf.put(b'\x03')
        };
        println!("ENCODE: {}", String::from_utf8_lossy(buf));
        Ok(())
    }
}
