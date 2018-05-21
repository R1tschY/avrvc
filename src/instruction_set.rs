
use core::AvrVm;
use core::CpuSignal;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Instruction {
    Adc { d: u8, r: u8 },
    Add { d: u8, r: u8 },
    Adiw { d: u8, k: u8 },
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

fn set_zns16(state: &mut AvrVm, res: u16) {
    state.n = (res >> 15) != 0;
    state.zero = res == 0;
    state.sign = state.n ^ state.v;
}

impl Instruction {

    /// execute instruction
    ///
    /// no checks on state are done!
    pub fn execute(&self, state: &mut AvrVm) -> Result<(), CpuSignal> {
        use instruction_set::Instruction::*;

        state.pc += 1;
        state.cycles += 1;

        match self {
            &Adc { d, r } => {
                let r = state.read_reg(d) as u16 + state.read_reg(r) as u16 + state.carry as u16;
                state.carry = r > 0xFF;
                // TODO: HV
                set_zns(state, r as u8);
                state.write_reg(d, (r & 0xFF) as u8);
            },
            &Add { d, r } => {
                let r = state.read_reg(d) as u16 + state.read_reg(r) as u16;
                state.carry = r > 0xFF;
                // TODO: HV
                set_zns(state, r as u8);
                state.write_reg(d, (r & 0xFF) as u8);
            },
            &Adiw { d, k } => {
                let rd = state.read_reg16(d) as u32;
                let r = rd + k as u32;
                state.carry = r > 0xFFFF;
                state.v = ((!rd & r) & 0x8000) != 0;
                set_zns16(state, r as u16);
                state.write_reg16(d, (r & 0xFFFF) as u16);
                state.cycles += 1;
            },

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
                let io = state.read_mem(a as usize);
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
                state.write_mem(a as usize, reg);
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
                    state.crash(CpuSignal::PcOutOfBounds {
                        pc: new_pc as i32
                    })?;
                } else {
                    state.pc = new_pc as usize;
                    state.cycles += 2;
                }
            },


            &Instruction::Invaild { opcode } => {
                state.cycles -= 1;
                state.crash(CpuSignal::InvaildOpcode { opcode })?;
            }
        }

        Ok(())
    }

    /// size in words
    pub fn size(&self) -> usize {
        match self {
            &Instruction::Jmp { .. } | &Instruction::Call { .. } => 2,
            _ => 1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::AvrVmInfo;
    use models::xmega_au::XmegaA4U::ATxmega128A4U;
    use models::AvrModel;

    #[test]
    fn execute_call_xmega() {
        let mut vm = ATxmega128A4U.create_vm();
        let old_sp = vm.sp;
        vm.pc = 0xAABBCC;

        let cmd = Instruction::Call { k: 0x1337 };
        cmd.execute(&mut vm).unwrap();

        assert_eq!(vm.sp, old_sp - 3);
        assert_eq!(vm.read_mem(old_sp - 0), 0xCE);
        assert_eq!(vm.read_mem(old_sp - 1), 0xBB);
        assert_eq!(vm.read_mem(old_sp - 2), 0xAA);
        assert_eq!(vm.pc, 0x1337);
        assert_eq!(vm.cycles, 4);
    }

    #[test]
    fn execute_jmp() {
        let mut vm = ATxmega128A4U.create_vm();

        let cmd = Instruction::Jmp { k: 0x1337 };
        cmd.execute(&mut vm).unwrap();

        assert_eq!(vm.pc, 0x1337);
        assert_eq!(vm.cycles, 3);
    }

    #[test]
    fn execute_rjmp() {
        let mut vm = ATxmega128A4U.create_vm();
        vm.pc = 1000;

        let cmd = Instruction::Rjmp { k: -5 };
        cmd.execute(&mut vm).unwrap();

        assert_eq!(vm.pc, 1000 - 5 + 1);
        assert_eq!(vm.cycles, 3);
    }

    #[test]
    fn execute_ret() {
        let mut vm = ATxmega128A4U.create_vm();

        vm.sp -= 3;
        let sp = vm.sp;
        vm.write_mem(sp + 1, 0xAAu8);
        vm.write_mem(sp + 2, 0xBBu8);
        vm.write_mem(sp + 3, 0xCCu8);

        let cmd = Instruction::Ret;
        cmd.execute(&mut vm).unwrap();

        assert_eq!(vm.pc, 0xAABBCC);
        assert_eq!(vm.cycles, 5);
    }

    #[test]
    fn execute_in() {
        let mut vm = ATxmega128A4U.create_vm();

        vm.write_mem(33, 0x42u8);

        let cmd = Instruction::In { d: 26, a: 33 };
        cmd.execute(&mut vm).unwrap();

        assert_eq!(vm.read_reg(26), 0x42u8);
        assert_eq!(vm.cycles, 1);
    }

    #[test]
    fn execute_out() {
        let mut vm = ATxmega128A4U.create_vm();

        vm.write_reg(30, 0x76u8);

        let cmd = Instruction::Out { r: 30, a: 15 };
        cmd.execute(&mut vm).unwrap();

        assert_eq!(vm.read_mem(15), 0x76u8);
        assert_eq!(vm.cycles, 1);
    }

    #[test]
    fn execute_ldi() {
        let mut vm = ATxmega128A4U.create_vm();

        let cmd = Instruction::Ldi { d: 17, k: 42 };
        cmd.execute(&mut vm).unwrap();

        assert_eq!(vm.read_reg(17), 42);
    }
}