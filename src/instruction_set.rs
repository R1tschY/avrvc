
use core::AvrVm;
use core::Crash;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Instruction {
    Call { k: usize },
    Cli,
    Eor { d: u8, r: u8 },
    Ldi { d: u8, k: u8 },
    In  { d: u8, a: u8 },
    Jmp { k: usize },
    Out { r: u8, a: u8 },
    Pop { r: u8 },
    Push { r: u8 },
    Ret,
    Rjmp { k: i16 },

    Invaild { opcode: u16 }
}

fn set_zns(state: &mut AvrVm, res: u8) {
    state.n = (res >> 7) != 0;
    state.zero = res == 0;
    state.sign = state.n ^ state.v;
}

impl Instruction {

    /// execute instruction
    ///
    /// no checks on state are done!
    pub fn execute(&self, state: &mut AvrVm) {
        state.pc += 1;
        state.cycles += 1;

        match self {
            &Instruction::Call { k } => {
                let pc = state.pc;
                if state.info.pc_bytes == 3 {
                    state.push3((pc + 1) as u32);
                    if state.info.xmega {
                        state.cycles += 3;
                    } else {
                        state.cycles += 4;
                    }
                } else {
                    state.push2((pc + 1) as u16);
                    if state.info.xmega {
                        state.cycles += 2;
                    } else {
                        state.cycles += 3;
                    }
                }
                state.pc = k;
            },

            &Instruction::Cli => {
                state.interrupt = false;
            },

            &Instruction::Eor { d, r } => {
                let r: u8 = state.read_reg(d) ^ state.read_reg(r);
                state.write_reg(d, r);
                state.v = false;
                set_zns(state, r);
            },

            &Instruction::In { d, a } => {
                let io = state.read_io(a);
                state.write_reg(d, io);
            },

            &Instruction::Jmp { k } => {
                state.pc = k;
                state.cycles += 2;
            },

            &Instruction::Ldi { d, k } => {
                state.write_reg(d, k);
            },

            &Instruction::Out { r, a } => {
                let reg = state.read_reg(r);
                state.write_io(a, reg);
            },

            &Instruction::Pop { r } => {
                let reg = state.pop();
                state.write_reg(r, reg);
                state.cycles += 1;
            },

            &Instruction::Push { r } => {
                let reg = state.read_reg(r);
                state.push(reg);
                if !state.info.xmega {
                    state.cycles += 1;
                }
            },

            &Instruction::Ret => {
                if state.info.pc_bytes == 3 {
                    state.pc = state.pop3() as usize;
                    state.cycles += 4;
                } else {
                    state.pc = state.pop2() as usize;
                    state.cycles += 3;
                }
            },

            &Instruction::Rjmp { k } => {
                let new_pc = state.pc as i32 + k as i32;
                if new_pc < 0 {
                    state.crash(Crash::PcOutOfBounds {
                        pc: new_pc as i32
                    });
                } else {
                    state.pc = new_pc as usize;
                    state.cycles += 2;
                }
            },


            &Instruction::Invaild { opcode } => {
                state.cycles -= 1;
                state.crash(Crash::InvaildOpcode { opcode })
            }
        }
    }

    /// size in bytes
    pub fn size(&self) -> usize {
        match self {
            &Instruction::Jmp { .. } | &Instruction::Call { .. } => 4,
            _ => 2
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::AvrVmInfo;

    #[test]
    fn execute_call_xmega() {
        let info = AvrVmInfo { pc_bytes: 3, xmega: true, flash_bytes: 100, memory_bytes: 200 };
        let mut vm = AvrVm::new(&info);
        vm.sp = 100 - 1;
        vm.pc = 0xAABBCC;

        let cmd = Instruction::Call { k: 0x1337 };
        cmd.execute(&mut vm);

        assert_eq!(vm.sp, 100 - 4);
        assert_eq!(vm.memory[100 - 1], 0xCE);
        assert_eq!(vm.memory[100 - 2], 0xBB);
        assert_eq!(vm.memory[100 - 3], 0xAA);
        assert_eq!(vm.pc, 0x1337);
        assert_eq!(vm.cycles, 4);
    }

    #[test]
    fn execute_call() {
        let info = AvrVmInfo { pc_bytes: 2, xmega: false, flash_bytes: 100, memory_bytes: 200 };
        let mut vm = AvrVm::new(&info);
        vm.sp = 100 - 1;
        vm.pc = 0xAABB;

        let cmd = Instruction::Call { k: 0x1337 };
        cmd.execute(&mut vm);

        assert_eq!(vm.sp, 100 - 3);
        assert_eq!(vm.memory[100 - 1], 0xBD);
        assert_eq!(vm.memory[100 - 2], 0xAA);
        assert_eq!(vm.pc, 0x1337);
        assert_eq!(vm.cycles, 4);
    }

    #[test]
    fn execute_jmp() {
        let info = AvrVmInfo { pc_bytes: 2, xmega: false, flash_bytes: 100, memory_bytes: 200 };
        let mut vm = AvrVm::new(&info);

        let cmd = Instruction::Jmp { k: 0x1337 };
        cmd.execute(&mut vm);

        assert_eq!(vm.pc, 0x1337);
        assert_eq!(vm.cycles, 3);
    }

    #[test]
    fn execute_rjmp() {
        let info = AvrVmInfo { pc_bytes: 2, xmega: false, flash_bytes: 100, memory_bytes: 200 };
        let mut vm = AvrVm::new(&info);
        vm.pc = 1000;

        let cmd = Instruction::Rjmp { k: -5 };
        cmd.execute(&mut vm);

        assert_eq!(vm.pc, 1000 - 5 + 1);
        assert_eq!(vm.cycles, 3);
    }

    #[test]
    fn execute_ret() {
        let info = AvrVmInfo { pc_bytes: 3, xmega: false, flash_bytes: 100, memory_bytes: 200 };
        let mut vm = AvrVm::new(&info);
        vm.memory[100 - 3] = 0xAAu8;
        vm.memory[100 - 2] = 0xBBu8;
        vm.memory[100 - 1] = 0xCCu8;
        vm.sp = 100 - 4;

        let cmd = Instruction::Ret;
        cmd.execute(&mut vm);

        assert_eq!(vm.pc, 0xAABBCC);
        assert_eq!(vm.cycles, 5);
    }

    #[test]
    fn execute_in() {
        let info = AvrVmInfo { pc_bytes: 3, xmega: true, flash_bytes: 100, memory_bytes: 200 };
        let mut vm = AvrVm::new(&info);
        vm.memory[33] = 0x42u8;

        let cmd = Instruction::In { d: 26, a: 33 };
        cmd.execute(&mut vm);

        assert_eq!(vm.read_reg(26), 0x42u8);
        assert_eq!(vm.cycles, 1);
    }

    #[test]
    fn execute_out() {
        let info = AvrVmInfo { pc_bytes: 3, xmega: true, flash_bytes: 100, memory_bytes: 200 };
        let mut vm = AvrVm::new(&info);
        vm.write_reg(30, 0x76u8);

        let cmd = Instruction::Out { r: 30, a: 15 };
        cmd.execute(&mut vm);

        assert_eq!(vm.read_io(15), 0x76u8);
        assert_eq!(vm.cycles, 1);
    }

    #[test]
    fn execute_ldi() {
        let info = AvrVmInfo { pc_bytes: 2, xmega: false, flash_bytes: 100, memory_bytes: 200 };
        let mut vm = AvrVm::new(&info);

        let cmd = Instruction::Ldi { d: 17, k: 42 };
        cmd.execute(&mut vm);

        assert_eq!(vm.read_reg(17), 42);
    }
}