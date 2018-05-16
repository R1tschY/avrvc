use decoder::AvrDecoder;
use debug::AvrDebugger;
use byte_convert::u32be;
use byte_convert::u16be;
use std::rc::Rc;

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

    /// sram
    pub ram: Vec<u8>,

    /// flash bytes
    pub flash: Vec<u8>,

    pub debugger: AvrDebugger,

    pub info: AvrVmInfo,

    pub io: Vec<Box<AvrIoReg + Send>>,

    pub decoder: AvrDecoder
}

impl AvrVm {

    pub fn new(info: &AvrVmInfo) -> AvrVm {
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
            ram: Vec::new(),
            register: [0; 32],
            flash: Vec::new(),
            info: *info,
            io: Vec::new(),
            debugger: AvrDebugger::new(),
            decoder: AvrDecoder::new()
        };
        result.ram.resize(info.ram_bytes, 0);
        result.flash.resize(info.flash_bytes, 0);
        result
    }

    pub fn push3(&mut self, v: u32) {
        self.sp -= 3;

        let sp = self.sp - self.info.ram_offset;
        self.ram[sp + 1] = (v >> 16) as u8;
        self.ram[sp + 2] = (v >> 8) as u8;
        self.ram[sp + 3] = v as u8;
    }

    pub fn push2(&mut self, v: u16) {
        self.sp -= 2;

        let sp = self.sp - self.info.ram_offset;
        self.ram[sp + 1] = (v >> 8) as u8;
        self.ram[sp + 2] = v as u8;
    }

    pub fn push(&mut self, v: u8) {
        self.sp += 1;
        self.ram[self.sp - self.info.ram_offset + 1] = v;
    }

    pub fn pop3(&mut self) -> u32 {
        self.sp += 3;

        let sp = self.sp - self.info.ram_offset;
        u32be(
            0,
            self.ram[sp - 2],
            self.ram[sp - 1],
            self.ram[sp + 0],
        )
    }

    pub fn pop2(&mut self) -> u16 {
        self.sp += 2;

        let sp = self.sp - self.info.ram_offset;
        u16be(
            self.ram[sp - 1],
            self.ram[sp + 0],
        )
    }

    pub fn pop(&mut self) -> u8 {
        self.sp += 1;

        self.ram[self.sp - self.info.ram_offset]
    }

    pub fn read_reg(&self, addr: u8) -> u8 {
        self.register[addr as usize]
    }

    pub fn write_reg(&mut self, addr: u8, data: u8) -> () {
        self.register[addr as usize] = data;
    }

    pub fn write_io(&mut self, addr: u8, data: u8) {
//        if addr > 31 {
//            let io_addr = (addr - 31) as usize;
//            if io_addr < self.io.len() {
//                let _io: &mut Box<AvrIoReg + Send> = &mut (self.io[io_addr]);
//                // TODO: io.write(self, addr, data);
//            } else {
//                self.memory[addr as usize] = data;
//            }
//        } else {
//            self.memory[addr as usize] = data;
//        }
    }

    pub fn read_io(&mut self, addr: u8) -> u8 {
//        if addr > 31 {
//            let io_addr = (addr - 31) as usize;
//            if io_addr < self.io.len() {
//                let _io: &mut Box<AvrIoReg + Send> = &mut (self.io[io_addr]);
//                // TODO: return io.read(self, addr);
//                0u8
//            } else {
//                self.memory[addr as usize]
//            }
//        } else {
//            self.memory[addr as usize]
//        }
        0u8
    }

    pub fn read_mem(&mut self, addr: usize) -> u8 {
        0u8
    }

    pub fn write_mem(&mut self, addr: usize, data: u8) {

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
#[derive(Clone, Copy)]
pub struct AvrVmInfo {
    /// bytes needed for PC
    pub pc_bytes: i32, // TODO: make read-only

    /// is a XMEGA device
    pub xmega: bool, // TODO: make read-only

    pub flash_bytes: usize, // TODO: make read-only

    pub ram_bytes: usize, // TODO: make read-only

    pub ram_offset: usize,
}
