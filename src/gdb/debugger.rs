use std::sync::Arc;
use std::sync::Mutex;
use bytes::Bytes;


#[derive(Copy, Clone, PartialEq)]
pub enum DebuggerState {
    Running,
    Stopped
}

pub struct GdbDebugger {
    state: DebuggerState,
}

impl GdbDebugger {
    pub fn new() -> Self {
        GdbDebugger {
            state: DebuggerState::Stopped,
        }
    }

    pub fn get_state(&self) -> DebuggerState {
        self.state
    }

    pub fn set_state(&mut self, value: DebuggerState) {
        self.state = value;
    }

    pub fn istep(&mut self) {

    }
}