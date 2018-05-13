use gdb::package::GdbServerPkt;
use gdb::commands::CommandRegistry;
use gdb::debugger::GdbDebugger;
use std::io;

pub struct GdbServer {
    debugger: GdbDebugger,
    commands: CommandRegistry
}

impl GdbServer {

    pub fn new() -> Self {
        GdbServer {
            debugger: GdbDebugger { },
            commands: CommandRegistry::new()
        }
    }

    pub fn handle(&mut self, pkt: &GdbServerPkt, writer: &mut io::Write) {
        match pkt {
            &GdbServerPkt::Ack { okay } => { }, // ignore because we use TCP
            &GdbServerPkt::CtrlC => { }, // TODO
            &GdbServerPkt::Packet(ref bytes) => {
                writer.write_all(b"+");
                self.commands.handle(&bytes, &mut self.debugger);
            }
        }


    }

}