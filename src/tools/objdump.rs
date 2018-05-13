use instruction_set::Instruction;
use decoder::AvrDecoder;

pub fn objdump_instr(instr: Instruction) -> String {
    return match instr {
        Instruction::Call { k } => format!("call\t0x{:x}", k),
        Instruction::Cli => String::from("cli"),
        Instruction::Eor { d, r } => format!("eor\tr{}, r{}", d, r),
        Instruction::Ldi { d, k } => format!("ldi\tr{}, 0x{:02X}", d, k),
        Instruction::In { d, a } => format!("in\tr{}, 0x{:02x}", d, a),
        Instruction::Jmp { k } => {
            if k == 0 {
                String::from("jmp\t0")
            } else {
                format!("jmp\t0x{:x}", k)
            }
        },
        Instruction::Out { r, a } => format!("out\t0x{:02x}, r{}", a, r),
        Instruction::Pop { r } => format!("pop\tr{}", r),
        Instruction::Push { r } => format!("push\tr{}", r),
        Instruction::Ret => String::from("ret"),
        Instruction::Rjmp { k } => format!("rjmp\t.{}", k),
        Instruction::Invaild { .. } => format!("invalid"),
    };
}

pub fn objdump(bytes: &Vec<u8>) -> String {
    let mut pos = 0;
    let decoder = AvrDecoder::new();
    let mut result: Vec<String> = Vec::with_capacity(bytes.len() / 2);
    while pos < bytes.len() {
        let instr = decoder.decode(&bytes, pos);
        result.push(objdump_instr(instr));
        pos += instr.size();
    }
    result.join("\n")
}