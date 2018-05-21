use std::collections::HashSet;
use core::AvrVm;
use core::CpuSignal;
use instruction_set::Instruction;
use tools::objdump::ObjDumpInstr;

pub struct AvrDebugger {
    /// code breakpoints
    hw_breakpoints: HashSet<usize>,

    pub trace: bool
}

impl AvrDebugger {
    pub fn new() -> AvrDebugger {
        AvrDebugger {
            hw_breakpoints: HashSet::new(),
            trace: false
        }
    }

    pub fn add_breakpoint(&mut self, pos: usize) {
        self.hw_breakpoints.insert(pos);
    }

    pub fn remove_breakpoint(&mut self, pos: usize) {
        self.hw_breakpoints.insert(pos);
    }

    pub fn pre_instr_hook(&self, vm: &AvrVm, instr: &Instruction) -> Result<(), CpuSignal> {
        if self.hw_breakpoints.contains(&vm.pc) {
            return Err(CpuSignal::Break)
        }

        if self.trace {
            println!("{:06x}: {}\t{:x}", vm.pc * 2, instr.dump(), vm.sp);
        }

        Ok(())
    }
}