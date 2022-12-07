use crate::cpu::{registers::{LongRegister, Register, Registers}, Cpu};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArithmeticInstruction {
    // 8-bits arithmetic
    /// ADD A, n
    ///
    /// Add to the 8-bit register A, the immediate data n.
    ///
    /// Cycles: 8
    AddImmediate(u8),
    /// ADD A, r
    ///
    /// Add to the 8-bit register A, the value of the 8-bit register r.
    ///
    /// Cycles: 4
    AddRegister(Register),
    /// ADD A, (HL)
    ///
    /// Add to the 8-bit register A, the value at the absolute address specified by HL.
    ///
    /// Cycles: 8
    AddAddrHL,
    /// SUB A, n
    ///
    /// Sub to the 8-bit register A, the immediate data n.
    ///
    /// Cycles: 8
    SubImmediate(u8),
    /// SUB A, r
    ///
    /// Sub to the 8-bit register A, the value of the 8-bit register r.
    ///
    /// Cycles: 4
    SubRegister(Register),
    /// SUB A, (HL)
    ///
    /// Sub to the 8-bit register A, the value at the absolute address specified by HL.
    ///
    /// Cycles: 8
    SubAddrHL,
    /// ADC A, n
    ///
    /// Add to the 8-bit register A, the immediate data n + the carry flag.
    ///
    /// Cycles: 8
    AddCarryImmediate(u8),
    /// ADC A, r
    ///
    /// Add to the 8-bit register A, the value of the 8-bit register r + the carry flag.
    ///
    /// Cycles: 4
    AddCarryRegister(Register),
    /// ADC A, (HL)
    ///
    /// Add to the 8-bit register A, the value at the absolute address specified by HL + the carry flag.
    ///
    /// Cycles: 8
    AddCarryAddrHL,
    /// SBC A, n
    ///
    /// Sub to the 8-bit register A, the immediate data n + the carry flag.
    ///
    /// Cycles: 8
    SubCarryImmediate(u8),
    /// SBC A, r
    ///
    /// Sub to the 8-bit register A, the value of the 8-bit register r + the carry flag.
    ///
    /// Cycles: 4
    SubCarryRegister(Register),
    /// SBC A, (HL)
    ///
    /// Sub to the 8-bit register A, the value at the absolute address specified by HL + the carry flag.
    ///
    /// Cycles: 8
    SubCarryAddrHL,
    /// AND n
    ///
    /// Logically AND the 8-bit register A with the immediate value n, result in A
    ///
    /// Cycles: 8
    AndImmediate(u8),
    /// AND r
    ///
    /// Logically AND the 8-bit register A with the 8-bit register r, result in A
    ///
    /// Cycles: 4
    AndRegister(Register),
    /// AND (HL)
    ///
    /// Logically AND the 8-bit register A with the value at the absolute address HL, result in A
    ///
    /// Cycles: 8
    AndAddrHL,
    /// OR n
    ///
    /// Logically OR the 8-bit register A with the immediate value n, result in A
    ///
    /// Cycles: 8
    OrImmediate(u8),
    /// OR r
    ///
    /// Logically OR the 8-bit register A with the 8-bit register r, result in A
    ///
    /// Cycles: 4
    OrRegister(Register),
    /// OR (HL)
    ///
    /// Logically OR the 8-bit register A with the value at the absolute address HL, result in A
    ///
    /// Cycles: 8
    OrAddrHL,
    /// XOR n
    ///
    /// Logically XOR the 8-bit register A with the immediate value n, result in A
    ///
    /// Cycles: 8
    XorImmediate(u8),
    /// XOR r
    ///
    /// Logically XOR the 8-bit register A with the 8-bit register r, result in A
    ///
    /// Cycles: 4
    XorRegister(Register),
    /// XOR (HL)
    ///
    /// Logically XOR the 8-bit register A with the value at the absolute address HL, result in A
    ///
    /// Cycles: 8
    XorAddrHL,
    /// CMP n
    ///
    /// Compare the 8-bit register A with the immediate value n, basically A - n but the result is thrown away.
    ///
    /// Cycles: 8
    CmpImmediate(u8),
    /// CMP r
    ///
    /// Compare the 8-bit register A with the 8-bit register r, basically A - r but the result is thrown away.
    ///
    /// Cycles: 4
    CmpRegister(Register),
    /// CMP (HL)
    ///
    /// Compare the 8-bit register A with the value at the absolute address HL, basically A - (HL) but the result is thrown away.
    ///
    /// Cycles: 8
    CmpAddrHL,
    /// INC r
    ///
    /// Increment register r.
    ///
    /// Cycles: 4
    IncRegister(Register),
    /// INC (HL)
    ///
    /// Increment the value at the absolute address HL.
    ///
    /// Cycles: 12
    IncAddrHL,
    /// DEC r
    ///
    /// Decrement register r.
    ///
    /// Cycles: 4
    DecRegister(Register),
    /// DEC (HL)
    ///
    /// Decrement the value at the absolute address HL.
    ///
    /// Cycles: 12
    DecAddrHL,

    // 16-bits arithmetic
    /// ADD HL, lr
    ///
    /// Add to the 16-bit register HL, the value from the 16-bit register lr.
    ///
    /// Cycles: 8
    AddHL(LongRegister),
    /// ADD SP, n
    ///
    /// Add to the 16-bit register SP, the immediate 8-bit value n.
    ///
    /// Cycles: 16
    AddSPImmediate(u8),
    /// INC lr
    ///
    /// Increment register lr.
    ///
    /// Cycles: 8
    IncLongRegister(LongRegister),
    /// DEC lr
    ///
    /// Decrement register lr.
    ///
    /// Cycles: 8
    DecLongRegister(LongRegister),
}

impl ArithmeticInstruction {
    fn fetch_add(opcode: u8) -> Self {
        let i = opcode & 0b00000111;
        let with_carry = opcode & 0b00001000 != 0;
        let reg = Registers::REGISTERS[i as usize];
        match (with_carry, reg) {
            (true, Register::F) => ArithmeticInstruction::AddCarryAddrHL,
            (false, Register::F) => ArithmeticInstruction::AddAddrHL,
            (true, reg) => ArithmeticInstruction::AddCarryRegister(reg),
            (false, reg) => ArithmeticInstruction::AddRegister(reg)
        }
    }
    fn fetch_sub(opcode: u8) -> Self {
        let i = opcode & 0b00000111;
        let with_carry = opcode & 0b00001000 != 0;
        let reg = Registers::REGISTERS[i as usize];
        match (with_carry, reg) {
            (true, Register::F) => ArithmeticInstruction::SubCarryAddrHL,
            (false, Register::F) => ArithmeticInstruction::SubAddrHL,
            (true, reg) => ArithmeticInstruction::SubCarryRegister(reg),
            (false, reg) => ArithmeticInstruction::SubRegister(reg)
        }
    }
    fn fetch_bitwise(opcode: u8) -> Self{
        use ArithmeticInstruction::*;
        let i = opcode & 0b00000111;
        let op = (opcode & 0b00011000) >> 3;
        let reg = Registers::REGISTERS[i as usize];
        match (op, reg) {
            (0, Register::F) => AndAddrHL,
            (1, Register::F) => XorAddrHL,
            (2, Register::F) => OrAddrHL,
            (3, Register::F) => CmpAddrHL,
            (0, reg) => AndRegister(reg),
            (1, reg) => XorRegister(reg),
            (2, reg) => OrRegister(reg),
            // match any here, impossible to have other value but still need to cover them in the match
            (_, reg) => CmpRegister(reg),
        }
    }

    fn fetch_inc_dec(opcode: u8) -> Self {
        let dec = opcode & 0x01 == 0x01;
        let i = (opcode & 0b00011100) >> 2;
        let reg = Registers::REGISTERS[i as usize];
        match (dec, reg) {
            (true, Register::F) => ArithmeticInstruction::DecAddrHL,
            (false, Register::F) => ArithmeticInstruction::IncAddrHL,
            (true, reg) => ArithmeticInstruction::DecRegister(reg),
            (false, reg) => ArithmeticInstruction::IncRegister(reg),
        }
    }

    fn fetch_inc_dec_long(opcode: u8) -> Self {
        let dec = opcode & 0b00001000 != 0;
        let i = (opcode & 0b00110000) >> 4;
        let reg = Registers::LONG_REGISTERS[i as usize];
        if dec {
            ArithmeticInstruction::DecLongRegister(reg)
        } else {
            ArithmeticInstruction::IncLongRegister(reg)
        }
    }

    fn fetch_add_hl_long(opcode: u8) -> Self {
        let i = (opcode & 0b00110000) >> 4;
        let reg = Registers::LONG_REGISTERS[i as usize];
        ArithmeticInstruction::AddHL(reg)
    }

    pub fn fetch(cpu: &Cpu, opcode: u8) -> Option<Self> {
        use ArithmeticInstruction::*;

        let n = cpu.get_relative(1);
        match opcode {
            0x80..=0x8F => Some(Self::fetch_add(opcode)),
            0x90..=0x9F => Some(Self::fetch_sub(opcode)),
            0xC6 => Some(AddImmediate(n)),
            0xCE => Some(AddCarryImmediate(n)),
            0xD6 => Some(SubImmediate(n)),
            0xDE => Some(SubCarryImmediate(n)),
            0xE6 => Some(AndImmediate(n)),
            0xE8 => Some(AddSPImmediate(n)),
            0xEE => Some(XorImmediate(n)),
            0xF6 => Some(OrImmediate(n)),
            0xFE => Some(CmpImmediate(n)),
            0xA0..=0xBF => Some(Self::fetch_bitwise(opcode)),
            x if x & 0b11000110 == 0x04 => Some(Self::fetch_inc_dec(opcode)),
            x if x & 0b11000111 == 0x03 => Some(Self::fetch_inc_dec_long(opcode)),
            x if x & 0b11001111 == 0x09 => Some(Self::fetch_add_hl_long(opcode)),
            _ => None
        }
    }
    
    pub const fn size(self) -> u16 {
        match self {
            ArithmeticInstruction::AddImmediate(_) => 2,
            ArithmeticInstruction::AddRegister(_) => 1,
            ArithmeticInstruction::AddAddrHL => 1,
            ArithmeticInstruction::SubImmediate(_) => 2,
            ArithmeticInstruction::SubRegister(_) => 1,
            ArithmeticInstruction::SubAddrHL => 1,
            ArithmeticInstruction::AddCarryImmediate(_) => 2,
            ArithmeticInstruction::AddCarryRegister(_) => 1,
            ArithmeticInstruction::AddCarryAddrHL => 1,
            ArithmeticInstruction::SubCarryImmediate(_) => 2,
            ArithmeticInstruction::SubCarryRegister(_) => 1,
            ArithmeticInstruction::SubCarryAddrHL => 1,
            ArithmeticInstruction::AndImmediate(_) => 2,
            ArithmeticInstruction::AndRegister(_) => 1,
            ArithmeticInstruction::AndAddrHL => 1,
            ArithmeticInstruction::OrImmediate(_) => 2,
            ArithmeticInstruction::OrRegister(_) => 1,
            ArithmeticInstruction::OrAddrHL => 1,
            ArithmeticInstruction::XorImmediate(_) => 2,
            ArithmeticInstruction::XorRegister(_) => 1,
            ArithmeticInstruction::XorAddrHL => 1,
            ArithmeticInstruction::CmpImmediate(_) => 2,
            ArithmeticInstruction::CmpRegister(_) => 1,
            ArithmeticInstruction::CmpAddrHL => 1,
            ArithmeticInstruction::IncRegister(_) => 1,
            ArithmeticInstruction::IncAddrHL => 1,
            ArithmeticInstruction::DecRegister(_) => 1,
            ArithmeticInstruction::DecAddrHL => 1,
            ArithmeticInstruction::AddHL(_) => 1,
            ArithmeticInstruction::AddSPImmediate(_) => 1,
            ArithmeticInstruction::IncLongRegister(_) => 1,
            ArithmeticInstruction::DecLongRegister(_) => 1,
        }
    }

    pub const fn cycles(self) -> u8 {
        match self {
            ArithmeticInstruction::AddImmediate(_) => 8,
            ArithmeticInstruction::AddRegister(_) => 4,
            ArithmeticInstruction::AddAddrHL => 8,
            ArithmeticInstruction::SubImmediate(_) => 8,
            ArithmeticInstruction::SubRegister(_) => 4,
            ArithmeticInstruction::SubAddrHL => 8,
            ArithmeticInstruction::AddCarryImmediate(_) => 8,
            ArithmeticInstruction::AddCarryRegister(_) => 4,
            ArithmeticInstruction::AddCarryAddrHL => 8,
            ArithmeticInstruction::SubCarryImmediate(_) => 8,
            ArithmeticInstruction::SubCarryRegister(_) => 4,
            ArithmeticInstruction::SubCarryAddrHL => 8,
            ArithmeticInstruction::AndImmediate(_) => 8,
            ArithmeticInstruction::AndRegister(_) => 4,
            ArithmeticInstruction::AndAddrHL => 8,
            ArithmeticInstruction::OrImmediate(_) => 8,
            ArithmeticInstruction::OrRegister(_) => 4,
            ArithmeticInstruction::OrAddrHL => 8,
            ArithmeticInstruction::XorImmediate(_) => 8,
            ArithmeticInstruction::XorRegister(_) => 4,
            ArithmeticInstruction::XorAddrHL => 8,
            ArithmeticInstruction::CmpImmediate(_) => 8,
            ArithmeticInstruction::CmpRegister(_) => 4,
            ArithmeticInstruction::CmpAddrHL => 8,
            ArithmeticInstruction::IncRegister(_) => 4,
            ArithmeticInstruction::IncAddrHL => 12,
            ArithmeticInstruction::DecRegister(_) => 4,
            ArithmeticInstruction::DecAddrHL => 12,
            ArithmeticInstruction::AddHL(_) => 8,
            ArithmeticInstruction::AddSPImmediate(_) => 16,
            ArithmeticInstruction::IncLongRegister(_) => 8,
            ArithmeticInstruction::DecLongRegister(_) => 8,
        }
    }

    pub fn execute(self, cpu: &mut Cpu) {
        todo!()
    }
}
