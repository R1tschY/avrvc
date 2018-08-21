use std::str;

use bytes::Bytes;

use gdb::debugger::GdbDebugger;
use gdb::GdbServerPkt;
use bytes::BytesMut;
use hex;
use std::ops::Range;
use std::num;
use std::str::Utf8Error;
use core::AccessError;
use gdb::server::Tx;
use futures::Sink;
use gdb::debugger::DebuggerState;

// Constants

static RAM_OFFSET: usize = 0x800000;


// Types

#[derive(Debug, Clone, PartialEq)]
enum GdbError {
    InvalidFormat(&'static str),
    InvaildHex(hex::FromHexError),
    ParseIntError(num::ParseIntError),
    NonAsciiChar,

    AccessError(AccessError),
}

impl From<num::ParseIntError> for GdbError {
    fn from(error: num::ParseIntError) -> Self { GdbError::ParseIntError(error) }
}

impl From<hex::FromHexError> for GdbError {
    fn from(error: hex::FromHexError) -> Self { GdbError::InvaildHex(error) }
}

impl From<Utf8Error> for GdbError {
    fn from(_error: Utf8Error) -> Self { GdbError::NonAsciiChar }
}

impl From<AccessError> for GdbError {
    fn from(error: AccessError) -> Self { GdbError::AccessError(error) }
}

pub struct GdbCommands {
    tx: Tx
}

impl GdbCommands {
    pub fn new(tx: Tx) -> GdbCommands {
        GdbCommands { tx }
    }

    pub fn handle(&mut self, pkt: &GdbServerPkt, dbg: &mut GdbDebugger) -> Option<GdbServerPkt> {
        match pkt {
            &GdbServerPkt::Packet(ref bytes) =>
                self.handle_packet(&bytes, dbg).map(|reply| GdbServerPkt::Packet(reply)),

            &GdbServerPkt::Ack { .. } => None, // TODO
            &GdbServerPkt::CtrlC => None, // TODO
        }
    }

    fn convert_result(&mut self, pkt: &[u8], result: Result<Bytes, GdbError>) -> Option<Bytes> {
        match result {
            Err(error) => {
                error!(
                    "error {:?} while processing {}",
                    error,
                    str::from_utf8(pkt).unwrap_or("<invalid utf-8>"));
                Some(Bytes::from_static(b"E01"))
            },
            Ok(bytes) => Some(bytes)
        }
    }

    fn convert_silent_result(&mut self, pkt: &[u8], result: Result<(), GdbError>) -> Option<Bytes> {
        match result {
            Err(error) => self.convert_result(pkt, Err(error)),
            Ok(()) => None
        }
    }

    #[allow(unused)]
    fn send_output(&mut self, msg: &str) {
        self.tx.start_send(
            GdbServerPkt::Packet(
                Bytes::from(format!("O{}", hex::encode(Bytes::from(msg)))))
        ).unwrap();
    }

    fn handle_packet(&mut self, pkt: &[u8], dbg: &mut GdbDebugger) -> Option<Bytes> {
        let key = pkt[0];
        let data = &pkt[1..];
        match key {
            b'c' => self.convert_silent_result(pkt, continue_command(data, dbg)),
            b'D' => detach_command(pkt, dbg),
            b'k' => kill_command(pkt, dbg),
            b'g' => g_command(data, dbg),
            b'q' => query_commands(data, dbg),
            b'?' => Some(dbg.signal_reply()),
            b's' => s_command(data, dbg),
            b'm' => self.convert_result(pkt, memread_command(data, dbg)),
            b'M' => self.convert_result(pkt, memwrite_command(data, dbg)),
            _ => Some(Bytes::new()),
        }
    }
}

// Parse helper

fn parse_start_length(pkt: &[u8]) -> Result<Range<usize>, GdbError> {
    let pkt = str::from_utf8(pkt)?;
    let args: Vec<&str> = pkt.split(',').collect();
    if args.len() == 2 {
        let addr = usize::from_str_radix(args[0], 16)?;
        let length = usize::from_str_radix(args[1], 16)?;
        Ok(addr..addr + length)
    } else {
        Err(GdbError::InvalidFormat("multiple commas in range"))
    }
}


// Commands

fn continue_command(pkt: &[u8], dbg: &mut GdbDebugger) -> Result<(), GdbError> {
    if pkt.is_empty() {
        dbg.set_state(DebuggerState::Running);
    } else {
        let pkt = str::from_utf8(pkt)?;
        let addr = usize::from_str_radix(pkt, 16)?;
        dbg.vm.core.pc = addr;
        dbg.set_state(DebuggerState::Running);
    }

    Ok(())
}

fn detach_command(_pkt: &[u8], dbg: &mut GdbDebugger) -> Option<Bytes> {
    dbg.set_state(DebuggerState::Detached);
    Some(Bytes::from_static(b"OK"))
}

fn kill_command(_pkt: &[u8], dbg: &mut GdbDebugger) -> Option<Bytes> {
    dbg.set_state(DebuggerState::Killed);
    Some(Bytes::from_static(b"OK"))
}

//fn read_io_regs(pkt: &[u8], dbg: &mut GdbDebugger) -> Option<Bytes> {
//    if pkt.is_empty() {
//        // FIXME: "info io_register" endless requests this
//        Some(Bytes::from(format!("{:02x}", dbg.vm.info.io_regs.len())))
//    } else {
//        if let Ok(range) = parse_start_length(&pkt[1..]) {
//            let mut io_regs: Vec<(&&str, &usize)> = dbg.vm.info.io_regs.iter().collect();
//            io_regs.sort_by_key(|e| *e.1);
//            let answer = io_regs[range]
//                .iter()
//                .map(|e| format!("{},{:02x};", *e.0, dbg.vm.read_io(*e.1)))
//                .join("");
//            Some(Bytes::from(answer))
//        } else {
//            Some(Bytes::from_static(b"E01"))
//        }
//    }
//}

fn query_commands(pkt: &[u8], dbg: &mut GdbDebugger) -> Option<Bytes> {
    match pkt {
        b"Supported" => Some(Bytes::from_static(b"qXfer:memory-map:read+")),
        b"Attached" => Some(Bytes::from_static(b"1")),
        // _ if pkt.starts_with(b"Ravr.io_reg") => read_io_regs(&pkt[b"Ravr.io_reg".len()..], dbg),
        _ if pkt.starts_with(b"Xfer:memory-map:read::") =>
            Some(Bytes::from(format!(
                "l<memory-map>
                    <memory type='ram' start='{:#x}' length='{:#x}'/>
                    <memory type='flash' start='0' length='{:#x}'>
                        <property name='blocksize'>0x80</property>
                    </memory>
                </memory-map>",
                RAM_OFFSET,
                dbg.vm.info.ram.end,
                dbg.vm.info.flash_bytes))),
        _ => Some(Bytes::new())
    }
}


fn g_command(_pkt: &[u8], dbg: &mut GdbDebugger) -> Option<Bytes> {
    let mut bytes = BytesMut::with_capacity(128);
    for i in 0..35 {
        dbg.read_register(i, &mut bytes);
    }
    Some(bytes.freeze())
}

fn memread_command(pkt: &[u8], dbg: &mut GdbDebugger) -> Result<Bytes, GdbError> {
    let Range { start, end } = parse_start_length(pkt)?;

    if end < dbg.vm.info.flash_bytes {
        Ok(Bytes::from(hex::encode(&dbg.vm.core.flash[start..end])))
    } else if start >= RAM_OFFSET {
        let memstart = start - RAM_OFFSET;
        let memend = end - RAM_OFFSET;
        let memory: Vec<u8> = (memstart..memend)
            .map(|addr| dbg.vm.read(addr, true).map(|(res, _)| res).unwrap_or(0)).collect();

        Ok(Bytes::from(hex::encode(&memory)))
    } else {
        Err(GdbError::from(AccessError::ReadError(start)))
    }
}

fn write_memory(dbg: &mut GdbDebugger, addr: usize, bytes: &[u8]) -> Result<(), GdbError> {
    if addr < dbg.vm.info.flash_bytes {
        dbg.vm.write_flash(addr, bytes);
        Ok(())
    } else if addr >= RAM_OFFSET {
        let memaddr = addr - RAM_OFFSET;
        for addr in memaddr..memaddr + bytes.len() {
            dbg.vm.write(addr, bytes[addr - memaddr])?;
        }
        Ok(())
    } else {
        Err(GdbError::from(AccessError::WriteError(addr)))
    }
}

fn memwrite_command(pkt: &[u8], dbg: &mut GdbDebugger) -> Result<Bytes, GdbError> {
    if let Some(colon) = pkt.iter().position(|&b| b == b':') {
        let (range, bytes) = pkt.split_at(colon);
        let range = parse_start_length(range)?;
        let binary = hex::decode(&bytes[1..])?;
        if binary.len() == range.len() {
            write_memory(dbg, range.start, &binary).map(|_| Bytes::from_static(b"OK"))
        } else {
            Err(GdbError::InvalidFormat("M command: length of data is inconsistent"))
        }
    } else {
        Err(GdbError::InvalidFormat("M command: missing colon"))
    }
}

fn s_command(pkt: &[u8], dbg: &mut GdbDebugger) -> Option<Bytes> {
    if pkt.len() > 0 {
        if let Ok(addr_str) = str::from_utf8(pkt) {
            if let Ok(addr_int) = usize::from_str_radix(addr_str, 16) {
                dbg.vm.core.pc = addr_int;
                dbg.step();
                return Some(dbg.signal_reply());
            }
        }
    } else {
        dbg.step();
        return Some(dbg.signal_reply());
    }

    None // silent error
}

