use instruction_set::Instruction;
use decoder::AvrDecoder;


pub trait ObjDumpInstr {
    fn dump(&self) -> String;
}

fn format_calljmp(mnemonic: &str, k: usize) -> String {
    if k == 0 {
        format!("{}\t0", mnemonic)
    } else {
        format!("{}\t0x{:x}", mnemonic, k * 2)
    }
}

fn format_ldd(mnemonic: &str, yz: char, q: u8, d: u8) -> String {
    if q == 0 {
        format!("{}\tr{}, {}", &mnemonic[..2], d, yz)
    } else {
        format!("{}\tr{}, {}+{}", mnemonic, d, yz, q)
    }
}

fn format_std(mnemonic: &str, yz: char, q: u8, r: u8) -> String {
    if q == 0 {
        format!("{}\t{}, r{}", &mnemonic[..2], yz, r)
    } else {
        format!("{}\t{}+{}, r{}", mnemonic, yz, q, r)
    }
}


impl ObjDumpInstr for Instruction {
    fn dump(&self) -> String {
        use instruction_set::Instruction::*;

        return match self {
            &Adc { d, r } => format!("adc\tr{}, r{}", d, r),
            &Add { d, r } => format!("add\tr{}, r{}", d, r),
            &Adiw { d, k } => format!("adiw\tr{}, 0x{:02x}", d, k),
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
            &Eor { d, r } => format!("eor\tr{}, r{}", d, r),
            &LddY { q, d } => format_ldd("ldd", 'Y', q, d),
            &LddZ { q, d } => format_ldd("ldd", 'Z', q, d),
            &Ldi { d, k } => format!("ldi\tr{}, 0x{:02X}", d, k),
            &In { d, a } => format!("in\tr{}, 0x{:02x}", d, a),
            &Jmp { k } => format_calljmp("jmp", k),
            &Nop => String::from("nop"),
            &Mov { d, r } => format!("mov\tr{}, r{}", d, r),
            &Out { r, a } => format!("out\t0x{:02x}, r{}", a, r),
            &Pop { r } => format!("pop\tr{}", r),
            &Push { r } => format!("push\tr{}", r),
            &Ret => String::from("ret"),
            &Rjmp { k } => format!("rjmp\t.{:+}", k * 2),
            &Sbci { d, k } => format!("sbci\tr{}, 0x{:02X}", d, k),
            &Sbiw { d, k } => format!("sbiw\tr{}, 0x{:02x}", d, k),
            &Sbrc { r, b } => format!("sbrc\tr{}, {}", r, b),
            &StdY { q, r } => format_std("std", 'Y', q, r),
            &StdZ { q, r } => format_std("std", 'Z', q, r),
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