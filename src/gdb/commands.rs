use std::collections::HashMap;

extern crate bytes;

use self::bytes::BytesMut;
use gdb::package::GdbServerPkt;
use gdb::debugger::GdbDebugger;


pub trait GdbCommandGroup {
    fn handle(&self, pkt: &BytesMut, dbg: &mut GdbDebugger) -> Vec<u8>;
}

pub struct CommandRegistry {
    commands: HashMap<u8, Box<GdbCommandGroup>>
}

impl CommandRegistry {
    pub fn new() -> CommandRegistry {
        let mut registry: HashMap<u8, Box<GdbCommandGroup>> = HashMap::new();
        registry.insert(b'q', Box::new(QCommands {}));


        CommandRegistry {
            commands: registry
        }
    }

    pub fn handle(&self, bytes: &BytesMut, dbg: &mut GdbDebugger) -> Vec<u8> {
        let key = bytes[0];
        match self.commands.get(&key) {
            Some(command_group) => command_group.handle(bytes, dbg),
            None => vec![]
        }
    }
}

#[derive(Copy, Clone)]
struct QCommands {}

impl GdbCommandGroup for QCommands {
    fn handle(&self, pkt: &BytesMut, dbg: &mut GdbDebugger) -> Vec<u8> {
        if pkt.to_vec() == b"qSupported" || pkt.starts_with(b"qSupported:") {
            return b"qXfer:memory-map:read+".to_vec()
        }

        vec![]
    }
}