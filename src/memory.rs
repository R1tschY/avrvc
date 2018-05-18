use std::ops::Range;
use core::AvrIoReg;
use std::rc::Rc;
use std::sync::Arc;

pub struct MemoryController {
    io_regs: Vec<Option<Box<AvrIoReg + Send>>>,
    io_reg_state: Vec<u8>,

    ram_offset: usize,
    ram: Vec<u8>,

    eeprom_offset: usize,
    eeprom: Vec<u8>,
}

impl MemoryController {

    pub fn new(ios: usize, ram: Range<usize>, eeprom: Range<usize>) -> MemoryController {
        MemoryController {
            io_regs: Vec::new(),//vec![None; ios],
            io_reg_state: vec![0u8; ios],

            ram_offset: ram.start,
            ram: vec![0u8; ram.end - ram.start],

            eeprom_offset: eeprom.start,
            eeprom: vec![0u8; eeprom.end - eeprom.start],
        }
    }

    pub fn read_u8(&self, addr: usize) -> u8 {
        if addr > self.ram_offset && addr < self.ram_offset + self.ram.len() {
            self.ram[addr - self.ram_offset]
        } else if addr < self.io_reg_state.len() {
            self.io_reg_state[addr]
        } else if addr > self.eeprom_offset && addr < self.eeprom_offset + self.eeprom.len() {
            self.eeprom[addr - self.eeprom_offset]
        } else {
            0
        }
    }

    pub fn write_u8(&mut self, addr: usize, value: u8) {
        if addr > self.ram_offset && addr < self.ram_offset + self.ram.len() {
            let offset = self.ram_offset;
            self.ram[addr - offset] = value;
        } else if addr < self.io_reg_state.len() {
            self.io_reg_state[addr] = value;
        } else if addr > self.eeprom_offset && addr < self.eeprom_offset + self.eeprom.len() {
            let offset = self.eeprom_offset;
            self.eeprom[addr - offset] = value;
        }
    }

}