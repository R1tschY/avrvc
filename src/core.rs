use decoder::AvrDecoder;
use debug::AvrDebugger;
use memory::MemoryController;
use byte_convert::u32be;
use byte_convert::u16be;
use std::rc::Rc;
use std::ops::Range;
use byte_convert::u16le;
use byte_convert::u8bits;

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

pub trait AvrIoReg {
    fn write(&mut self, vm: &mut AvrVm, addr: u8, data: u8);
    fn read(&mut self, vm: &mut AvrVm, addr: u8);
}

/// avr cpu state, cpu information and external hooks
pub struct AvrVm {
    /// cycle counter
    pub cycles: u64,

    /// program counter (PC) in bytes
    pub pc: usize,

    /// stack pointer (SP)
    pub sp: usize,

    /// hardware register
    pub register: [u8; 32],

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

    pub debugger: AvrDebugger,

    pub info: AvrVmInfo,

    pub mem: MemoryController,

    pub decoder: AvrDecoder
}

impl AvrVm {

    pub fn new(info: &AvrVmInfo) -> AvrVm {
        let info = info.clone();
        let mut result = AvrVm {
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
            register: [0; 32],
            flash: Vec::new(),
            info: info.clone(),
            mem: MemoryController::new(info.ios, info.ram, info.eeprom),
            debugger: AvrDebugger::new(),
            decoder: AvrDecoder::new()
        };
        result.flash.resize(info.flash_bytes, 0);
        result
    }

    pub fn push3(&mut self, v: u32) {
        self.sp -= 3;

        let sp = self.sp;
        self.write_mem(sp + 1, (v >> 16) as u8);
        self.write_mem(sp + 2, (v >> 8) as u8);
        self.write_mem(sp + 3, v as u8);
    }

    pub fn push2(&mut self, v: u16) {
        self.sp -= 2;

        let sp = self.sp;
        self.write_mem(sp + 1, (v >> 8) as u8);
        self.write_mem(sp + 2, v as u8);
    }

    pub fn push(&mut self, v: u8) {
        self.sp += 1;
        let sp = self.sp;
        self.write_mem(sp + 1, v);
    }

    pub fn pop3(&mut self) -> u32 {
        self.sp += 3;

        let sp = self.sp;
        u32be(
            0,
            self.read_mem(sp - 2),
            self.read_mem(sp - 1),
            self.read_mem(sp + 0),
        )
    }

    pub fn pop2(&mut self) -> u16 {
        self.sp += 2;

        let sp = self.sp;
        u16be(
            self.read_mem(sp - 1),
            self.read_mem(sp + 0),
        )
    }

    pub fn pop(&mut self) -> u8 {
        self.sp += 1;

        self.read_mem(self.sp)
    }

    pub fn read_reg(&self, addr: u8) -> u8 {
        self.register[addr as usize]
    }

    pub fn write_reg(&mut self, addr: u8, data: u8) -> () {
        self.register[addr as usize] = data;
    }

    pub fn read_mem(&self, addr: usize) -> u8 {
        self.mem.read_u8(addr)
    }

    pub fn write_mem(&mut self, addr: usize, data: u8) {
        self.mem.write_u8(addr, data)
    }

    pub fn read_x(&self) -> u16 {
        u16le(self.read_reg(26), self.read_reg(27))
    }

    pub fn read_y(&self) -> u16 {
        u16le(self.read_reg(28), self.read_reg(29))
    }

    pub fn read_z(&self) -> u16 {
        u16le(self.read_reg(30), self.read_reg(31))
    }

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
        u8bits(self.interrupt, self.t, self.h, self.sign, self.v, self.n, self.zero, self.carry)
    }

    pub fn write_sreg(&mut self, value: u8) {
        self.carry = (value & (1 << 0) != 0);
        self.zero = (value & (1 << 1) != 0);
        self.n = (value & (1 << 2) != 0);
        self.v = (value & (1 << 3) != 0);
        self.sign = (value & (1 << 4) != 0);
        self.h = (value & (1 << 5) != 0);
        self.t = (value & (1 << 6) != 0);
        self.interrupt = (value & (1 << 7) != 0);
    }

    pub fn crash(&mut self, crash_info: CpuSignal) -> Result<(), CpuSignal> {
        self.pc = 0; // reset

        Err(crash_info)
    }

    pub fn step(&mut self) -> Result<(), CpuSignal> {
        self.debugger.pre_instr_hook(self)?;

        let instr = self.decoder.decode(&self.flash, self.pc);
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

    pub flash_bytes: usize, // TODO: make read-only

    pub ios: usize,

    pub ram: Range<usize>,

    pub eeprom: Range<usize>
}
