use std::fmt;

pub struct Instruction {
    pub opcode: Opcode,
    mask: u16,
    code: u16,
    pub operand_encoding: &'static OperandEncoding
}

pub struct OperandEncoding {
    reg_x_mask: u16,
    reg_x_right_shift: u8,
    reg_y_mask: u16,
    reg_y_right_shift: u8,
    address_mask: u16,
    imm_mask: u16
}

static OPERAND_NONE: OperandEncoding = OperandEncoding {
    reg_x_mask: 0x0000, reg_x_right_shift:0,
    reg_y_mask: 0x0000, reg_y_right_shift:0,
    address_mask: 0x0000,
    imm_mask: 0x0000
};

static OPERAND_ADDRESS: OperandEncoding = OperandEncoding {
    reg_x_mask: 0x0000, reg_x_right_shift:0,
    reg_y_mask: 0x0000, reg_y_right_shift:0,
    address_mask: 0x0FFF,
    imm_mask: 0x0000
};

static OPERAND_REG_XY: OperandEncoding = OperandEncoding {
    reg_x_mask: 0x0F00, reg_x_right_shift:8,
    reg_y_mask: 0x00F0, reg_y_right_shift:4,
    address_mask: 0x0000,
    imm_mask: 0x0000
};

static OPERAND_REG_X: OperandEncoding = OperandEncoding {
    reg_x_mask: 0x0F00, reg_x_right_shift:8,
    reg_y_mask: 0x0000, reg_y_right_shift:0,
    address_mask: 0x0000,
    imm_mask: 0x0000
};

static OPERAND_REG_X_IMM: OperandEncoding = OperandEncoding {
    reg_x_mask: 0x0F00, reg_x_right_shift:8,
    reg_y_mask: 0x0000, reg_y_right_shift:0,
    address_mask: 0x0000,
    imm_mask: 0x00FF
};

static OPERAND_REG_X_Y_IMM: OperandEncoding = OperandEncoding {
    reg_x_mask: 0x0F00, reg_x_right_shift:8,
    reg_y_mask: 0x00F0, reg_y_right_shift:4,
    address_mask: 0x0000,
    imm_mask: 0x000F
};

static OPS: [Instruction;35] = [
    Instruction{mask: 0xFFFF, code: 0x0000, opcode: Opcode::Invalid, operand_encoding: &OPERAND_NONE},
    Instruction{mask: 0xFFFF, code: 0x00E0, opcode: Opcode::Cls,     operand_encoding: &OPERAND_NONE},
    Instruction{mask: 0xFFFF, code: 0x00EE, opcode: Opcode::Ret,     operand_encoding: &OPERAND_NONE},
    Instruction{mask: 0xF000, code: 0x1000, opcode: Opcode::Jp,      operand_encoding: &OPERAND_ADDRESS},
    Instruction{mask: 0xF000, code: 0x2000, opcode: Opcode::Call,    operand_encoding: &OPERAND_ADDRESS},
    Instruction{mask: 0xF000, code: 0x3000, opcode: Opcode::Sei,     operand_encoding: &OPERAND_REG_X_IMM},
    Instruction{mask: 0xF000, code: 0x4000, opcode: Opcode::Snei,    operand_encoding: &OPERAND_REG_X_IMM},
    Instruction{mask: 0xF00F, code: 0x5000, opcode: Opcode::Se,      operand_encoding: &OPERAND_REG_XY},
    Instruction{mask: 0xF000, code: 0x6000, opcode: Opcode::Ldi,     operand_encoding: &OPERAND_REG_X_IMM},
    Instruction{mask: 0xF000, code: 0x7000, opcode: Opcode::Addi,    operand_encoding: &OPERAND_REG_X_IMM},
    Instruction{mask: 0xF00F, code: 0x8000, opcode: Opcode::Ld,      operand_encoding: &OPERAND_REG_XY},
    Instruction{mask: 0xF00F, code: 0x8001, opcode: Opcode::Or,      operand_encoding: &OPERAND_REG_XY},
    Instruction{mask: 0xF00F, code: 0x8002, opcode: Opcode::And,     operand_encoding: &OPERAND_REG_XY},
    Instruction{mask: 0xF00F, code: 0x8003, opcode: Opcode::Xor,     operand_encoding: &OPERAND_REG_XY},
    Instruction{mask: 0xF00F, code: 0x8004, opcode: Opcode::Add,     operand_encoding: &OPERAND_REG_XY},
    Instruction{mask: 0xF00F, code: 0x8005, opcode: Opcode::Sub,     operand_encoding: &OPERAND_REG_XY},
    Instruction{mask: 0xF00F, code: 0x8006, opcode: Opcode::Shr,     operand_encoding: &OPERAND_REG_XY},
    Instruction{mask: 0xF00F, code: 0x8007, opcode: Opcode::Subn,    operand_encoding: &OPERAND_REG_XY},
    Instruction{mask: 0xF00F, code: 0x800E, opcode: Opcode::Shl,     operand_encoding: &OPERAND_REG_XY},
    Instruction{mask: 0xF00F, code: 0x9000, opcode: Opcode::Sne,     operand_encoding: &OPERAND_REG_XY},
    Instruction{mask: 0xF000, code: 0xA000, opcode: Opcode::Seti,    operand_encoding: &OPERAND_ADDRESS},
    Instruction{mask: 0xF000, code: 0xB000, opcode: Opcode::B,       operand_encoding: &OPERAND_NONE},
    Instruction{mask: 0xF000, code: 0xC000, opcode: Opcode::Rnd,     operand_encoding: &OPERAND_REG_X_IMM},
    Instruction{mask: 0xF000, code: 0xD000, opcode: Opcode::Drw,     operand_encoding: &OPERAND_REG_X_Y_IMM},
    Instruction{mask: 0xF0FF, code: 0xE09E, opcode: Opcode::Skp,     operand_encoding: &OPERAND_REG_X},
    Instruction{mask: 0xF0FF, code: 0xE0A1, opcode: Opcode::Sknp,    operand_encoding: &OPERAND_REG_X},
    Instruction{mask: 0xF0FF, code: 0xF007, opcode: Opcode::Lddt,    operand_encoding: &OPERAND_REG_X},
    Instruction{mask: 0xF0FF, code: 0xF00A, opcode: Opcode::Ldkp,    operand_encoding: &OPERAND_REG_X},
    Instruction{mask: 0xF0FF, code: 0xF015, opcode: Opcode::Setdt,   operand_encoding: &OPERAND_REG_X},
    Instruction{mask: 0xF0FF, code: 0xF018, opcode: Opcode::Setst,   operand_encoding: &OPERAND_REG_X},
    Instruction{mask: 0xF0FF, code: 0xF01E, opcode: Opcode::Addir,   operand_encoding: &OPERAND_REG_X},
    Instruction{mask: 0xF0FF, code: 0xF029, opcode: Opcode::Setis,   operand_encoding: &OPERAND_REG_X},
    Instruction{mask: 0xF0FF, code: 0xF033, opcode: Opcode::Ibcd,    operand_encoding: &OPERAND_REG_X},
    Instruction{mask: 0xF0FF, code: 0xF055, opcode: Opcode::Stri,    operand_encoding: &OPERAND_REG_X},
    Instruction{mask: 0xF0FF, code: 0xF065, opcode: Opcode::Fetch,   operand_encoding: &OPERAND_REG_X}];

#[derive(Debug)]
pub enum Opcode {
    Invalid, Cls, Ret, Jp, Call, Sei, Snei, Se, Ldi, Addi,
    Ld, Or, And, Xor, Add, Sub, Shr, Subn, Shl, Sne,
    Seti, B, Rnd, Drw, Skp, Sknp, Lddt, Ldkp, Setdt, Setst,
    Addir, Setis, Ibcd, Stri, Fetch
}

impl Instruction {
    pub fn decode(instruction_bytes: u16) -> &'static Instruction {
        for x in 0..OPS.len() {
            if Instruction::test(&OPS[x], instruction_bytes) {
                return &OPS[x];
            }
        }
        return &OPS[0]; //Unknown opcode
    }

    fn test(test_code: &Instruction, instruction_bytes: u16) -> bool {
        instruction_bytes & test_code.mask == test_code.code
    }

    pub fn get_operands(&self, instruction_bytes: u16) -> Operands{
        Operands {
            reg_x: ((self.operand_encoding.reg_x_mask & instruction_bytes) >> self.operand_encoding.reg_x_right_shift) as u8,
            reg_y: ((self.operand_encoding.reg_y_mask & instruction_bytes) >> self.operand_encoding.reg_y_right_shift) as u8,
            address: (self.operand_encoding.address_mask & instruction_bytes) as u16,
            imm: (self.operand_encoding.imm_mask & instruction_bytes) as u8
        }
    }
}

pub struct Operands {
    pub reg_x: u8,
    pub reg_y: u8,
    pub address: u16,
    pub imm: u8
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.opcode)
    }
}
