
use core::AvrVm;
use core::CpuSignal;
use bits::BitOps;
use bytes::LittleEndian;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum RegIncDec {
    Unchanged,
    Inc,
    Dec
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Instruction {
    Adc { d: u8, r: u8 },
    Add { d: u8, r: u8 },
    Adiw { d: u8, k: u8 },
    And { d: u8, r: u8 },
    Andi { d: u8, k: u8 },
    Break,
    Brcc { k: i8 },
    Brcs { k: i8 },
    Breq { k: i8 },
    Brge { k: i8 },
    Brhc { k: i8 },
    Brhs { k: i8 },
    Brid { k: i8 },
    Brie { k: i8 },
    Brlt { k: i8 },
    Brmi { k: i8 },
    Brne { k: i8 },
    Brpl { k: i8 },
    Brtc { k: i8 },
    Brts { k: i8 },
    Brvc { k: i8 },
    Brvs { k: i8 },
    Call { k: u32 },
    Cli,
    Com { d: u8 },
    Cp { d: u8, r: u8},
    Cpc { d: u8, r: u8},
    Cpi { d: u8, k: u8 },
    Dec { d: u8 },
    Elpm0,
    Elpm { d: u8 },
    ElpmInc { d: u8 },
    Eor { d: u8, r: u8 },
    LdX { d: u8, xop: RegIncDec },
    LdY { d: u8, yop: RegIncDec },
    LdZ { d: u8, zop: RegIncDec },
    LddY { q: u8, d: u8 },
    LddZ { q: u8, d: u8 },
    Ldi { d: u8, k: u8 },
    Lds { d: u8, k: u8 },
    Lds16 { d: u8, k: u16 },
    In  { d: u8, a: u8 },
    Inc { d: u8 },
    Jmp { k: u32 },
    Mov { d: u8, r: u8 },
    Movw { d: u8, r: u8 },
    Mul { d: u8, r: u8 },
    Muls { d: u8, r: u8 },
    Mulsu { d: u8, r: u8 },
    Nop,
    Or { d: u8, r: u8 },
    Ori { d: u8, k: u8 },
    Out { r: u8, a: u8 },
    Pop { r: u8 },
    Push { r: u8 },
    Ret,
    Rjmp { k: i16 },
    Sbc { d: u8, r: u8 },
    Sbci { d: u8, k: u8 },
    Sbiw { d: u8, k: u8 },
    Sbrc { r: u8, b: u8 },
    Sbrs { r: u8, b: u8 },
    StX { r: u8, xop: RegIncDec },
    StY { r: u8, yop: RegIncDec },
    StZ { r: u8, zop: RegIncDec },
    StdY { q: u8, r: u8 },
    StdZ { q: u8, r: u8 },
    Sts { r: u8, k: u8 },
    Sts16 { r: u8, k: u16 },
    Sub { d: u8, r: u8 },
    Subi { d: u8, k: u8 },

    Invaild { opcode: u16 }
}

use instruction_set::Instruction::*;
use decoder::AvrDecoder;
use byte_convert::u16le;
use core::DataMemoryType;
use byte_convert::bit_at;
use byte_convert::bit_at_u16;
use byte_convert::as_signed;
use byte_convert::as_unsigned;
use byte_convert::as_unsigned16;

fn set_zns(state: &mut AvrVm, res: u8) {
    state.core.n = (res >> 7) != 0;
    state.core.zero = res == 0;
    state.core.sign = state.core.n ^ state.core.v;
}

fn set_zns16(state: &mut AvrVm, res: u16) {
    state.core.n = (res >> 15) != 0;
    state.core.zero = res == 0;
    state.core.sign = state.core.n ^ state.core.v;
}

fn calc_v(rd: u8, rr: u8, r: u8) -> bool {
    (rd.is_bit_set(7) & !rr.is_bit_set(7) & !r.is_bit_set(7))
        | (!rd.is_bit_set(7) & rr.is_bit_set(7) & r.is_bit_set(7))
}

fn rjmp(vm: &mut AvrVm, k: i32) -> Result<(), CpuSignal> {
    let new_pc = vm.core.pc as i32 + k as i32;
    if new_pc < 0 {
        vm.crash(CpuSignal::PcOutOfBounds {
            pc: new_pc as i32
        })
    } else {
        vm.core.pc = new_pc as usize;
        vm.core.cycles += 1;
        Ok(())
    }
}

fn ldd(vm: &mut AvrVm, yz: u32, q: u8, d: u8) {
    let yzq = match vm.read(yz as usize + q as usize, false) {
        Ok((yzq, DataMemoryType::SRam)) => {
            if vm.info.xmega { vm.core.cycles += 1; }
            yzq
        },
        Ok((_, DataMemoryType::Eeprom)) => 0,
        Ok((yzq, _)) => yzq,
        Err(_) => 0,
    };
    vm.core.write_reg(d, yzq);
    if vm.info.xmega {
        vm.core.cycles += 1;
    } else {
        vm.core.cycles += 2;
    }
}

fn ld(vm: &mut AvrVm, xyz: u32, op: RegIncDec, d: u8) -> u32 {
    let mut xyz = xyz;

    // TODO: only change 8 bit or 16 bit if data space < 256B or < 64KB
    if op == RegIncDec::Dec { xyz -= 1 }

    let r = ld_read(vm, xyz as usize);
    vm.core.write_reg(d, r);

    // TODO: only change 8 bit or 16 bit if data space < 256B or < 64KB
    if op == RegIncDec::Inc { xyz += 1 }

    if vm.info.xmega {
        if op == RegIncDec::Dec { vm.core.cycles += 1 }
    } else {
        match op {
            RegIncDec::Unchanged => {},
            RegIncDec::Inc => vm.core.cycles += 1,
            RegIncDec::Dec => vm.core.cycles += 2,
        }
    }

    xyz
}

fn ld_read(vm: &mut AvrVm, addr: usize) -> u8 {
    match vm.read(addr, false) {
        Ok((q, DataMemoryType::SRam)) => {
            if vm.info.xmega { vm.core.cycles += 1; }
            q
        },
        Ok((_, DataMemoryType::Eeprom)) => 0,
        Ok((q, _)) => q,
        Err(_) => 0,
    }
}

fn st(vm: &mut AvrVm, xyz: u32, op: RegIncDec, r: u8) -> u32 {
    let mut xyz = xyz;

    // TODO: only change 8 bit or 16 bit if data space < 256B or < 64KB
    if op == RegIncDec::Dec { xyz -= 1 }

    let rr = vm.core.read_reg(r);
    vm.write_u8_noneeprom(xyz as usize, rr);

    // TODO: only change 8 bit or 16 bit if data space < 256B or < 64KB
    if op == RegIncDec::Inc { xyz += 1 }

    if vm.info.xmega || vm.info.tiny {
        if op == RegIncDec::Dec { vm.core.cycles += 1; }
    } else {
        vm.core.cycles += 1;
    }

    xyz
}

fn elpm(vm: &mut AvrVm, d: u8) -> u32 {
    let z = vm.core.read_ramped_z() as usize;
    let r = vm.core.flash[z];
    vm.core.write_reg(d, r);
    vm.core.cycles += 2;

    z as u32
}

impl Instruction {

    /// execute instruction
    ///
    /// no checks on state are done!
    pub fn execute(&self, state: &mut AvrVm) -> Result<(), CpuSignal> {
        state.core.pc += 1;
        state.core.cycles += 1;

        match self {
            &Adc { d, r } => {
                let rr = state.core.read_reg(d) as u16 + state.core.read_reg(r) as u16 + state.core.carry as u16;
                state.core.carry = rr > 0xFF;
                // TODO: HV
                set_zns(state, rr as u8);
                state.core.write_reg(d, (rr & 0xFF) as u8);
            },
            &Add { d, r } => {
                let rr = state.core.read_reg(d) as u16 + state.core.read_reg(r) as u16;
                state.core.carry = rr > 0xFF;
                // TODO: HV
                set_zns(state, rr as u8);
                state.core.write_reg(d, (rr & 0xFF) as u8);
            },
            &Adiw { d, k } => {
                let rd = state.core.read_reg16(d) as u32;
                let r = rd + k as u32;
                state.core.carry = r > 0xFFFF;
                state.core.v = ((!rd & r) & 0x8000) != 0;
                set_zns16(state, (r & 0xFFFF) as u16);
                state.core.write_reg16(d, (r & 0xFFFF) as u16);
                state.core.cycles += 1;
            },

            &And { d, r } => {
                let rr = state.core.read_reg(d) & state.core.read_reg(r);
                state.core.write_reg(d, rr);
                state.core.v = false;
                set_zns(state, rr);
            },
            &Andi { d, k } => {
                let rr = state.core.read_reg(d) & k;
                state.core.write_reg(d, rr);
                state.core.v = false;
                set_zns(state, rr);
            }

            &Break => return Err(CpuSignal::Break),

            &Brcc { k } => if !state.core.carry { return rjmp(state, k as i32); },
            &Brcs { k } => if state.core.carry { return rjmp(state, k as i32); },
            &Breq { k } => if state.core.zero { return rjmp(state, k as i32); },
            &Brge { k } => if !state.core.sign { return rjmp(state, k as i32); },
            &Brhc { k } => if !state.core.h { return rjmp(state, k as i32); },
            &Brhs { k } => if state.core.h { return rjmp(state, k as i32); },
            &Brid { k } => if !state.core.interrupt { return rjmp(state, k as i32); },
            &Brie { k } => if state.core.interrupt { return rjmp(state, k as i32); },
            &Brlt { k } => if state.core.sign { return rjmp(state, k as i32); },
            &Brmi { k } => if state.core.n { return rjmp(state, k as i32); },
            &Brne { k } => if !state.core.zero { return rjmp(state, k as i32); },
            &Brpl { k } => if !state.core.n { return rjmp(state, k as i32); },
            &Brtc { k } => if !state.core.t { return rjmp(state, k as i32); },
            &Brts { k } => if state.core.t { return rjmp(state, k as i32); },
            &Brvc { k } => if !state.core.v { return rjmp(state, k as i32); },
            &Brvs { k } => if state.core.v { return rjmp(state, k as i32); },

            &Call { k } => {
                let pc = state.core.pc;
                if state.info.pc_bytes == 3 {
                    state.push3((pc + 1) as u32);
                    if state.info.xmega {
                        state.core.cycles += 3;
                    } else {
                        state.core.cycles += 4;
                    }
                } else {
                    state.push2((pc + 1) as u16);
                    if state.info.xmega {
                        state.core.cycles += 2;
                    } else {
                        state.core.cycles += 3;
                    }
                }
                state.core.pc = k as usize;
            },

            &Cli => state.core.interrupt = false,

            &Com { d } => {
                let r = !state.core.read_reg(d);
                state.core.carry = true;
                state.core.v = false;
                set_zns(state, r);
            },

            &Cp { d, r } => {
                let rd = state.core.read_reg(d);
                let rr = state.core.read_reg(r);
                let r = rd - rr;
                // TODO: H flag
                state.core.carry = rr > rd;
                state.core.v = calc_v(rd, rr, r);
                set_zns(state, r);
            }

            &Cpc { d, r } => {
                let rd = state.core.read_reg(d);
                let rr = state.core.read_reg(r);
                let r = rd as i16 - rr as i16 - state.core.carry as i16;
                // TODO: H flag
                state.core.carry = r < 0;
                state.core.v = calc_v(rd, rr, (r & 0xFF) as u8);
                state.core.n = (r >> 7) != 0;
                state.core.zero = (r == 0) && state.core.zero;
                state.core.sign = state.core.n ^ state.core.v;
            }

            &Cpi { d, k } => {
                let rd = state.core.read_reg(d);
                let r = ((rd as i16 - k as i16) & 0xFF) as u8;
                // TODO: H flag
                state.core.carry = k > rd;
                state.core.v = calc_v(rd, k, r);
                set_zns(state, r);
            }

            &Dec { d } => {
                let rd = state.core.read_reg(d);
                let res = ((rd as i16 - 1i16) & 0xFF) as u8;
                state.core.write_reg(d, res);
                state.core.v = rd == 0x80;
                set_zns(state, res);
            }

            &Elpm0 => { elpm(state, 0); },
            &Elpm { d } => { elpm(state, d); },
            &ElpmInc { d } => {
                let z = elpm(state, d);
                state.core.write_ramped_z(z + 1);
            },

            &Eor { d, r } => {
                let r: u8 = state.core.read_reg(d) ^ state.core.read_reg(r);
                state.core.write_reg(d, r);
                state.core.v = false;
                set_zns(state, r);
            },

            &In { d, a } => {
                let io = state.read_io(a as usize, false);
                state.core.write_reg(d, io);
            },

            &Inc { d } => {
                let rd = state.core.read_reg(d);
                let res = ((rd as i16 + 1i16) & 0xFF) as u8;
                state.core.write_reg(d, res);
                state.core.v = rd == 0x7f;
                set_zns(state, res);
            }

            &Jmp { k } => {
                state.core.pc = k as usize;
                state.core.cycles += 2;
            },

            &Mov { d, r } => {
                let rr = state.core.read_reg(r);
                state.core.write_reg(d, rr);
            },

            &Movw { d, r } => {
                let rr = state.core.read_reg16(r);
                state.core.write_reg16(d, rr);
            },

            &Mul { d, r } => {
                let rr = state.core.read_reg(r) as u16;
                let rd = state.core.read_reg(d) as u16;
                let r = rr * rd;
                state.core.write_reg16(0, r);
                state.core.zero = r == 0;
                state.core.carry = bit_at_u16(r, 15);
                state.core.cycles += 1;
            }

            &Muls { d, r } => {
                let rr = as_signed(state.core.read_reg(r)) as i16;
                let rd = as_signed(state.core.read_reg(d)) as i16;
                let r = rr * rd;
                state.core.write_reg16(0, as_unsigned16(r));
                state.core.zero = r == 0;
                state.core.carry = bit_at_u16(as_unsigned16(r), 15);
                state.core.cycles += 1;
            }

            &Mulsu { d, r } => {
                let rr = state.core.read_reg(r) as i16;
                let rd = as_signed(state.core.read_reg(d)) as i16;
                let r = rr * rd;
                state.core.write_reg16(0, as_unsigned16(r));
                state.core.zero = r == 0;
                state.core.carry = bit_at_u16(as_unsigned16(r), 15);
                state.core.cycles += 1;
            }

            &Nop => { },

            &LdX { d, xop } => {
                let mut x = state.core.read_ramped_x();
                x = ld(state, x, xop, d);
                state.core.write_ramped_x(x);
            },

            &LdY { d, yop } => {
                let mut y = state.core.read_ramped_y();
                y = ld(state, y, yop, d);
                state.core.write_ramped_y(y);
            },

            &LdZ { d, zop } => {
                let mut z = state.core.read_ramped_z();
                z = ld(state, z, zop, d);
                state.core.write_ramped_z(z);
            },

            &LddY { q, d } => {
                let y = state.core.read_ramped_y();
                ldd(state, y, q, d);
            }
            &LddZ { q, d } => {
                let z = state.core.read_ramped_z();
                ldd(state, z, q, d);
            },

            &Ldi { d, k } => {
                state.core.write_reg(d, k);
            },

            &Lds16 { d, k } => {
                let addr = state.core.ramped_addr(k);
                let r = ld_read(state, addr);
                state.core.write_reg(d, r);
                state.core.cycles += 1;
                state.core.pc += 1;
            },

            &Lds { d, k } => {
                let r = state.read_u8_noneeprom(k as usize, false);
                state.core.write_reg(d, r);
            },

            &Or { d, r } => {
                let rr = state.core.read_reg(d) | state.core.read_reg(r);
                state.core.write_reg(d, rr);
                state.core.v = false;
                set_zns(state, rr);
            },
            &Ori { d, k } => {
                let rr = state.core.read_reg(d) | k;
                state.core.write_reg(d, rr);
                state.core.v = false;
                set_zns(state, rr);
            }

            &Out { r, a } => {
                let reg = state.core.read_reg(r);
                state.write_io(a as usize, reg);
            },

            &Pop { r } => {
                let reg = state.pop();
                state.core.write_reg(r, reg);
                state.core.cycles += 1;
            },

            &Push { r } => {
                let reg = state.core.read_reg(r);
                state.push(reg);
                if !state.info.xmega {
                    state.core.cycles += 1;
                }
            },

            &Ret => {
                if state.info.pc_bytes == 3 {
                    state.core.pc = state.pop3() as usize;
                    state.core.cycles += 4;
                } else {
                    state.core.pc = state.pop2() as usize;
                    state.core.cycles += 3;
                }
            },

            &Rjmp { k } => return rjmp(state, k as i32),

            &Sbc { d, r } => {
                let rd = state.core.read_reg(d);
                let rr = state.core.read_reg(r);
                let res = rd as i16 - rr as i16 - state.core.carry as i16;
                // TODO: HV
                state.core.carry = res < 0;
                state.core.v = calc_v(rd, rr, (res & 0xFF) as u8);
                state.core.n = (res >> 7) != 0;
                state.core.zero = (res == 0) && state.core.zero;
                state.core.sign = state.core.n ^ state.core.v;
                state.core.write_reg(d, (res & 0xFF) as u8);
            },
            &Sbci { d, k } => {
                let rd = state.core.read_reg(d);
                let res = rd as i16 - k as i16 - state.core.carry as i16;

                // TODO: H flag
                state.core.carry = res < 0;
                state.core.v = calc_v(rd, k, (res & 0xFF) as u8);
                state.core.n = (res >> 7) != 0;
                state.core.zero = (res == 0) && state.core.zero;
                state.core.sign = state.core.n ^ state.core.v;
                state.core.write_reg(d, (res & 0xFF) as u8);
            }

            &Sbiw { d, k } => {
                let rd = state.core.read_reg16(d) as u32;
                let r = rd - k as u32;
                state.core.carry = r > 0xFFFF;
                state.core.v = ((!rd & r) & 0x8000) != 0;
                set_zns16(state, r as u16);
                state.core.write_reg16(d, (r & 0xFFFF) as u16);
                state.core.cycles += 1;
            }

            &Sbrc { r, b } | &Sbrs { r, b } => {
                let rr = state.core.read_reg(r);
                let target = if let &Sbrs { .. } = self { true } else { false };
                if (rr & (1 << b) != 0) == target {
                    let instr16 = AvrDecoder::is_2word_instruction(u16le(
                        state.core.flash[state.core.pc], state.core.flash[state.core.pc + 1]
                    ));
                    state.core.pc += 1 + instr16 as usize;
                    state.core.cycles += 1 + instr16 as u64;
                }
            }

            &StX { r, xop } => {
                let mut x = state.core.read_ramped_x();
                x = st(state, x, xop, r);
                state.core.write_ramped_x(x);
            },

            &StY { r, yop } => {
                let mut y = state.core.read_ramped_y();
                y = st(state, y, yop, r);
                state.core.write_ramped_y(y);
            },

            &StZ { r, zop } => {
                let mut z = state.core.read_ramped_z();
                z = st(state, z, zop, r);
                state.core.write_ramped_z(z);
            },

            &StdY { q, r } => {
                let y = state.core.read_ramped_y();
                let rr = state.core.read_reg(r);
                state.write_u8_noneeprom(y as usize + q as usize, rr);
                if !state.info.xmega && !state.info.tiny {
                    state.core.cycles += 1;
                }
            },

            &StdZ { q, r } => {
                let z = state.core.read_ramped_z();
                let rr = state.core.read_reg(r);
                state.write_u8_noneeprom(z as usize + q as usize, rr);
                if !state.info.xmega && !state.info.tiny {
                    state.core.cycles += 1;
                }
            },

            &Sts16 { r, k } => {
                let addr = state.core.ramped_addr(k);
                let rr = state.core.read_reg(r);
                state.write_u8_noneeprom(addr, rr);
                state.core.cycles += 1;
                state.core.pc += 1;
            }

            &Sts { r, k } => {
                let rr = state.core.read_reg(r);
                state.write_u8_noneeprom(k as usize, rr);
            }

            &Sub { d, r } => {
                let rd = state.core.read_reg(d);
                let rr = state.core.read_reg(r);
                let res = rd as i16 - rr as i16;
                state.core.carry = rr > rd;
                // TODO: HV
                set_zns(state, res as u8);
                state.core.write_reg(d, (res & 0xFF) as u8);
            },
            &Subi { d, k } => {
                let rd = state.core.read_reg(d);
                let res = rd as i16 - k as i16;
                state.core.write_reg(d, (res & 0xFF) as u8);

                // TODO: H flag
                state.core.carry = res < 0;
                state.core.v = calc_v(rd, k, (res & 0xFF) as u8);
                set_zns(state, (res & 0xFF) as u8);
            }

            &Invaild { opcode } => {
                state.core.cycles -= 1;
                return state.crash(CpuSignal::InvaildOpcode { opcode });
            }
        }

        Ok(())
    }

    /// size in words
    pub fn size(&self) -> usize {
        match self {
            &Jmp { .. } | &Call { .. } | &Lds16 { .. } | &Sts16 { .. } => 2,
            _ => 1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use models::xmega_au::XmegaA4U::ATxmega128A4U;
    use models::AvrModel;
    use std::mem;

    #[test]
    fn execute_call_xmega() {
        let mut vm = ATxmega128A4U.create_vm();
        let old_sp = vm.core.sp;
        vm.core.pc = 0xAABBCC;

        let cmd = Call { k: 0x1337 };
        cmd.execute(&mut vm).unwrap();

        assert_eq!(vm.core.sp, old_sp - 3);
        assert_eq!(vm.read_unchecked(old_sp - 0, true), 0xCE);
        assert_eq!(vm.read_unchecked(old_sp - 1, true), 0xBB);
        assert_eq!(vm.read_unchecked(old_sp - 2, true), 0xAA);
        assert_eq!(vm.core.pc, 0x1337);
        assert_eq!(vm.core.cycles, 4);
    }

    #[test]
    fn execute_jmp() {
        let mut vm = ATxmega128A4U.create_vm();

        let cmd = Jmp { k: 0x1337 };
        cmd.execute(&mut vm).unwrap();

        assert_eq!(vm.core.pc, 0x1337);
        assert_eq!(vm.core.cycles, 3);
    }

    #[test]
    fn execute_rjmp() {
        let mut vm = ATxmega128A4U.create_vm();
        vm.core.pc = 1000;

        let cmd = Rjmp { k: -5 };
        cmd.execute(&mut vm).unwrap();

        assert_eq!(vm.core.pc, 1000 - 5 + 1);
        assert_eq!(vm.core.cycles, 2);
    }

    #[test]
    fn execute_ret() {
        let mut vm = ATxmega128A4U.create_vm();

        vm.core.sp -= 3;
        let sp = vm.core.sp;
        vm.write_unchecked(sp + 1, 0xAAu8);
        vm.write_unchecked(sp + 2, 0xBBu8);
        vm.write_unchecked(sp + 3, 0xCCu8);

        let cmd = Ret;
        cmd.execute(&mut vm).unwrap();

        assert_eq!(vm.core.pc, 0xAABBCC);
        assert_eq!(vm.core.cycles, 5);
    }

    #[test]
    fn execute_in() {
        let mut vm = ATxmega128A4U.create_vm();

        vm.write_unchecked(33, 0x42u8);

        let cmd = In { d: 26, a: 33 };
        cmd.execute(&mut vm).unwrap();

        assert_eq!(vm.core.read_reg(26), 0x42u8);
        assert_eq!(vm.core.cycles, 1);
    }

    #[test]
    fn execute_out() {
        let mut vm = ATxmega128A4U.create_vm();

        vm.core.write_reg(30, 0x76u8);

        let cmd = Out { r: 30, a: 15 };
        cmd.execute(&mut vm).unwrap();

        assert_eq!(vm.read_unchecked(15, true), 0x76u8);
        assert_eq!(vm.core.cycles, 1);
    }

    #[test]
    fn execute_ldi() {
        let mut vm = ATxmega128A4U.create_vm();

        let cmd = Ldi { d: 17, k: 42 };
        cmd.execute(&mut vm).unwrap();

        assert_eq!(vm.core.read_reg(17), 42);
    }

    #[test]
    fn execute_add() {
        let mut vm = ATxmega128A4U.create_vm();
        vm.core.write_reg(17, 34);
        vm.core.write_reg(18, 12);

        let cmd = Add { d: 17, r: 18 };
        cmd.execute(&mut vm).unwrap();

        assert_eq!(vm.core.read_reg(17), 46);
        assert_eq!(vm.core.zero, false);
        assert_eq!(vm.core.carry, false);
    }

    #[test]
    fn execute_add_zero() {
        let mut vm = ATxmega128A4U.create_vm();
        vm.core.write_reg(17, 255);
        vm.core.write_reg(18, 1);

        let cmd = Add { d: 17, r: 18 };
        cmd.execute(&mut vm).unwrap();

        assert_eq!(vm.core.read_reg(17), 0);
        assert_eq!(vm.core.zero, true);
        assert_eq!(vm.core.carry, true);
    }

    #[test]
    fn execute_add_carry() {
        let mut vm = ATxmega128A4U.create_vm();
        vm.core.write_reg(17, 1);
        vm.core.write_reg(18, 1);
        vm.core.carry = true;

        let cmd = Adc { d: 17, r: 18 };
        cmd.execute(&mut vm).unwrap();

        assert_eq!(vm.core.read_reg(17), 3);
        assert_eq!(vm.core.zero, false);
        assert_eq!(vm.core.carry, false);
    }

    #[test]
    fn execute_sub() {
        let mut vm = ATxmega128A4U.create_vm();
        vm.core.write_reg(0, 34);
        vm.core.write_reg(1, 12);

        let cmd = Sub { d: 0, r: 1 };
        cmd.execute(&mut vm).unwrap();

        assert_eq!(vm.core.read_reg(0), 22);
        assert_eq!(vm.core.zero, false);
        assert_eq!(vm.core.carry, false);
    }

    #[test]
    fn execute_sub_zero() {
        let mut vm = ATxmega128A4U.create_vm();
        vm.core.write_reg(0, 55);
        vm.core.write_reg(1, 55);

        let cmd = Sub { d: 0, r: 1 };
        cmd.execute(&mut vm).unwrap();

        assert_eq!(vm.core.read_reg(0), 0);
        assert_eq!(vm.core.zero, true);
        assert_eq!(vm.core.carry, false);
    }

    #[test]
    fn execute_sub_neg1() {
        let mut vm = ATxmega128A4U.create_vm();
        vm.core.write_reg(0, 50);
        vm.core.write_reg(1, 100);

        let cmd = Sub { d: 0, r: 1 };
        cmd.execute(&mut vm).unwrap();

        assert_eq!(vm.core.read_reg(0), as_unsigned(-50));
        assert_eq!(vm.core.zero, false);
        assert_eq!(vm.core.carry, true);
    }

    #[test]
    fn execute_sub_neg2() {
        let mut vm = ATxmega128A4U.create_vm();
        vm.core.write_reg(0, as_unsigned(-100));
        vm.core.write_reg(1, as_unsigned(-50));

        let cmd = Sub { d: 0, r: 1 };
        cmd.execute(&mut vm).unwrap();

        assert_eq!(vm.core.read_reg(0), as_unsigned(-50));
        assert_eq!(vm.core.zero, false);
        assert_eq!(vm.core.carry, true);
    }

    #[test]
    fn execute_dec() {
        let mut vm = ATxmega128A4U.create_vm();
        vm.core.write_reg(12, 0);

        let cmd = Dec { d: 12 };
        cmd.execute(&mut vm).unwrap();

        assert_eq!(vm.core.read_reg(12), 0xFF);
    }

    #[test]
    fn execute_dec_0x80() {
        let mut vm = ATxmega128A4U.create_vm();
        vm.core.write_reg(12, 0x80);

        let cmd = Dec { d: 12 };
        cmd.execute(&mut vm).unwrap();

        assert_eq!(vm.core.read_reg(12), 0x7f);
        assert_eq!(vm.core.v, true);
    }

    #[test]
    fn execute_inc() {
        let mut vm = ATxmega128A4U.create_vm();
        vm.core.write_reg(12, 0xFF);

        let cmd = Inc { d: 12 };
        cmd.execute(&mut vm).unwrap();

        assert_eq!(vm.core.read_reg(12), 0);
        assert_eq!(vm.core.zero, true);
    }

    #[test]
    fn execute_inc_0x7f() {
        let mut vm = ATxmega128A4U.create_vm();
        vm.core.write_reg(12, 0x7f);

        let cmd = Inc { d: 12 };
        cmd.execute(&mut vm).unwrap();

        assert_eq!(vm.core.read_reg(12), 0x80);
        assert_eq!(vm.core.v, true);
    }

    #[test]
    fn instr_size() {
        assert_eq!(mem::size_of::<Instruction>(), 8);
    }
}