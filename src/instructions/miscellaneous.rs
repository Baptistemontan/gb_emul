use crate::{
    cpu::{registers::{Register, Flags}, Cpu},
    map_fetch_register,
};

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

    pub fn fetch(cpu: &mut Cpu, opcode: u8) -> Option<Self> {
        use MiscInstruction::*;
        match opcode {
            0x27 => Some(DecimalAdjustA),
            0x2F => Some(ComplementA),
            0x3F => Some(ComplementCarry),
            0x37 => Some(SetCarry),
            0x00 => Some(Nop),
            0x76 => Some(Halt),
            0x10 if cpu.advance() == 0x00 => {
                Some(Stop)
            },
            0xF3 => Some(DisableInterrupt),
            0xFB => Some(EnableInterrupt),
            _ => None,
        }
    }

    fn swap(value: u8) -> u8 {
        let lower = value & 0x0F;
        let upper = value & 0xF0;
        lower << 4 | upper >> 4
    }

    pub fn execute(self, cpu: &mut Cpu) {
        match self {
            MiscInstruction::SwapRegister(reg) => {
                // 1 wide opcode and no memory access, but 2 cycles
                // so need to put one there
                cpu.cycle();
                let value = cpu.get_reg(reg);
                let value = Self::swap(value);
                cpu.put_reg(reg, value);
            },
            MiscInstruction::SwapAddrHL => {
                // 1 wide opcode and 2 memory access, but 4 cycles
                // so need to put one there
                cpu.cycle();
                let value = cpu.get_at_hl();
                let value = Self::swap(value);
                cpu.put_at_hl(value);
            },
            MiscInstruction::DecimalAdjustA => todo!(),
            MiscInstruction::ComplementA => {
                let value = cpu.get_reg_a();
                cpu.put_reg_a(!value);
            },
            MiscInstruction::ComplementCarry => {
                let carry = cpu.get_flag(Flags::Carry);
                cpu.set_flag_to(Flags::Carry, !carry);
            },
            MiscInstruction::SetCarry => {
                cpu.set_flag(Flags::Carry);
            },
            MiscInstruction::Nop => {
                // litteraly do nothing
            },
            MiscInstruction::Halt => todo!(),
            MiscInstruction::Stop => todo!(),
            MiscInstruction::DisableInterrupt => {
                cpu.disable_interrupts();
            },
            MiscInstruction::EnableInterrupt => {
                cpu.enable_interrupts();
            },
        }
    }
}
