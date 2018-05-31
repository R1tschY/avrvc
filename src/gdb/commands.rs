use std::collections::HashMap;
use std::str;

use bytes::Bytes;

use gdb::debugger::GdbDebugger;
use gdb::GdbServerPkt;
use bytes::BytesMut;
use hex;


type GdbRemoteCommand = Box<fn(pkt: &[u8], dbg: &mut GdbDebugger) -> Option<Bytes>>;


pub struct GdbCommands {
    commands: HashMap<u8, GdbRemoteCommand>
}

impl GdbCommands {
    pub fn new() -> GdbCommands {
        let mut registry: HashMap<u8, GdbRemoteCommand> = HashMap::new();
        registry.insert(b'g', Box::new(g_command));
        registry.insert(b'q', Box::new(q_commands));
        registry.insert(b'v', Box::new(v_commands));
        registry.insert(b'?', Box::new(question_mark_command));
        registry.insert(b's', Box::new(s_command));
        registry.insert(b'm', Box::new(m_command));

        GdbCommands {
            commands: registry
        }
    }

    pub fn handle(&self, pkt: &GdbServerPkt, dbg: &mut GdbDebugger) -> Option<GdbServerPkt> {
        match pkt {
            &GdbServerPkt::Packet(ref bytes) => {
                let key = bytes[0];

                if let Some(command) = self.commands.get(&key) {
                    command(bytes, dbg).map(|reply| GdbServerPkt::Packet(reply))
                } else {
                    // not supported command
                    Some(GdbServerPkt::Packet(Bytes::new()))
                }
            },

            &GdbServerPkt::Ack { .. } => None, // TODO
            &GdbServerPkt::CtrlC => None, // TODO
        }
    }
}


// Commands

fn q_commands(pkt: &[u8], dbg: &mut GdbDebugger) -> Option<Bytes> {
    match pkt {
        b"qSupported" => Some(Bytes::from_static(b"qXfer:memory-map:read+")),
        b"qAttached" => Some(Bytes::from_static(b"1")),
        _ if pkt.starts_with(b"qXfer:memory-map:read::") => {
            Some(Bytes::from(format!(
                "l<memory-map>
                    <memory type='ram' start='0x800000' length='{:#x}'/>
                    <memory type='flash' start='0' length='{:#x}'>
                        <property name='blocksize'>0x80</property>
                    </memory>
                </memory-map>", dbg.vc.core.info.ram.len(), dbg.vc.core.info.flash_bytes)))
        },
        _ => Some(Bytes::new())
    }
}


fn v_commands(_pkt: &[u8], _dbg: &mut GdbDebugger) -> Option<Bytes> {
    Some(Bytes::new())
}

fn question_mark_command(_pkt: &[u8], dbg: &mut GdbDebugger) -> Option<Bytes> {
    Some(dbg.signal_reply())
}

fn g_command(_pkt: &[u8], dbg: &mut GdbDebugger) -> Option<Bytes> {
    let mut bytes = BytesMut::with_capacity(128);
    for i in 0..35 {
        dbg.read_register(i, &mut bytes);
    }
    Some(bytes.freeze())
}

fn read_memory(dbg: &mut GdbDebugger, addr: usize, length: usize) -> Option<Bytes> {
    if addr < dbg.vc.core.info.flash_bytes {
        Some(Bytes::from(hex::encode(&dbg.vc.core.core.flash[addr..addr + length])))
    } else {
        error!(target: "gdb", "memory read error: start: {}, length: {}", addr, length);
        Some(Bytes::from_static(b"E01"))
    }
}

fn m_command(pkt: &[u8], dbg: &mut GdbDebugger) -> Option<Bytes> {
    if let Ok(pkt) = str::from_utf8(pkt) {
        let args: Vec<&str> = pkt[1..].split(',').collect();
        if args.len() == 2 {
            if let Ok(addr) = usize::from_str_radix(args[0], 16) {
                if let Ok(length) = usize::from_str_radix(args[1], 16) {
                    return read_memory(dbg, addr, length);
                }
            }
        }
    }

    Some(Bytes::from_static(b"E01"))
}

fn s_command(pkt: &[u8], dbg: &mut GdbDebugger) -> Option<Bytes> {
    let addr = &pkt[1..];
    if addr.len() > 0 {
        if let Ok(addr_str) = str::from_utf8(addr) {
            if let Ok(addr_int) = usize::from_str_radix(addr_str, 16) {
                dbg.vc.core.core.pc = addr_int;
                match dbg.step() {
                    _ => return Some(dbg.signal_reply()),
                };
            }
        }
    } else {
        match dbg.step() {
            _ => return Some(dbg.signal_reply()),
        };
    }

    None // silent error
}

