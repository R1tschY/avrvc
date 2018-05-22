use std::collections::HashSet;
use core::AvrVm;
use core::CpuSignal;
use instruction_set::Instruction;
use tools::objdump::ObjDumpInstr;
use std::str::FromStr;
use std::ops::Add;

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
            let sreg = vm.read_sreg();
            let mut flags = String::new();
            let flags_chars: Vec<char> = "CZNVSHTI".chars().collect();
            for i in 0..8 {
                if (sreg & (1 << i)) != 0 {
                    flags.push(flags_chars[i]);
                }
            }

            println!(
                "{:06x} / {:>3} / {:>8}: {}",
                vm.pc * 2, vm.cycles, flags, instr.dump());
        }

        Ok(())
    }
}