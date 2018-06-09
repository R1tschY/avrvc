
pub fn i16le(a: u8, b: u8) -> i16 { (b as i16) << 8 | a as i16 }
pub fn u16le(a: u8, b: u8) -> u16 { (b as u16) << 8 | a as u16 }
pub fn u16be(a: u8, b: u8) -> u16 { (a as u16) << 8 | b as u16 }

pub fn read_u16le(bytes: &[u8]) -> u16 { u16le(bytes[0], bytes[1]) }
pub fn write_u16le(bytes: &mut [u8], value: u16) {
    bytes[0] = (value & 0xFF) as u8;
    bytes[1] = (value >> 8) as u8;
}


pub fn i32le(a: u8, b: u8, c: u8, d: u8) -> i32 {
    (d as i32) << 24 | (c as i32) << 16 | (b as i32) << 8 | (a as i32)
}

pub fn u32le(a: u8, b: u8, c: u8, d: u8) -> u32 {
    (d as u32) << 24 | (c as u32) << 16 | (b as u32) << 8 | (a as u32)
}

pub fn u32be(a: u8, b: u8, c: u8, d: u8) -> u32 {
    (a as u32) << 24 | (b as u32) << 16 | (c as u32) << 8 | (d as u32)
}

pub fn u8bits(a: bool, b: bool, c: bool, d: bool, e: bool, f: bool, g: bool, h: bool) -> u8 {
    ((a as u8) << 7) | ((b as u8) << 6) | ((c as u8) << 5) | ((d as u8) << 4) | ((e as u8) << 3) |
        ((f as u8) << 2) | ((g as u8) << 1) | ((h as u8) << 0)
}

pub fn bit_at(value: u8, addr: u8) -> bool {
    value & (1 << addr) != 0
}