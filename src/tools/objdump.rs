use instruction_set::Instruction;
use decoder::AvrDecoder;


pub trait ObjDumpInstr {
    fn dump(&self) -> String;
}

impl ObjDumpInstr for Instruction {
    fn dump(&self) -> String {
        use instruction_set::Instruction::*;

        return match self {
            &Adc { d, r } => format!("adc\tr{}, r{}", d, r),
            &Add { d, r } => format!("add\tr{}, r{}", d, r),
            &Adiw { d, k } => format!("adiw\tr{}:{}, 0x{:x}", d + 1, d, k),
            &Call { k } => format!("call\t0x{:x}", k * 2),
            &Cli => String::from("cli"),
            &Eor { d, r } => format!("eor\tr{}, r{}", d, r),
            &Ldi { d, k } => format!("ldi\tr{}, 0x{:02X}", d, k),
            &In { d, a } => format!("in\tr{}, 0x{:02x}", d, a),
            &Jmp { k } => {
                if k == 0 {
                    String::from("jmp\t0")
                } else {
                    format!("jmp\t0x{:x}", k * 2)
                }
            },
            &Out { r, a } => format!("out\t0x{:02x}, r{}", a, r),
            &Pop { r } => format!("pop\tr{}", r),
            &Push { r } => format!("push\tr{}", r),
            &Ret => String::from("ret"),
            &Rjmp { k } => format!("rjmp\t.{}", k),
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