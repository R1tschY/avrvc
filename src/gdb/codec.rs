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
    let sum = calc_checksum(data);
    if sum == checksum {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("checksum error: 0x{:x} != 0x{:x}", checksum, sum)))
    }
}

fn calc_checksum(data: &[u8]) -> u8 {
    data.iter().fold(0u8, |acc, &x| ((acc as u16 + x as u16) & 0xFFu16) as u8)
}


impl Decoder for GdbServerCodec {
    type Item = GdbServerPkt;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        println!("DECODE: {}", String::from_utf8_lossy(buf));
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
                let checksum = calc_checksum(&data);

                buf.reserve(data.len() + 4);
                buf.put(b'$');
                buf.put(data);
                buf.put(b'#');
                buf.put(format!("{:02x}", checksum).as_bytes());
            },
            GdbServerPkt::Ack { okay } => {
                buf.reserve(1);
                buf.put(if okay { b'+' } else { b'-' })

            },
            GdbServerPkt::CtrlC => buf.put(b'\x03')
        };
        println!("ENCODE: {}", String::from_utf8_lossy(buf));
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_checksum() {
        assert_eq!(calc_checksum(b"Hg0"), 0xdf);
        assert_eq!(calc_checksum(b""), 0x00);
        assert_eq!(calc_checksum(b"vMustReplyEmpty"), 0x3a);
        assert_eq!(calc_checksum(b"qTStatus"), 0x49);
    }
}