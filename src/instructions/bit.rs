use crate::cpu::{registers::Register, Cpu};

use super::FetchRegister;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitInstruction {
    /// BIT b, r
    ///
    /// Test bit b in register r.
    ///
    /// Cycles: 8
    BitRegister(Register, u8),
    /// BIT b, (HL)
    ///
    /// Test bit b at the absolute address HL.
    ///
    /// Cycles: 16
    BitAddrHL(u8),
    /// SET b, r
    ///
    /// Set bit b in register r.
    ///
    /// Cycles: 8
    SetRegister(Register, u8),
    /// SET b, r
    ///
    /// Set bit b at the absolute address HL.
    ///
    /// Cycles: 16
    SetAddrHL(u8),
    /// RES b, r
    ///
    /// reset bit b in register r.
    ///
    /// Cycles: 8
    ResRegister(Register, u8),
    /// RES b, r
    ///
    /// reset bit b at the absolute address HL.
    ///
    /// Cycles: 16
    ResAddrHL(u8),
}

impl BitInstruction {

    pub fn fetch_prefixed(_: &Cpu, opcode_id: u8, reg: FetchRegister) -> Option<Self> {
        use BitInstruction::*;

        let bit = (opcode_id & 0b00111000) >> 3;


        match opcode_id {
            x if x & 0b11000111 == 0x40 => Some(reg.map(|reg| BitRegister(reg, bit), BitAddrHL(bit))),
            x if x & 0b11000111 == 0x80 => Some(reg.map(|reg| ResRegister(reg, bit), ResAddrHL(bit))),
            x if x & 0b11000111 == 0xC0 => Some(reg.map(|reg| SetRegister(reg, bit), SetAddrHL(bit))),
            _ => None
        }
    }

    pub const fn size(self) -> u16 {
        1
    }

    pub const fn cycles(self) -> u8 {
        match self {
            BitInstruction::BitRegister(_, _) => 8,
            BitInstruction::BitAddrHL(_) => 16,
            BitInstruction::SetRegister(_, _) => 8,
            BitInstruction::SetAddrHL(_) => 16,
            BitInstruction::ResRegister(_, _) => 8,
            BitInstruction::ResAddrHL(_) => 16,
        }
    }

    pub fn execute(self, cpu: &mut Cpu) {
        todo!()
    }
}
