use std::collections::HashMap;
use bytes::Bytes;

use gdb::debugger::GdbDebugger;
use gdb::GdbServerPkt;
use bytes::{BufMut, BytesMut};


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

            &GdbServerPkt::Ack { okay } => None, // TODO
            &GdbServerPkt::CtrlC => None, // TODO
        }
    }
}

// Helper

fn signal_reply(dbg: &GdbDebugger) -> Bytes {
    Bytes::from(format!("S{:02x}", dbg.get_signal()))
}

fn read_register(reg: u32, dbg: &GdbDebugger, bytes: &mut BytesMut) {
    match reg {
        0...31 => bytes.put(format!("{:02x}", dbg.vc.core.read_reg(reg as u8))),
        32 => bytes.put(format!("{:02x}", 0x00)), // TODO
        33 => bytes.put(format!("{:04x}", dbg.vc.core.sp)), // TODO: sp as LE
        34 => bytes.put(format!("{:08x}", dbg.vc.core.pc)), // TODO: pc as LE
        _ => () // TODO: Error
    }
}

// Commands

fn q_commands(pkt: &Bytes, dbg: &mut GdbDebugger) -> Option<Bytes> {
    let (name, args) = pkt.split_at(pkt.iter().position(|&b| b == b':').unwrap_or(pkt.len()));

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


fn v_commands(pkt: &Bytes, dbg: &mut GdbDebugger) -> Option<Bytes> {
    Some(Bytes::new())
}

fn question_mark_command(pkt: &Bytes, dbg: &mut GdbDebugger) -> Option<Bytes> {
    Some(signal_reply(dbg))
}

fn g_command(pkt: &Bytes, dbg: &mut GdbDebugger) -> Option<Bytes> {
    let mut bytes = BytesMut::with_capacity(128);
    for i in 0..35 {
        read_register(i, dbg, &mut bytes);
    }
    Some(bytes.freeze())
}

