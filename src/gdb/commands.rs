use std::collections::HashMap;
use std::str;

use bytes::Bytes;

use gdb::debugger::GdbDebugger;
use gdb::GdbServerPkt;
use bytes::BytesMut;


type GdbRemoteCommand = Box<fn(pkt: &Bytes, dbg: &mut GdbDebugger) -> Option<Bytes>>;


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

fn q_commands(pkt: &Bytes, _dbg: &mut GdbDebugger) -> Option<Bytes> {
    let (name, _args) = pkt.split_at(pkt.iter().position(|&b| b == b':').unwrap_or(pkt.len()));

    match name {
        b"qSupported" => Some(Bytes::from_static(b"qXfer:memory-map:read+")),
        b"qAttached" => Some(Bytes::from_static(b"1")),
        b"Xfer:memory-map:read" => {
            Some(Bytes::from(format!(
                "l<memory-map>
                    <memory type='ram' start='0x800000' length='{:#x}'/>
                    <memory type='flash' start='0' length='{:#x}'>
                        <property name='blocksize'>0x80</property>
                    </memory>
                </memory-map>", 0, 0)))
        },
        _ => Some(Bytes::new())
    }
}


fn v_commands(_pkt: &Bytes, _dbg: &mut GdbDebugger) -> Option<Bytes> {
    Some(Bytes::new())
}

fn question_mark_command(_pkt: &Bytes, dbg: &mut GdbDebugger) -> Option<Bytes> {
    Some(dbg.signal_reply())
}

fn g_command(_pkt: &Bytes, dbg: &mut GdbDebugger) -> Option<Bytes> {
    let mut bytes = BytesMut::with_capacity(128);
    for i in 0..35 {
        dbg.read_register(i, &mut bytes);
    }
    Some(bytes.freeze())
}

fn s_command(pkt: &Bytes, dbg: &mut GdbDebugger) -> Option<Bytes> {
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

