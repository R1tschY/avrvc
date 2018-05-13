use std::collections::HashSet;
use core::AvrVm;

pub struct AvrDebugger {
    /// code breakpoints
    breakpoints: HashSet<usize>
}

impl AvrDebugger {
    pub fn new() -> AvrDebugger {
        AvrDebugger {
            breakpoints: HashSet::new()
        }
    }

    pub fn add_breakpoint(&mut self, pos: usize) {
        self.breakpoints.insert(pos);
    }

    pub fn remove_breakpoint(&mut self, pos: usize) {
        self.breakpoints.insert(pos);
    }

    pub fn instr_hook(&self, vm: &mut AvrVm) {
//        vm.
    }
}