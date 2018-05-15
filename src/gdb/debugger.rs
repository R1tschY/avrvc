use std::sync::Arc;
use std::sync::Mutex;
use bytes::Bytes;
use state::AvrState;
use core::AvrVm;
use core::AvrVmInfo;


/// Trace/breakpoint trap
pub const SIGILL: u32 = 4;

/// Illegal instruction
pub const SIGTRAP: u32 = 5;


#[derive(Copy, Clone, PartialEq)]
pub enum DebuggerState {
    Running,
    Stopped
}

pub struct GdbDebugger {
    pub vc: AvrState,
    state: DebuggerState,
    last_signal: u32
}

impl GdbDebugger {
    pub fn new(info: &AvrVmInfo) -> Self {
        GdbDebugger {
            vc: AvrState { core: AvrVm::new(info) },
            state: DebuggerState::Stopped,
            last_signal: SIGTRAP
        }
    }

    pub fn get_state(&self) -> DebuggerState {
        self.state
    }

    pub fn set_state(&mut self, value: DebuggerState) {
        self.state = value;
    }

    pub fn get_signal(&self) -> u32 { self.last_signal }

    pub fn istep(&mut self) {

    }
}