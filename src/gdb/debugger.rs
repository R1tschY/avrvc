use bytes::Bytes;
use controller::AvrController;
use core::AvrVm;
use core::AvrVmInfo;
use core::CpuSignal;
use bytes::{BytesMut, BufMut};


/// Illegal instruction
pub const SIGILL: u32 = 4;

/// Trace/breakpoint trap
pub const SIGTRAP: u32 = 5;


#[derive(Copy, Clone, PartialEq)]
pub enum DebuggerState {
    Running,
    Stopped
}

pub struct GdbDebugger {
    pub vc: AvrController,
    state: DebuggerState,
    last_signal: u32
}

impl GdbDebugger {
    pub fn new(vm: AvrVm) -> Self {
        GdbDebugger {
            vc: AvrController { core: vm },
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

    /// step one cpu instruction
    ///
    /// returns `false` if signal raised while executing. See `last_signal` member.
    pub fn step(&mut self) -> bool {
        match self.vc.step() {
            Ok(_) => true,
            Err(signal) => {
                self.last_signal = self.get_signal_code(signal);
                false
            }
        }
    }

    pub fn signal_reply(&self) -> Bytes {
        Bytes::from(format!("S{:02x}", self.last_signal))
    }

    pub fn read_register(&self, reg: u32, bytes: &mut BytesMut) {
        match reg {
            0...31 => bytes.put(format!("{:02x}", self.vc.core.read_reg(reg as u8))),
            32 => bytes.put(format!("{:02x}", self.vc.core.read_sreg())),
            33 => bytes.put(format!("{:04x}", self.vc.core.sp)), // TODO: sp as LE
            34 => bytes.put(format!("{:08x}", self.vc.core.pc)), // TODO: pc as LE
            _ => () // TODO: Error
        }
    }

    fn get_signal_code(&self, signal: CpuSignal) -> u32 {
        match signal {
            CpuSignal::InvaildOpcode { .. } | CpuSignal::PcOutOfBounds { .. } => SIGILL,
            CpuSignal::Break => SIGTRAP
        }
    }
}