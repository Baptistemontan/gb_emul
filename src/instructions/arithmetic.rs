use std::ops::{AddAssign, SubAssign};

use crate::cpu::{
    registers::{LongRegister, Register, Registers, SetFlags, Flags},
    Cpu,
};

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
            (false, reg) => ArithmeticInstruction::AddRegister(reg),
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
            (false, reg) => ArithmeticInstruction::SubRegister(reg),
        }
    }
    fn fetch_bitwise(opcode: u8) -> Self {
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

    pub fn fetch(cpu: &mut Cpu, opcode: u8) -> Option<Self> {
        use ArithmeticInstruction::*;
        match opcode {
            0x80..=0x8F => Some(Self::fetch_add(opcode)),
            0x90..=0x9F => Some(Self::fetch_sub(opcode)),
            0xC6 => Some(AddImmediate(cpu.advance())),
            0xCE => Some(AddCarryImmediate(cpu.advance())),
            0xD6 => Some(SubImmediate(cpu.advance())),
            0xDE => Some(SubCarryImmediate(cpu.advance())),
            0xE6 => Some(AndImmediate(cpu.advance())),
            0xE8 => Some(AddSPImmediate(cpu.advance())),
            0xEE => Some(XorImmediate(cpu.advance())),
            0xF6 => Some(OrImmediate(cpu.advance())),
            0xFE => Some(CmpImmediate(cpu.advance())),
            0xA0..=0xBF => Some(Self::fetch_bitwise(opcode)),
            x if x & 0b11000110 == 0x04 => Some(Self::fetch_inc_dec(opcode)),
            x if x & 0b11000111 == 0x03 => Some(Self::fetch_inc_dec_long(opcode)),
            x if x & 0b11001111 == 0x09 => Some(Self::fetch_add_hl_long(opcode)),
            _ => None,
        }
    }

    pub fn execute(self, cpu: &mut Cpu) {
        match self {
            ArithmeticInstruction::AddImmediate(n) => {
                let a = cpu.get_reg_a();
                let (value, flags) = Self::add(a, n);
                cpu.set_flags(flags);
                cpu.put_reg_a(value);
            },
            ArithmeticInstruction::AddRegister(reg) => {
                let n = cpu.get_reg(reg);
                ArithmeticInstruction::AddImmediate(n).execute(cpu);
            },
            ArithmeticInstruction::AddAddrHL => {
                let n = cpu.get_at_hl();
                ArithmeticInstruction::AddImmediate(n).execute(cpu);
            },
            ArithmeticInstruction::SubImmediate(n) => {
                let a = cpu.get_reg_a();
                let (value, flags) = Self::sub(a, n);
                cpu.set_flags(flags);
                cpu.put_reg_a(value);
            },
            ArithmeticInstruction::SubRegister(reg) => {
                let n = cpu.get_reg(reg);
                ArithmeticInstruction::SubImmediate(n).execute(cpu);
            },
            ArithmeticInstruction::SubAddrHL => {
                let n = cpu.get_at_hl();
                ArithmeticInstruction::SubImmediate(n).execute(cpu);
            },
            ArithmeticInstruction::AddCarryImmediate(n) => {
                let a = cpu.get_reg_a();
                let carry = cpu.get_flag(Flags::Carry);
                let (value, flags) = Self::add_carry(a, n, carry);
                cpu.set_flags(flags);
                cpu.put_reg_a(value);
            },
            ArithmeticInstruction::AddCarryRegister(reg) => {
                let n = cpu.get_reg(reg);
                ArithmeticInstruction::AddCarryImmediate(n).execute(cpu);
            },
            ArithmeticInstruction::AddCarryAddrHL => {
                let n = cpu.get_at_hl();
                ArithmeticInstruction::AddCarryImmediate(n).execute(cpu);
            },
            ArithmeticInstruction::SubCarryImmediate(n) => {
                let a = cpu.get_reg_a();
                let carry = cpu.get_flag(Flags::Carry);
                let (value, flags) = Self::sub_carry(a, n, carry);
                cpu.set_flags(flags);
                cpu.put_reg_a(value);
            },
            ArithmeticInstruction::SubCarryRegister(reg) => {
                let n = cpu.get_reg(reg);
                ArithmeticInstruction::SubCarryImmediate(n).execute(cpu);
            },
            ArithmeticInstruction::SubCarryAddrHL => {
                let n = cpu.get_at_hl();
                ArithmeticInstruction::SubCarryImmediate(n).execute(cpu);
            },
            ArithmeticInstruction::AndImmediate(n) => {
                let a = cpu.get_reg_a();
                let result = a & n;
                let flags = SetFlags {
                    zero: result == 0,
                    substract: false,
                    half_carry: true,
                    carry: false
                };
                cpu.put_reg_a(result);
                cpu.set_flags(flags);
            },
            ArithmeticInstruction::AndRegister(reg) => {
                let n = cpu.get_reg(reg);
                ArithmeticInstruction::AndImmediate(n).execute(cpu);
            },
            ArithmeticInstruction::AndAddrHL => {
                let n = cpu.get_at_hl();
                ArithmeticInstruction::AndImmediate(n).execute(cpu);
            },
            ArithmeticInstruction::OrImmediate(n) => {
                let a = cpu.get_reg_a();
                let result = a | n;
                let flags = SetFlags {
                    zero: result == 0,
                    ..Default::default()
                };
                cpu.put_reg_a(result);
                cpu.set_flags(flags);
            },
            ArithmeticInstruction::OrRegister(reg) => {
                let n = cpu.get_reg(reg);
                ArithmeticInstruction::OrImmediate(n).execute(cpu);
            },
            ArithmeticInstruction::OrAddrHL => {
                let n = cpu.get_at_hl();
                ArithmeticInstruction::OrImmediate(n).execute(cpu);
            },
            ArithmeticInstruction::XorImmediate(n) => {
                let a = cpu.get_reg_a();
                let result = a ^ n;
                let flags = SetFlags {
                    zero: result == 0,
                    ..Default::default()
                };
                cpu.put_reg_a(result);
                cpu.set_flags(flags);
            },
            ArithmeticInstruction::XorRegister(reg) => {
                let n = cpu.get_reg(reg);
                ArithmeticInstruction::XorImmediate(n).execute(cpu);
            },
            ArithmeticInstruction::XorAddrHL => {
                let n = cpu.get_at_hl();
                ArithmeticInstruction::XorImmediate(n).execute(cpu);
            },
            ArithmeticInstruction::CmpImmediate(n) => {
                let a = cpu.get_reg_a();
                let (_, flags) = Self::sub(a, n);
                cpu.set_flags(flags);
            },
            ArithmeticInstruction::CmpRegister(reg) => {
                let n = cpu.get_reg(reg);
                ArithmeticInstruction::CmpImmediate(n).execute(cpu);
            },
            ArithmeticInstruction::CmpAddrHL => {
                let n = cpu.get_at_hl();
                ArithmeticInstruction::CmpImmediate(n).execute(cpu);
            },
            ArithmeticInstruction::IncRegister(reg) => {
                let value = cpu.get_reg(reg);
                let carry = cpu.get_flag(Flags::Carry);
                let (value, flags) = Self::inc(value, carry);
                cpu.set_flags(flags);
                cpu.put_reg(reg, value);
            },
            ArithmeticInstruction::IncAddrHL => {
                let value = cpu.get_at_hl();
                let carry = cpu.get_flag(Flags::Carry);
                let (value, flags) = Self::inc(value, carry);
                cpu.set_flags(flags);
                cpu.put_at_hl(value);
            },
            ArithmeticInstruction::DecRegister(reg) => {
                let value = cpu.get_reg(reg);
                let carry = cpu.get_flag(Flags::Carry);
                let (value, flags) = Self::dec(value, carry);
                cpu.set_flags(flags);
                cpu.put_reg(reg, value);
            },
            ArithmeticInstruction::DecAddrHL => {
                let value = cpu.get_at_hl();
                let carry = cpu.get_flag(Flags::Carry);
                let (value, flags) = Self::dec(value, carry);
                cpu.set_flags(flags);
                cpu.put_at_hl(value);
            },
            ArithmeticInstruction::AddHL(lr) => {
                todo!()
            },
            ArithmeticInstruction::AddSPImmediate(n) => {
                todo!()
            },
            ArithmeticInstruction::IncLongRegister(reg) => {
                cpu.get_long_reg(reg).add_assign(1);
            },
            ArithmeticInstruction::DecLongRegister(reg) => {
                cpu.get_long_reg(reg).sub_assign(1);
            },
        }
    }


    fn add(a: u8, b: u8) -> (u8, SetFlags) {
        let half_carry = a & 0x0F + b & 0x0F > 0x0F;
        let (value, carry) = a.overflowing_add(b);
        let zero = value == 0;
        let flags = SetFlags {
            half_carry,
            carry,
            zero,
            substract: false
        };
        (value, flags)
    }

    fn add_carry(a: u8, b: u8, carry: bool) -> (u8, SetFlags) {
        if carry {
            match (a, b) {
                (0xFF, 0xFF) => (0xFF, SetFlags { carry: true, half_carry: true , ..Default::default()}),
                (0xFF, x) | (x, 0xFF) => Self::add(x + 1, 0xFF),
                _ => Self::add(a + 1, b)
            }
        } else {
            Self::add(a, b)
        }
    }

    fn sub(a: u8, b: u8) -> (u8, SetFlags) {
        todo!()
    }

    fn sub_carry(a: u8, b: u8, carry: bool) -> (u8, SetFlags) {
        todo!()
    }

    fn inc(a: u8, carry: bool) -> (u8, SetFlags) {
        let (value, mut flags) = Self::add(a, 1);
        flags.carry = carry;
        (value, flags)
    }

    fn dec(a: u8, carry: bool) -> (u8, SetFlags) {
        let (value, mut flags) = Self::sub(a, 1);
        flags.carry = carry;
        (value, flags)
    }

}
