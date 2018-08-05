use instruction_set::Instruction;
use std::collections::HashMap;
use byte_convert::u16le;
use byte_convert::u8bits;
use byte_convert::bit_at;

pub trait Decoder {
    fn decode(&self, bytes: &Vec<u8>, pos: usize) -> Instruction;
}


fn encode_56(r: u8, a: u8) -> u16 {
    ((a as u16 & 0b110000) << 5) | (a as u16 & 0b1111) | ((r as u16) << 4)
}

fn add_instr26(
    instr16: &mut HashMap<u16, Instruction>,
    base: u16,
    factory: fn(u8, u8) -> Instruction
) {
    for &d in [24u8, 26u8, 28u8, 30u8].iter() {
        for k in 0..64u8 {
            instr16.insert(
                base | (k as u16 & 0x0F) | ((k as u16 & 0x30) << 2) | (((d as u16 - 24) / 2) << 4),
                factory(d, k));
        }
    }
}


fn add_instr55(
    instr16: &mut HashMap<u16, Instruction>,
    base: u16,
    factory: fn(u8, u8) -> Instruction
) {
    for d in 0..32u8 {
        for r in 0..32u8 {
            instr16.insert(
                base | ((r as u16 & 0b10000) << 5) | (r as u16 & 0b1111) | ((d as u16) << 4),
                factory(d, r));
        }
    }
}

fn add_instr44(
    instr16: &mut HashMap<u16, Instruction>,
    base: u16,
    factory: fn(u8, u8) -> Instruction
) {
    for d in 16..32u8 {
        for r in 16..32u8 {
            instr16.insert(
                base | ((r - 16) as u16) | (((d - 16) as u16) << 4),
                factory(d, r));
        }
    }
}

fn add_instr33(
    instr16: &mut HashMap<u16, Instruction>,
    base: u16,
    factory: fn(u8, u8) -> Instruction
) {
    for d in 16..24u8 {
        for r in 16..24u8 {
            instr16.insert(
                base | ((r - 16) as u16) | (((d - 16) as u16) << 4),
                factory(d, r));
        }
    }
}

fn add_stdldd(
    instr16: &mut HashMap<u16, Instruction>,
    base: u16,
    factory: fn(u8, u8) -> Instruction
) {
    for r in 0..32u8 {
        for q in 1..64u8 { // skip q == 0
            instr16.insert(
                base | ((q as u16 & 0b100000) << 8) | ((q as u16 & 0b11000) << 7)
                    | (q as u16 & 0b111) | ((r as u16) << 4),
                factory(r, q));
        }
    }
}

fn add_instr_sbix(
    instr16: &mut HashMap<u16, Instruction>,
    base: u16,
    factory: fn(u8, u8) -> Instruction
) {
    for b in 0..8u8 {
        for r in 0..32u8 {
            instr16.insert(
                base | ((r as u16) << 3) | (b as u16),
                factory(r, b));
        }
    }
}

fn add_instr_sbrx(
    instr16: &mut HashMap<u16, Instruction>,
    base: u16,
    factory: fn(u8, u8) -> Instruction
) {
    for b in 0..8u8 {
        for r in 0..32u8 {
            instr16.insert(
                base | ((r as u16) << 4) | (b as u16),
                factory(r, b));
        }
    }
}

fn add_instr48(
    instr16: &mut HashMap<u16, Instruction>,
    base: u16,
    factory: fn(u8, u8) -> Instruction
) {
    for d in 16..32u8 {
        for k in 0..256 {
            instr16.insert(
                base
                    | (k as u16 & 0xF0) << 4
                    | ((d - 16) as u16) << 4
                    | (k as u16 & 0x0F) << 0,
                factory(d, k as u8));
        }
    }
}

fn add_ldssts(
    instr16: &mut HashMap<u16, Instruction>,
    base: u16,
    factory: fn(u8, u8) -> Instruction
) {
    for d in 16..32u8 {
        for k in 0..128u8 {
            let addr = u8bits(
                bit_at(k, 0),
                bit_at(k, 1),
                bit_at(k, 2),
                bit_at(k, 3),
                bit_at(k, 5),
                bit_at(k, 6),
                bit_at(k, 4),
                !bit_at(k, 4)
            );
            instr16.insert(
                base
                    | (k as u16 & 0xF0) << 4
                    | ((d - 16) as u16) << 4
                    | (k as u16 & 0x0F) << 0,
                factory(addr, k));
        }
    }
}

fn add_movw(
    instr16: &mut HashMap<u16, Instruction>,
    base: u16,
    factory: fn(u8, u8) -> Instruction
) {
    for d in 0..16u8 {
        for r in 0..16u8 {
            instr16.insert(
                base | (r as u16) | ((d as u16) << 4),
                factory(d * 2, r * 2));
        }
    }
}

fn add_instr5(
    instr16: &mut HashMap<u16, Instruction>,
    base: u16,
    factory: fn(u8) -> Instruction
) {
    for d in 0..32u8 {
        instr16.insert(base | ((d as u16) << 4), factory(d));
    }
}

fn add_instr7(
    instr16: &mut HashMap<u16, Instruction>,
    base: u16,
    factory: fn(i8) -> Instruction
) {
    for k in -64i8..64i8 {
        instr16.insert(base | (((k & 0x7F) as u16) << 3), factory(k));
    }
}

fn add_instr12(
    instr16: &mut HashMap<u16, Instruction>,
    base: u16,
    factory: fn(i16) -> Instruction
) {
    for k in -2048i16..2048i16 {
        instr16.insert(base | (k & 0x0FFF) as u16, factory(k));
    }
}

pub struct AvrDecoder {
    instr16: HashMap<u16, Instruction>
}

impl AvrDecoder {

    pub fn new() -> AvrDecoder {
        use instruction_set::RegIncDec;
        use instruction_set::Instruction::*;

        let mut instr16 = HashMap::new();

        instr16.insert(0, Nop);
        instr16.insert(0b_1001_0101_0000_1000_u16, Ret);
        instr16.insert(0b_1001_0101_1001_1000_u16, Break);
        instr16.insert(0b_1001_0100_1111_1000_u16, Cli);
        instr16.insert(0b_1001_0101_1101_1000_u16, Elpm0);

        add_instr5(&mut instr16, 0b_1001_0100_0000_0000_u16, |d| Com { d });
        add_instr5(&mut instr16, 0b_1001_0100_0000_0101_u16, |d| Asr { d });
        add_instr5(&mut instr16, 0b_1001_0100_0000_0110_u16, |d| Lsr { d });
        add_instr5(&mut instr16, 0b_1001_0100_0000_0111_u16, |d| Ror { d });
        add_instr5(&mut instr16, 0b_1001_0100_0000_1010_u16, |d| Dec { d });
        add_instr5(&mut instr16, 0b_1001_0100_0000_0011_u16, |d| Inc { d });
        add_instr5(&mut instr16, 0b_1001_0000_0000_1111_u16, |r| Pop { r });
        add_instr5(&mut instr16, 0b_1001_0010_0000_1111_u16, |r| Push { r });
        add_instr5(&mut instr16, 0b_1001_0000_0000_0110_u16, |d| Elpm { d });
        add_instr5(&mut instr16, 0b_1001_0000_0000_0111_u16, |d| ElpmInc { d });
        add_instr5(&mut instr16, 0b_1001_0000_0000_1100_u16, |d| LdX { d, xop: RegIncDec::Unchanged });
        add_instr5(&mut instr16, 0b_1001_0000_0000_1101_u16, |d| LdX { d, xop: RegIncDec::Inc });
        add_instr5(&mut instr16, 0b_1001_0000_0000_1110_u16, |d| LdX { d, xop: RegIncDec::Dec });
        add_instr5(&mut instr16, 0b_1000_0000_0000_1000_u16, |d| LdY { d, yop: RegIncDec::Unchanged });
        add_instr5(&mut instr16, 0b_1001_0000_0000_1001_u16, |d| LdY { d, yop: RegIncDec::Inc });
        add_instr5(&mut instr16, 0b_1001_0000_0000_1010_u16, |d| LdY { d, yop: RegIncDec::Dec });
        add_instr5(&mut instr16, 0b_1000_0000_0000_0000_u16, |d| LdZ { d, zop: RegIncDec::Unchanged });
        add_instr5(&mut instr16, 0b_1001_0000_0000_0001_u16, |d| LdZ { d, zop: RegIncDec::Inc });
        add_instr5(&mut instr16, 0b_1001_0000_0000_0010_u16, |d| LdZ { d, zop: RegIncDec::Dec });
        add_instr5(&mut instr16, 0b_1001_0010_0000_1100_u16, |r| StX { r, xop: RegIncDec::Unchanged });
        add_instr5(&mut instr16, 0b_1001_0010_0000_1101_u16, |r| StX { r, xop: RegIncDec::Inc });
        add_instr5(&mut instr16, 0b_1001_0010_0000_1110_u16, |r| StX { r, xop: RegIncDec::Dec });
        add_instr5(&mut instr16, 0b_1000_0010_0000_1000_u16, |r| StY { r, yop: RegIncDec::Unchanged });
        add_instr5(&mut instr16, 0b_1001_0010_0000_1001_u16, |r| StY { r, yop: RegIncDec::Inc });
        add_instr5(&mut instr16, 0b_1001_0010_0000_1010_u16, |r| StY { r, yop: RegIncDec::Dec });
        add_instr5(&mut instr16, 0b_1000_0010_0000_0000_u16, |r| StZ { r, zop: RegIncDec::Unchanged });
        add_instr5(&mut instr16, 0b_1001_0010_0000_0001_u16, |r| StZ { r, zop: RegIncDec::Inc });
        add_instr5(&mut instr16, 0b_1001_0010_0000_0010_u16, |r| StZ { r, zop: RegIncDec::Dec });

        add_instr7(&mut instr16, 0b_1111_0100_0000_0000_u16, |k| Brcc { k });
        add_instr7(&mut instr16, 0b_1111_0000_0000_0000_u16, |k| Brcs { k });
        add_instr7(&mut instr16, 0b_1111_0000_0000_0001_u16, |k| Breq { k });
        add_instr7(&mut instr16, 0b_1111_0100_0000_0100_u16, |k| Brge { k });
        add_instr7(&mut instr16, 0b_1111_0100_0000_0101_u16, |k| Brhc { k });
        add_instr7(&mut instr16, 0b_1111_0000_0000_0101_u16, |k| Brhs { k });
        add_instr7(&mut instr16, 0b_1111_0100_0000_0111_u16, |k| Brid { k });
        add_instr7(&mut instr16, 0b_1111_0000_0000_0111_u16, |k| Brie { k });
        add_instr7(&mut instr16, 0b_1111_0000_0000_0100_u16, |k| Brlt { k });
        add_instr7(&mut instr16, 0b_1111_0000_0000_0010_u16, |k| Brmi { k });
        add_instr7(&mut instr16, 0b_1111_0100_0000_0001_u16, |k| Brne { k });
        add_instr7(&mut instr16, 0b_1111_0100_0000_0010_u16, |k| Brpl { k });
        add_instr7(&mut instr16, 0b_1111_0100_0000_0110_u16, |k| Brtc { k });
        add_instr7(&mut instr16, 0b_1111_0000_0000_0110_u16, |k| Brts { k });
        add_instr7(&mut instr16, 0b_1111_0100_0000_0011_u16, |k| Brvc { k });
        add_instr7(&mut instr16, 0b_1111_0000_0000_0011_u16, |k| Brvs { k });

        add_instr12(&mut instr16, 0b_1101_0000_0000_0000_u16, |k| Rcall { k });
        add_instr12(&mut instr16, 0b_1100_0000_0000_0000_u16, |k| Rjmp { k });

        add_instr26(&mut instr16, 0b_1001_0111_0000_0000_u16, |d, k| Sbiw { d, k });
        add_instr26(&mut instr16, 0b_1001_0110_0000_0000_u16, |d, k| Adiw { d, k });

        add_instr_sbix(&mut instr16, 0b_1001_1001_0000_0000_u16, |r, b| Sbic { r, b });
        add_instr_sbix(&mut instr16, 0b_1001_1011_0000_0000_u16, |r, b| Sbis { r, b });
        add_instr_sbrx(&mut instr16, 0b_1111_1100_0000_0000_u16, |r, b| Sbrc { r, b });
        add_instr_sbrx(&mut instr16, 0b_1111_1110_0000_0000_u16, |r, b| Sbrs { r, b });

        add_instr48(&mut instr16, 0b_0011_0000_0000_0000_u16, |d, k| Cpi { d, k });
        add_instr48(&mut instr16, 0b_0100_0000_0000_0000_u16, |d, k| Sbci { d, k });
        add_instr48(&mut instr16, 0b_0101_0000_0000_0000_u16, |d, k| Subi { d, k });
        add_instr48(&mut instr16, 0b_0110_0000_0000_0000_u16, |d, k| Ori { d, k });
        add_instr48(&mut instr16, 0b_0111_0000_0000_0000_u16, |d, k| Andi { d, k });
        add_instr48(&mut instr16, 0b_1110_0000_0000_0000_u16, |d, k| Ldi { d, k });

        add_movw(&mut instr16, 0b_0000_0001_0000_0000_u16, |d, r| Movw { d, r });

        add_ldssts(&mut instr16, 0b_1010_0000_0000_0000_u16, |d, k| { Lds { d, k } });
        add_ldssts(&mut instr16, 0b_1010_1000_0000_0000_u16, |r, k| { Sts { r, k } });

        add_instr55(&mut instr16, 0b_0000_0100_0000_0000_u16, |d, r| Cpc { d, r });
        add_instr55(&mut instr16, 0b_0000_1000_0000_0000_u16, |d, r| Sbc { d, r });
        add_instr55(&mut instr16, 0b_0000_1100_0000_0000_u16, |d, r| Add { d, r });
        add_instr55(&mut instr16, 0b_0001_0100_0000_0000_u16, |d, r| Cp { d, r });
        add_instr55(&mut instr16, 0b_0001_1000_0000_0000_u16, |d, r| Sub { d, r });
        add_instr55(&mut instr16, 0b_0001_1100_0000_0000_u16, |d, r| Adc { d, r });
        add_instr55(&mut instr16, 0b_0010_0100_0000_0000_u16, |d, r| Eor { d, r });
        add_instr55(&mut instr16, 0b_0010_1000_0000_0000_u16, |d, r| Or { d, r });
        add_instr55(&mut instr16, 0b_0010_1100_0000_0000_u16, |d, r| Mov { d, r });
        add_instr55(&mut instr16, 0b_0010_0000_0000_0000_u16, |d, r| And { d, r });
        add_instr55(&mut instr16, 0b_1001_1100_0000_0000_u16, |d, r| Mul { d, r });

        add_instr44(&mut instr16, 0b_0000_0010_0000_0000_u16, |d, r| Muls { d, r });
        add_instr33(&mut instr16, 0b_0000_0011_0000_0000_u16, |d, r| Mulsu { d, r });

        add_stdldd(&mut instr16, 0b_1000_0010_0000_1000_u16, |r, q| StdY { r, q });
        add_stdldd(&mut instr16, 0b_1000_0010_0000_0000_u16, |r, q| StdZ { r, q });
        add_stdldd(&mut instr16, 0b_1000_0000_0000_1000_u16, |d, q| LddY { d, q });
        add_stdldd(&mut instr16, 0b_1000_0000_0000_0000_u16, |d, q| LddZ { d, q });


        // OUT
        for r in 0..32u8 {
            for a in 0..64u8 {
                instr16.insert(
                    0b_1011_1000_0000_0000_u16 | encode_56(r, a),
                    Out { r, a });
            }
        }

        // IN
        for d in 0..32u8 {
            for a in 0..64u8 {
                instr16.insert(
                    0b_1011_0000_0000_0000_u16 | encode_56(d, a),
                    In { d, a });
            }
        }

        AvrDecoder { instr16 }
    }

    fn decode_jump_call(flash: &Vec<u8>, pos: usize, w0: u16) -> usize {
        if pos + 4 >= flash.len() {
            panic!("decode: index out of bounds: {}", pos)
        }

        let w1 = u16le(flash[pos + 2], flash[pos + 3]);

        (
            ((w0 as usize & 0b111110000) << 13)
                | ((w0 as usize & 0x1) << 16)
                | w1 as usize
        )
    }

    fn decode_ldssts16(flash: &Vec<u8>, pos: usize, w0: u16) -> (u8, u16) {
        if pos + 4 >= flash.len() {
            panic!("decode: index out of bounds: {}", pos)
        }

        let w1 = u16le(flash[pos + 2], flash[pos + 3]);

        (
            ((w0 & 0b111110000) >> 4) as u8,
            w1
        )
    }

    pub fn is_2word_instruction(opcode: u16) -> bool {
        (
            ((opcode >> 8) & 0b11111110 == 0b10010100) && (opcode & 0b1100 == 0b1100)
        ) || (
            ((opcode >> 8) & 0b11111100 == 0b10010000) && (opcode & 0b1111 == 0b0000)
        )
    }
}

impl Decoder for AvrDecoder {
    /// decode opcode at position `pos`
    ///
    /// no bounds checking performed!
    fn decode(&self, bytes: &Vec<u8>, pos: usize) -> Instruction {
        // 1001 010x JMP
        // 1001 010x CALL
        // 1001 000x LDS
        let b0: u8 = bytes[pos];
        let b1: u8 = if pos + 1 == bytes.len() { 0 } else { bytes[pos + 1] };
        let w0 = u16le(b0, b1);

        match b1 & 0b11111110 {
            0b10010100 =>
                match b0 & 0b1110 {
                    0b1100 => return Instruction::Jmp {
                        k: AvrDecoder::decode_jump_call(bytes, pos, w0) as u32 },
                    0b1110 => return Instruction::Call {
                        k: AvrDecoder::decode_jump_call(bytes, pos, w0) as u32 },
                    _ => { }
                },

            0b10010010 if b0 & 0b1111 == 0b0000 => {
                let (r, k) = AvrDecoder::decode_ldssts16(bytes, pos, w0);
                return Instruction::Sts16 { r, k }
            }

            0b10010000 if b0 & 0b1111 == 0b0000 => {
                let (d, k) = AvrDecoder::decode_ldssts16(bytes, pos, w0);
                return Instruction::Lds16 { d, k }
            }

            _ => { }
        }

        match self.instr16.get(&w0) {
            Some(instr) => *instr,
            None => Instruction::Invaild { opcode: w0 }
        }
    }
}


pub struct AvrDecoderCache {
    opcodes: Vec<Instruction>,
    decoder: AvrDecoder
}

impl AvrDecoderCache {
    pub fn new() -> AvrDecoderCache {
        AvrDecoderCache {
            opcodes: vec!(),
            decoder: AvrDecoder::new()
        }
    }

    pub fn refresh(&mut self, flash: &Vec<u8>) {
        info!(target: "avrvc.decoder", "Refreshing instruction cache ...");
        self.opcodes = (0..flash.len()).map(|pc| self.decoder.decode(flash, pc)).collect();
    }
}

impl Decoder for AvrDecoderCache {
    fn decode(&self, bytes: &Vec<u8>, pos: usize) -> Instruction {
        debug_assert!(bytes.len() == self.opcodes.len());
        self.opcodes[pos]
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use instruction_set::RegIncDec;

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

    #[test]
    fn test_sbiw() {
        let decoder = AvrDecoder::new();
        let bytes = vec![0x00u8, 0x97u8];
        let instr = decoder.decode(&bytes, 0);
        assert_eq!(instr, Instruction::Sbiw { d: 24, k: 0 });
    }

    #[test]
    fn test_brhs() {
        let decoder = AvrDecoder::new();
        let bytes = vec![0x05u8, 0xf0u8];
        let instr = decoder.decode(&bytes, 0);
        assert_eq!(instr, Instruction::Brhs { k: 0 });
    }

    #[test]
    fn test_ldz() {
        let decoder = AvrDecoder::new();
        let bytes = vec![0b00000001, 0b10010000];
        let instr = decoder.decode(&bytes, 0);
        assert_eq!(instr, Instruction::LdZ { d: 0, zop: RegIncDec::Inc });
    }
}