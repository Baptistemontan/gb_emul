use crate::cpu::{
    registers::{Flags, Register, SetFlags},
    Cpu,
};

use super::FetchRegister;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetBit {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eighth,
}

impl From<u8> for TargetBit {
    fn from(bit: u8) -> Self {
        use TargetBit::*;
        match bit % 8 {
            0 => First,
            1 => Second,
            2 => Third,
            3 => Fourth,
            4 => Fifth,
            5 => Sixth,
            6 => Seventh,
            _ => Eighth,
        }
    }
}

impl TargetBit {
    pub fn get_mask(self) -> u8 {
        match self {
            TargetBit::First => 1 << 0,
            TargetBit::Second => 1 << 1,
            TargetBit::Third => 1 << 2,
            TargetBit::Fourth => 1 << 3,
            TargetBit::Fifth => 1 << 4,
            TargetBit::Sixth => 1 << 5,
            TargetBit::Seventh => 1 << 6,
            TargetBit::Eighth => 1 << 7,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitInstruction {
    /// BIT b, r
    ///
    /// Test bit b in register r.
    ///
    /// Cycles: 8
    BitRegister(Register, TargetBit),
    /// BIT b, (HL)
    ///
    /// Test bit b at the absolute address HL.
    ///
    /// Cycles: 16
    BitAddrHL(TargetBit),
    /// SET b, r
    ///
    /// Set bit b in register r.
    ///
    /// Cycles: 8
    SetRegister(Register, TargetBit),
    /// SET b, r
    ///
    /// Set bit b at the absolute address HL.
    ///
    /// Cycles: 16
    SetAddrHL(TargetBit),
    /// RES b, r
    ///
    /// reset bit b in register r.
    ///
    /// Cycles: 8
    ResRegister(Register, TargetBit),
    /// RES b, r
    ///
    /// reset bit b at the absolute address HL.
    ///
    /// Cycles: 16
    ResAddrHL(TargetBit),
}

impl BitInstruction {
    pub fn fetch_prefixed(_: &Cpu, opcode_id: u8, reg: FetchRegister) -> Option<Self> {
        use BitInstruction::*;

        let bit = opcode_id >> 3;
        let bit = bit.into();

        match opcode_id & 0b11000111 {
            0x40 => Some(reg.map(|reg| BitRegister(reg, bit), BitAddrHL(bit))),
            0x80 => Some(reg.map(|reg| ResRegister(reg, bit), ResAddrHL(bit))),
            0xC0 => Some(reg.map(|reg| SetRegister(reg, bit), SetAddrHL(bit))),
            _ => None,
        }
    }

    fn test_bit(byte: u8, target_bit: TargetBit, carry: bool) -> SetFlags {
        let mask = target_bit.get_mask();
        let zero = byte & mask == 0;
        SetFlags {
            zero,
            substract: false,
            half_carry: true,
            carry,
        }
    }

    pub fn execute(self, cpu: &mut Cpu) {
        // every bit instructions are 1 byte instruction and don't access memory,
        // but they are all either 2 / 4 cycles
        // 1 cycle already happened at fetch, so add another so it remains 0 / 2 cycles.
        // Opcodes that take 2 more cycles are memory fetch which are read then write,
        // so the cycle count is good !
        cpu.cycle();
        match self {
            BitInstruction::BitRegister(reg, bit) => {
                let value = cpu.get_reg(reg);
                let carry = cpu.get_flag(Flags::Carry);
                let flags = Self::test_bit(value, bit, carry);
                cpu.set_flags(flags);
            }
            BitInstruction::BitAddrHL(bit) => {
                let value = cpu.get_at_hl();
                let carry = cpu.get_flag(Flags::Carry);
                let flags = Self::test_bit(value, bit, carry);
                cpu.set_flags(flags);
            }
            BitInstruction::SetRegister(reg, bit) => {
                let reg = cpu.get_reg_mut(reg);
                let mask = bit.get_mask();
                *reg |= mask;
            }
            BitInstruction::SetAddrHL(bit) => {
                let mut value = cpu.get_at_hl();
                let mask = bit.get_mask();
                value |= mask;
                cpu.put_at_hl(value);
            }
            BitInstruction::ResRegister(reg, bit) => {
                let reg = cpu.get_reg_mut(reg);
                let mask = bit.get_mask();
                *reg &= !mask;
            }
            BitInstruction::ResAddrHL(bit) => {
                let mut value = cpu.get_at_hl();
                let mask = bit.get_mask();
                value &= !mask;
                cpu.put_at_hl(value);
            }
        }
    }
}
