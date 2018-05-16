use std::collections::HashSet;
use core::AvrVm;
use core::CpuSignal;

pub struct AvrDebugger {
    /// code breakpoints
    hw_breakpoints: HashSet<usize>
}

impl AvrDebugger {
    pub fn new() -> AvrDebugger {
        AvrDebugger {
            hw_breakpoints: HashSet::new()
        }
    }

    pub fn add_breakpoint(&mut self, pos: usize) {
        self.hw_breakpoints.insert(pos);
    }

    pub fn remove_breakpoint(&mut self, pos: usize) {
        self.hw_breakpoints.insert(pos);
    }

    pub fn pre_instr_hook(&self, vm: &AvrVm) -> Result<(), CpuSignal> {
        if self.hw_breakpoints.contains(&vm.pc) {
            Err(CpuSignal::Break)
        } else {
            Ok(())
        }
    }
}