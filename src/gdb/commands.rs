use std::collections::HashMap;
use bytes::Bytes;

use gdb::debugger::GdbDebugger;


type GdbRemoteCommand = Box<fn(pkt: &Bytes, dbg: &mut GdbDebugger) -> Option<Bytes>>;


pub struct GdbCommands {
    commands: HashMap<u8, GdbRemoteCommand>
}

impl GdbCommands {
    pub fn new() -> GdbCommands {
        let mut registry: HashMap<u8, GdbRemoteCommand> = HashMap::new();
        registry.insert(b'q', Box::new(q_commands));

        GdbCommands {
            commands: registry
        }
    }

    pub fn handle(&self, bytes: &Bytes, dbg: &mut GdbDebugger) -> Option<Bytes> {
        let key = bytes[0];
        self.commands.get(&key).and_then(
            |command| command(bytes, dbg))
    }
}


fn q_commands(pkt: &Bytes, dbg: &mut GdbDebugger) -> Option<Bytes> {
    if pkt.to_vec() == b"qSupported" || pkt.starts_with(b"qSupported:") {
        return Some(Bytes::from_static(b"qXfer:memory-map:read+"))
    }

    None
}