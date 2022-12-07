use crate::{cpu::{registers::Register, Cpu}, map_fetch_register};

use super::FetchRegister;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MiscInstruction {
    /// SWAP r
    ///
    /// Swap upper & lower nibles of r.
    ///
    /// Cycles: 8
    SwapRegister(Register),
    /// SWAP (HL)
    ///
    /// Swap upper & lower nibles of the value at the absolute adress HL.
    ///
    /// Cycles: 16
    SwapAddrHL,
    /// DAA
    ///
    /// Decimal adjust register A.
    ///
    /// This instruction adjusts register A so that the correct
    /// representation of Binary Coded Decimal (BCD) is obtained.
    ///
    /// Cycles: 4
    DecimalAdjustA,
    /// CPL
    ///
    /// Complement A register. (Flip all bits.)
    ///
    /// Cycles: 4
    ComplementA,
    /// CCF
    ///
    /// Complement carry flag.
    /// If C flag is set, then reset it.
    /// If C flag is reset, then set it
    ///
    /// Cycles: 4
    ComplementCarry,
    /// SCF
    ///
    /// Set Carry flag.
    ///
    /// Cycles: 4
    SetCarry,
    /// NOP
    ///
    /// No operation.
    ///
    /// Cycles: 4
    Nop,
    /// HALT
    ///
    /// Power down CPU until an interrupt occurs.
    /// Use this when ever possible to reduce energy consumption.
    ///
    /// Cycles: 4
    Halt,
    /// STOP
    ///
    /// Halt CPU & LCD display until button pressed.
    ///
    /// Cycles: 4
    Stop,
    /// DI
    ///
    /// This instruction disables interrupts but not immediately.
    /// Interrupts are disabled after instruction after DI is executed.
    ///
    /// Cycles: 4
    DisableInterrupt,
    /// EI
    ///
    /// This instruction enables interrupts but not immediately.
    /// Interrupts are enabled after instruction after EI is executed.
    ///
    /// Cycles: 4
    EnableInterrupt,
}

impl MiscInstruction {

    pub fn fetch_prefixed(_: &Cpu, opcode_id: u8, reg: FetchRegister) -> Option<Self> {
        use MiscInstruction::*;
        (opcode_id == 0x30).then(|| map_fetch_register!(reg, SwapRegister, SwapAddrHL))
    }

    pub fn fetch(cpu: &Cpu, opcode: u8) -> Option<Self> {
        use MiscInstruction::*;
        match opcode {
            0x27 => Some(DecimalAdjustA),
            0x2F => Some(ComplementA),
            0x3F => Some(ComplementCarry),
            0x37 => Some(SetCarry),
            0x00 => Some(Nop),
            0x76 => Some(Halt),
            0x10 if cpu.get_relative(1) == 0x00 => Some(Stop),
            0xF3 => Some(DisableInterrupt),
            0xFB => Some(EnableInterrupt),
            _ => None
        }
    }

    pub const fn size(self) -> u16 {
        match self {
            MiscInstruction::Stop => 2,
            _ => 1,
        }
    }

    pub const fn cycles(self) -> u8 {
        match self {
            MiscInstruction::SwapRegister(_) => 8,
            MiscInstruction::SwapAddrHL => 16,
            MiscInstruction::DecimalAdjustA => 4,
            MiscInstruction::ComplementA => 4,
            MiscInstruction::ComplementCarry => 4,
            MiscInstruction::SetCarry => 4,
            MiscInstruction::Nop => 4,
            MiscInstruction::Halt => 4,
            MiscInstruction::Stop => 4,
            MiscInstruction::DisableInterrupt => 4,
            MiscInstruction::EnableInterrupt => 4,
        }
    }

    pub fn execute(self, cpu: &mut Cpu) {
        todo!()
    }
}

