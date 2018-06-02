use bytes::Bytes;

#[derive(Debug)]
pub enum GdbServerPkt {
    Ack { okay: bool },
    Packet(Bytes),
    CtrlC
}

pub mod codec;
pub mod commands;
pub mod debugger;
pub mod server;
pub mod tokio_ext;

pub use gdb::server::serve;
pub use gdb::commands::GdbCommands;
pub use gdb::debugger::GdbDebugger;
pub use gdb::tokio_ext::TcpListenerExt;

