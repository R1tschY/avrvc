use decoder::AvrDecoder;
use debug::AvrDebugger;
use byte_convert::u32be;
use byte_convert::u16be;

pub enum Crash {
    InvaildOpcode { opcode: u16 },
    PcOutOfBounds { pc: i32 },
}

pub trait AvrIoReg {
    fn write(&mut self, vm: &mut AvrVm, addr: u8, data: u8);
    fn read(&mut self, vm: &mut AvrVm, addr: u8);
}

pub struct AvrVm {
    /// program counter (PC) in bytes
    pub pc: usize,

    /// stack pointer (SP)
    pub sp: usize,

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

    /// cycle counter
    pub cycles: u64,

    /// register, I/O, sram
    pub memory: Vec<u8>,

    /// flash bytes
    pub flash: Vec<u8>,

    pub debugger: AvrDebugger,

    pub info: AvrVmInfo,

    pub io: Vec<Box<AvrIoReg + Send>>
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
            memory: Vec::new(),
            flash: Vec::new(),
            info: *info,
            io: Vec::new(),
            debugger: AvrDebugger::new()
        };
        result.memory.resize(info.memory_bytes, 0);
        result.flash.resize(info.flash_bytes, 0);
        result
    }

    pub fn push3(&mut self, v: u32) {
        self.sp -= 3;
        self.memory[self.sp + 1] = (v >> 16) as u8;
        self.memory[self.sp + 2] = (v >> 8) as u8;
        self.memory[self.sp + 3] = v as u8;
    }

    pub fn push2(&mut self, v: u16) {
        self.sp -= 2;
        self.memory[self.sp + 1] = (v >> 8) as u8;
        self.memory[self.sp + 2] = v as u8;
    }

    pub fn push(&mut self, v: u8) {
        self.sp += 1;
        self.memory[self.sp + 1] = v;
    }

    pub fn pop3(&mut self) -> u32 {
        self.sp += 3;
        u32be(
            0,
            self.memory[self.sp - 2],
            self.memory[self.sp - 1],
            self.memory[self.sp + 0],
        )
    }

    pub fn pop2(&mut self) -> u16 {
        self.sp += 2;

        u16be(
            self.memory[self.sp - 1],
            self.memory[self.sp + 0],
        )
    }

    pub fn pop(&mut self) -> u8 {
        self.sp += 1;
        self.memory[self.sp]
    }

    pub fn read_reg(&self, addr: u8) -> u8 {
        self.memory[addr as usize]
    }

    pub fn write_reg(&mut self, addr: u8, data: u8) -> () {
        self.memory[addr as usize] = data;
    }

    pub fn write_io(&mut self, addr: u8, data: u8) {
        if addr > 31 {
            let io_addr = (addr - 31) as usize;
            if io_addr < self.io.len() {
                let _io: &mut Box<AvrIoReg + Send> = &mut (self.io[io_addr]);
                // TODO: io.write(self, addr, data);
            } else {
                self.memory[addr as usize] = data;
            }
        } else {
            self.memory[addr as usize] = data;
        }
    }

    pub fn read_io(&mut self, addr: u8) -> u8 {
        if addr > 31 {
            let io_addr = (addr - 31) as usize;
            if io_addr < self.io.len() {
                let _io: &mut Box<AvrIoReg + Send> = &mut (self.io[io_addr]);
                // TODO: return io.read(self, addr);
                0u8
            } else {
                self.memory[addr as usize]
            }
        } else {
            self.memory[addr as usize]
        }
    }

    pub fn crash(&mut self, crash_info: Crash) -> Result<(), Crash> {
        self.pc = 0; // reset

        Err(crash_info)
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

    pub memory_bytes: usize, // TODO: make read-only
}

pub fn execute_one(decoder: &AvrDecoder, vm: &mut AvrVm) -> () {
    let instr = decoder.decode(&vm.flash, vm.pc);
    instr.execute(vm);
}

pub fn default_crash_handler(crash_info: Crash) -> () {
    let message = match crash_info {
       Crash::InvaildOpcode { opcode } => format!("invalid opcode: {:x}", opcode),
       Crash::PcOutOfBounds { pc } => format!("pc out of bounds: {:x}", pc)
    };
    panic!("Processor crash: {}", message)
}
