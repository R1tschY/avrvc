
use instruction_set::Instruction;
use std::collections::HashMap;
use byte_convert::u16le;

fn encode_55(d: u8, r: u8) -> u16 {
    ((r as u16 & 0b10000) << 5) | (r as u16 & 0b1111) | ((d as u16) << 4)
}

fn encode_56(r: u8, a: u8) -> u16 {
    ((a as u16 & 0b110000) << 5) | (a as u16 & 0b1111) | ((r as u16) << 4)
}

pub struct AvrDecoder {
    instr16: HashMap<u16, Instruction>
}

impl AvrDecoder {

    pub fn new() -> AvrDecoder {
        let mut instr16 = HashMap::new();

        instr16.insert(0b_1001_0100_1111_1000_u16, Instruction::Cli);
        instr16.insert(0b_1001_0101_0000_1000_u16, Instruction::Ret);

        // LDI
        for d in 16..32u8 {
            for k in 0..256 {
                instr16.insert(
                    0b_1110_0000_0000_0000_u16
                        | (k as u16 & 0xF0) << 4
                        | ((d - 16) as u16) << 4
                        | (k as u16 & 0x0F) << 0,
                    Instruction::Ldi { d, k: k as u8 });
            }
        }

        // EOR
        for d in 0..32u8 {
            for r in 0..32u8 {
                instr16.insert(
                    0b_0010_0100_0000_0000_u16 | encode_55(d, r),
                    Instruction::Eor { d, r });
            }
        }

        // OUT
        for r in 0..32u8 {
            for a in 0..64u8 {
                instr16.insert(
                    0b_1011_1000_0000_0000_u16 | encode_56(r, a),
                    Instruction::Out { r, a });
            }
        }

        // RJMP
        for k in -2048i16..2048i16 {
            instr16.insert(
                0b_1100_0000_0000_0000_u16 | (k & 0x0FFF) as u16,
                Instruction::Rjmp { k: k * 2 });
        }

        // IN
        for d in 0..32u8 {
            for a in 0..64u8 {
                instr16.insert(
                    0b_1011_0000_0000_0000_u16 | encode_56(d, a),
                    Instruction::In { d, a });
            }
        }

        // POP
        for r in 0..32u8 {
            instr16.insert(
                0b_1001_0000_0000_1111_u16 | ((r as u16) << 4),
                Instruction::Pop { r });
        }

        // PUSH
        for r in 0..32u8 {
            instr16.insert(
                0b_1001_0010_0000_1111_u16 | ((r as u16) << 4),
                Instruction::Push { r });
        }

        AvrDecoder { instr16 }
    }

    /// decode opcode at position `pos`
    ///
    /// no bounds checking performed!
    pub fn decode(&self, bytes: &Vec<u8>, pos: usize) -> Instruction {
        // 1001 010x JMP
        // 1001 010x CALL
        // 1001 000x LDS
        let b0: u8 = bytes[pos];
        let b1: u8 = bytes[pos + 1];
        let w0 = u16le(b0, b1);

        match b1 {
            0b10010100 | 0b10010101 => {
                match b0 & 0b1110 {
                    0b1100 => return Instruction::Jmp { k: self.decode_jump_call(bytes, pos, w0) },
                    0b1110 => return Instruction::Call { k: self.decode_jump_call(bytes, pos, w0) },
                    _ => { }
                }
            },
            _ => { }
        }

        match self.instr16.get(&w0) {
            Some(instr) => *instr,
            None => Instruction::Invaild { opcode: w0 }
        }
    }

    fn decode_jump_call(&self, flash: &Vec<u8>, pos: usize, w0: u16) -> usize {
        if pos + 4 >= flash.len() {
            panic!("decode: index out of bounds: {}", pos)
        }

        let w1 = u16le(flash[pos + 2], flash[pos + 3]);

        (
            ((w0 as usize & 0b111110000) << 13)
                | ((w0 as usize & 0x1) << 16)
                | w1 as usize
        ) << 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ldi1() {
        let decoder = AvrDecoder::new();
        let bytes = vec![0x8au8, 0xe2u8];
        let instr = decoder.decode(&bytes, 0);
        assert_eq!(instr, Instruction::Ldi { d: 24, k: 0x2A });
    }

    #[test]
    fn test_ldi2() {
        let decoder = AvrDecoder::new();
        let bytes = vec![0x90u8, 0xe0u8];
        let instr = decoder.decode(&bytes, 0);
        assert_eq!(instr, Instruction::Ldi { d: 25, k: 0x00 });
    }

    #[test]
    fn test_ret() {
        let decoder = AvrDecoder::new();
        let bytes = vec![0x08u8, 0x95u8];
        let instr = decoder.decode(&bytes, 0);
        assert_eq!(instr, Instruction::Ret);
    }
}