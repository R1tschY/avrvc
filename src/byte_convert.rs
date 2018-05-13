

pub fn i16le(a: u8, b: u8) -> i16 { (b as i16) << 8 | a as i16 }
pub fn u16le(a: u8, b: u8) -> u16 { (b as u16) << 8 | a as u16 }
pub fn u16be(a: u8, b: u8) -> u16 { (a as u16) << 8 | b as u16 }


pub fn i32le(a: u8, b: u8, c: u8, d: u8) -> i32 {
    (d as i32) << 24 | (c as i32) << 16 | (b as i32) << 8 | (a as i32)
}

pub fn u32le(a: u8, b: u8, c: u8, d: u8) -> u32 {
    (d as u32) << 24 | (c as u32) << 16 | (b as u32) << 8 | (a as u32)
}

pub fn u32be(a: u8, b: u8, c: u8, d: u8) -> u32 {
    (a as u32) << 24 | (b as u32) << 16 | (c as u32) << 8 | (d as u32)
}