use instruction_set::Instruction;
use decoder::AvrDecoder;
use decoder::Decoder;
use instruction_set::RegIncDec;


pub trait ObjDumpInstr {
    fn dump(&self) -> String;
}

fn format_calljmp(mnemonic: &str, k: u32) -> String {
    if k == 0 {
        format!("{}\t0", mnemonic)
    } else {
        format!("{}\t0x{:x}", mnemonic, k * 2)
    }
}

fn incdec(xyz: &str, op: RegIncDec) -> String {
    match op {
        RegIncDec::Unchanged => String::from(xyz),
        RegIncDec::Inc => format!("{}+", xyz),
        RegIncDec::Dec => format!("-{}", xyz),
    }
}

impl ObjDumpInstr for Instruction {
    fn dump(&self) -> String {
        use instruction_set::Instruction::*;

        return match self {
            &Adc { d, r } => format!("adc\tr{}, r{}", d, r),
            &Add { d, r } => format!("add\tr{}, r{}", d, r),
            &Adiw { d, k } => format!("adiw\tr{}, 0x{:02x}", d, k),
            &And { d, r } => format!("and\tr{}, r{}", d, r),
            &Andi { d, k } => format!("andi\tr{}, 0x{:02X}", d, k),
            &Break => String::from("break"),

            &Brcc { k } => format!("brcc\t.{:+}", k * 2),
            &Brcs { k } => format!("brcs\t.{:+}", k * 2),
            &Breq { k } => format!("breq\t.{:+}", k * 2),
            &Brge { k } => format!("brge\t.{:+}", k * 2),
            &Brhc { k } => format!("brhc\t.{:+}", k * 2),
            &Brhs { k } => format!("brhs\t.{:+}", k * 2),
            &Brid { k } => format!("brid\t.{:+}", k * 2),
            &Brie { k } => format!("brie\t.{:+}", k * 2),
            &Brlt { k } => format!("brlt\t.{:+}", k * 2),
            &Brmi { k } => format!("brmi\t.{:+}", k * 2),
            &Brne { k } => format!("brne\t.{:+}", k * 2),
            &Brpl { k } => format!("brpl\t.{:+}", k * 2),
            &Brtc { k } => format!("brtc\t.{:+}", k * 2),
            &Brts { k } => format!("brts\t.{:+}", k * 2),
            &Brvc { k } => format!("brvc\t.{:+}", k * 2),
            &Brvs { k } => format!("brvs\t.{:+}", k * 2),

            &Call { k } => format_calljmp("call", k),
            &Cli => String::from("cli"),
            &Cp { d, r } => format!("cp\tr{}, r{}", d, r),
            &Cpc { d, r } => format!("cpc\tr{}, r{}", d, r),
            &Cpi { d, k } => format!("cpi\tr{}, 0x{:02X}", d, k),
            &Com { d } => format!("com\tr{}", d),
            &Elpm0 => String::from("elpm"),
            &Elpm { d } => format!("elpm\tr{}, Z", d),
            &ElpmInc { d } => format!("elpm\tr{}, Z+", d),
            &Eor { d, r } => format!("eor\tr{}, r{}", d, r),
            &LdX { d, xop } => format!("ld\tr{}, {}", d, incdec("X", xop)),
            &LdY { d, yop } => format!("ld\tr{}, {}", d, incdec("Y", yop)),
            &LdZ { d, zop } => format!("ld\tr{}, {}", d, incdec("Z", zop)),
            &LddY { q, d } => format!("ldd\tr{}, Y+{}", d, q),
            &LddZ { q, d } => format!("ldd\tr{}, Z+{}", d, q),
            &Ldi { d, k } => format!("ldi\tr{}, 0x{:02X}", d, k),
            &Lds { d, k } => format!("lds\tr{}, 0x{:02X}", d, k),
            &Lds16 { d, k } => format!("lds\tr{}, 0x{:04X}", d, k),
            &In { d, a } => format!("in\tr{}, 0x{:02x}", d, a),
            &Jmp { k } => format_calljmp("jmp", k),
            &Nop => String::from("nop"),
            &Mov { d, r } => format!("mov\tr{}, r{}", d, r),
            &Movw { d, r } => format!("movw\tr{}, r{}", d, r),
            &Mul { d, r } => format!("mul\tr{}, r{}", d, r),
            &Muls { d, r } => format!("muls\tr{}, r{}", d, r),
            &Mulsu { d, r } => format!("mulsu\tr{}, r{}", d, r),
            &Or { d, r } => format!("or\tr{}, r{}", d, r),
            &Ori { d, k } => format!("ori\tr{}, 0x{:02X}", d, k),
            &Out { r, a } => format!("out\t0x{:02x}, r{}", a, r),
            &Pop { r } => format!("pop\tr{}", r),
            &Push { r } => format!("push\tr{}", r),
            &Ret => String::from("ret"),
            &Rjmp { k } => format!("rjmp\t.{:+}", k * 2),
            &Sbci { d, k } => format!("sbci\tr{}, 0x{:02X}", d, k),
            &Sbiw { d, k } => format!("sbiw\tr{}, 0x{:02x}", d, k),
            &Sbrc { r, b } => format!("sbrc\tr{}, {}", r, b),
            &Sbrs { r, b } => format!("sbrs\tr{}, {}", r, b),
            &StX { r, xop } => format!("st\t{}, r{}", incdec("X", xop), r),
            &StY { r, yop } => format!("st\t{}, r{}", incdec("Y", yop), r),
            &StZ { r, zop } => format!("st\t{}, r{}", incdec("Z", zop), r),
            &StdY { q, r } => format!("std\tY+{}, r{}", q, r),
            &StdZ { q, r } => format!("std\tZ+{}, r{}", q, r),
            &Sts { r, k } => format!("sts\t0x{:02X}, r{}", k, r),
            &Sts16 { r, k } => format!("sts\t0x{:04X}, r{}", k, r),
            &Invaild { .. } => format!("invalid"),
        }
    }
}

pub fn objdump(bytes: &Vec<u8>) -> String {
    let mut pos = 0;
    let decoder = AvrDecoder::new();
    let mut result: Vec<String> = Vec::with_capacity(bytes.len() / 2);
    while pos < bytes.len() {
        let instr = decoder.decode(&bytes, pos);
        result.push(instr.dump());
        pos += instr.size() * 2;
    }
    result.join("\n")
}