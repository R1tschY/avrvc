use std::mem;
use bytes::ByteOrder;

pub trait IntToBytes {
    fn as_bytes<T: ByteOrder>(&self) -> Vec<u8>;
}

impl IntToBytes for usize {
    fn as_bytes<T: ByteOrder>(&self) -> Vec<u8> {
        let mut buffer: Vec<u8> = vec![0; mem::size_of::<usize>()];
        T::write_u64(&mut buffer, *self as u64);
        buffer
    }
}

impl IntToBytes for isize {
    fn as_bytes<T: ByteOrder>(&self) -> Vec<u8> {
        let mut buffer: Vec<u8> = vec![0; mem::size_of::<isize>()];
        T::write_i64(&mut buffer, *self as i64);
        buffer
    }
}

impl IntToBytes for u32 {
    fn as_bytes<T: ByteOrder>(&self) -> Vec<u8> {
        let mut buffer: Vec<u8> = vec![0; mem::size_of::<u32>()];
        T::write_u32(&mut buffer, *self as u32);
        buffer
    }
}

impl IntToBytes for i32 {
    fn as_bytes<T: ByteOrder>(&self) -> Vec<u8> {
        let mut buffer: Vec<u8> = vec![0; mem::size_of::<i32>()];
        T::write_i32(&mut buffer, *self as i32);
        buffer
    }
}

impl IntToBytes for u16 {
    fn as_bytes<T: ByteOrder>(&self) -> Vec<u8> {
        let mut buffer: Vec<u8> = vec![0; mem::size_of::<u16>()];
        T::write_u16(&mut buffer, *self as u16);
        buffer
    }
}

impl IntToBytes for i16 {
    fn as_bytes<T: ByteOrder>(&self) -> Vec<u8> {
        let mut buffer: Vec<u8> = vec![0; mem::size_of::<i16>()];
        T::write_i16(&mut buffer, *self as i16);
        buffer
    }
}

impl IntToBytes for u8 {
    fn as_bytes<T: ByteOrder>(&self) -> Vec<u8> {
        vec![*self]
    }
}

impl IntToBytes for i8 {
    fn as_bytes<T: ByteOrder>(&self) -> Vec<u8> {
        vec![*self as u8]
    }
}