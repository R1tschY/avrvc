use decoder::AvrDecoder;
use debug::AvrDebugger;
use byte_convert::u32be;
use byte_convert::u16be;
use std::ops::Range;
use byte_convert::u16le;
use byte_convert::u8bits;
use byte_convert::read_u16le;
use std::collections::HashMap;
use byte_convert::write_u16le;
use std::sync::Arc;

/// Signals send by cpu
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CpuSignal {
    /// execution of invaild opcode triggered
    InvaildOpcode { opcode: u16 },

    /// tried execution of a PC out of bounds
    PcOutOfBounds { pc: i32 },

    /// execution of break instruction
    Break,
}

pub type IoReadFunc = Box<Fn(&AvrCoreState, usize) -> u8 + Send>;
pub type IoWriteFunc = Box<Fn(&mut AvrCoreState, usize, u8) + Send>;

pub struct AvrCoreState {
    /// cycle counter
    pub cycles: u64,

    /// program counter (PC) in words
    pub pc: usize,

    /// stack pointer (SP)
    pub sp: usize,

    /// hardware register
    pub regs: [u8; 32],

    /// interrupt flag (I)
    pub interrupt: bool,

    pub t: bool,

    pub h: bool,

    /// sign flag (S)
    pub sign: bool,

    pub v: bool,

    pub n: bool,

    /// zero flag (Z)
    pub zero: bool,

    /// carry flag (C)
    pub carry: bool,

    /// flash bytes
    pub flash: Vec<u8>,

    ram_offset: usize,
    ram: Vec<u8>,

    eeprom_offset: usize,
    eeprom: Vec<u8>,
}

impl AvrCoreState {
    pub fn read_reg(&self, addr: u8) -> u8 { self.regs[addr as usize] }
    pub fn write_reg(&mut self, addr: u8, data: u8) -> () { self.regs[addr as usize] = data; }

    pub fn read_reg16(&self, addr: u8) -> u16 {
        read_u16le(&self.regs[addr as usize..])
    }

    pub fn write_reg16(&mut self, addr: u8, data: u16) -> () {
        self.regs[addr as usize] = (data & 0xFF) as u8;
        self.regs[addr as usize + 1] = (data >> 8) as u8;
    }

    pub fn read_x(&self) -> u16 { u16le(self.read_reg(26), self.read_reg(27)) }
    pub fn read_y(&self) -> u16 { u16le(self.read_reg(28), self.read_reg(29)) }
    pub fn read_z(&self) -> u16 { u16le(self.read_reg(30), self.read_reg(31)) }

    pub fn write_x(&mut self, value: u16) {
        self.write_reg(26, value as u8);
        self.write_reg(27, (value >> 8) as u8);
    }

    pub fn write_y(&mut self, value: u16) {
        self.write_reg(28, value as u8);
        self.write_reg(29, (value >> 8) as u8);
    }

    pub fn write_z(&mut self, value: u16) {
        self.write_reg(30, value as u8);
        self.write_reg(31, (value >> 8) as u8);
    }

    pub fn read_sreg(&self) -> u8 {
        u8bits(
            self.interrupt, self.t, self.h, self.sign, self.v, self.n,
            self.zero, self.carry)
    }

    pub fn write_sreg(&mut self, value: u8) {
        self.carry = value & (1 << 0) != 0;
        self.zero = value & (1 << 1) != 0;
        self.n = value & (1 << 2) != 0;
        self.v = value & (1 << 3) != 0;
        self.sign = value & (1 << 4) != 0;
        self.h = value & (1 << 5) != 0;
        self.t = value & (1 << 6) != 0;
        self.interrupt = value & (1 << 7) != 0;
    }

}

/// avr cpu state, cpu information and external hooks
pub struct AvrVm {
    pub core: AvrCoreState,

    pub debugger: AvrDebugger,

    pub info: AvrVmInfo,

    io_regs_w: HashMap<usize, IoWriteFunc>,
    io_regs_r: HashMap<usize, IoReadFunc>,
    io_reg_state: Vec<u8>,

    pub decoder: AvrDecoder
}

impl AvrVm {

    pub fn new(info: &AvrVmInfo) -> AvrVm {
        let info = info.clone();
        let mut result = AvrVm {
            core: AvrCoreState {
                pc: 0,
                sp: 0,
                interrupt: false,
                t: false,
                h: false,
                sign: false,
                v: false,
                n: false,
                zero: false,
                carry: false,
                cycles: 0,
                regs: [0; 32],

                flash: Vec::new(),

                ram_offset: info.ram.start,
                ram: vec![0u8; info.ram.end - info.ram.start],

                eeprom_offset: info.eeprom.start,
                eeprom: vec![0u8; info.eeprom.end - info.eeprom.start],
            },
            info: info.clone(),

            io_regs_w: HashMap::new(),
            io_regs_r: HashMap::new(),
            io_reg_state: vec![0u8; info.ios],

            debugger: AvrDebugger::new(),
            decoder: AvrDecoder::new()
        };
        result.core.flash.resize(info.flash_bytes, 0);
        result
    }

    pub fn push3(&mut self, v: u32) {
        self.core.sp -= 3;

        let sp = self.core.sp;
        self.write_u8(sp + 1, (v >> 16) as u8);
        self.write_u8(sp + 2, (v >> 8) as u8);
        self.write_u8(sp + 3, v as u8);
    }

    pub fn push2(&mut self, v: u16) {
        self.core.sp -= 2;

        let sp = self.core.sp;
        self.write_u8(sp + 1, (v >> 8) as u8);
        self.write_u8(sp + 2, v as u8);
    }

    pub fn push(&mut self, v: u8) {
        self.core.sp -= 1;
        let sp = self.core.sp;
        self.write_u8(sp + 1, v);
    }

    pub fn pop3(&mut self) -> u32 {
        self.core.sp += 3;

        let sp = self.core.sp;
        u32be(
            0,
            self.read_u8(sp - 2),
            self.read_u8(sp - 1),
            self.read_u8(sp + 0),
        )
    }

    pub fn pop2(&mut self) -> u16 {
        self.core.sp += 2;

        let sp = self.core.sp;
        u16be(
            self.read_u8(sp - 1),
            self.read_u8(sp + 0),
        )
    }

    pub fn pop(&mut self) -> u8 {
        self.core.sp += 1;

        self.read_u8(self.core.sp)
    }

    pub fn write_flash(&mut self, addr: usize, data: &[u8]) {
        self.core.flash[addr..addr+data.len()].copy_from_slice(&data);
    }

    pub fn register_io(&mut self, addr: usize, read_func: IoReadFunc, write_func: IoWriteFunc) {
        self.register_io_read(addr, read_func);
        self.register_io_write(addr, write_func);
    }

    pub fn register_io_read(&mut self, addr: usize, func: IoReadFunc) {
        self.io_regs_r.insert(addr, func);
    }

    pub fn register_io_write(&mut self, addr: usize, func: IoWriteFunc) {
        self.io_regs_w.insert(addr, func);
    }

    pub fn read_io(&self, addr: usize) -> u8 {
        if addr < self.io_reg_state.len() {
            match self.io_regs_r.get(&addr) {
                Some(func) => func(&self.core, addr),
                None => self.io_reg_state[addr]
            }
        } else {
            debug_assert!(false, "read from reserved memory: 0x{:08x}", addr);
            0
        }
    }

    pub fn write_io(&mut self, addr: usize, value: u8) {
        if addr < self.io_reg_state.len() {
            let i = self.io_regs_w.get(&addr).clone();
            if let Some(func) = i {
                func(&mut self.core, addr, value);
            }
            self.io_reg_state[addr] = value;
        } else {
            debug_assert!(false, "write to reserved memory: 0x{:08x}", addr);
        }
    }

    pub fn read_u8(&self, addr: usize) -> u8 {
        if addr > self.core.ram_offset && addr < self.core.ram_offset + self.core.ram.len() {
            self.core.ram[addr - self.core.ram_offset]
        } else if addr < self.io_reg_state.len() {
            self.io_reg_state[addr]
        } else if addr > self.core.eeprom_offset
                && addr < self.core.eeprom_offset + self.core.eeprom.len() {
            self.core.eeprom[addr - self.core.eeprom_offset]
        } else {
            debug_assert!(false, "read from reserved memory: 0x{:08x}", addr);
            0
        }
    }

    pub fn write_u8(&mut self, addr: usize, value: u8) {
        if addr > self.core.ram_offset && addr < self.core.ram_offset + self.core.ram.len() {
            let offset = self.core.ram_offset;
            self.core.ram[addr - offset] = value;
        } else if addr < self.io_reg_state.len() {
            self.io_reg_state[addr] = value;
        } else if addr > self.core.eeprom_offset && addr < self.core.eeprom_offset + self.core.eeprom.len() {
            let offset = self.core.eeprom_offset;
            self.core.eeprom[addr - offset] = value;
        } else {
            debug_assert!(false, "write to reserved memory: 0x{:08x}", addr);
        }
    }

    pub fn read_u8_noneeprom(&self, addr: usize) -> u8 {
        if addr > self.core.ram_offset && addr < self.core.ram_offset + self.core.ram.len() {
            self.core.ram[addr - self.core.ram_offset]
        } else if addr < self.io_reg_state.len() {
            self.io_reg_state[addr]
        } else {
            debug_assert!(false, "read from reserved memory: 0x{:08x}", addr);
            0
        }
    }

    pub fn write_u8_noneeprom(&mut self, addr: usize, value: u8) {
        if addr > self.core.ram_offset && addr < self.core.ram_offset + self.core.ram.len() {
            let offset = self.core.ram_offset;
            self.core.ram[addr - offset] = value;
        } else if addr < self.io_reg_state.len() {
            self.io_reg_state[addr] = value;
        } else {
            debug_assert!(false, "write to reserved memory: 0x{:08x}", addr);
        }
    }

    pub fn read_u16(&self, addr: usize) -> u16 {
        if addr > self.core.ram_offset && addr + 1 < self.core.ram_offset + self.core.ram.len() {
            let i = addr - self.core.ram_offset;
            read_u16le(&self.core.ram[i..])
        } else if addr + 1 < self.io_reg_state.len() {
            read_u16le(&self.io_reg_state[addr..])
        } else if addr > self.core.eeprom_offset && addr + 1 < self.core.eeprom_offset + self.core.eeprom.len() {
            read_u16le(&self.core.eeprom[addr - self.core.eeprom_offset..])
        } else {
            debug_assert!(false, "read from reserved memory: 0x{:08x}", addr);
            0
        }
    }

    pub fn write_u16(&mut self, addr: usize, value: u16) {
        if addr > self.core.ram_offset && addr + 1 < self.core.ram_offset + self.core.ram.len() {
            let i = addr - self.core.ram_offset;
            write_u16le(&mut self.core.ram[i..], value);
        } else if addr + 1 < self.io_reg_state.len() {
            write_u16le(&mut self.io_reg_state[addr..], value);
        } else if addr > self.core.eeprom_offset && addr + 1 < self.core.eeprom_offset + self.core.eeprom.len() {
            let offset = self.core.eeprom_offset;
            write_u16le(&mut self.core.eeprom[addr - offset..], value);
        }  else {
            debug_assert!(false, "write to reserved memory: 0x{:08x}", addr);
        }
    }


    pub fn crash(&mut self, crash_info: CpuSignal) -> Result<(), CpuSignal> {
        self.core.pc = 0; // reset

        Err(crash_info)
    }

    pub fn step(&mut self) -> Result<(), CpuSignal> {
        let instr = self.decoder.decode(&self.core.flash, self.core.pc * 2);
        self.debugger.pre_instr_hook(self, &instr)?;
        instr.execute(self)
    }

}

/// core informations needed for instruction execution
#[derive(Clone)]
pub struct AvrVmInfo {
    /// bytes needed for PC
    pub pc_bytes: i32, // TODO: make read-only

    /// is a XMEGA device
    pub xmega: bool, // TODO: make read-only

    /// is a Reduced Core tinyAVR device
    pub reduced_core_tiny: bool, // TODO: make read-only

    pub flash_bytes: usize, // TODO: make read-only

    pub ios: usize,

    pub ram: Range<usize>,

    pub eeprom: Range<usize>
}
