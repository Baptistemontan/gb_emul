use crate::{
    cpu::{
        registers::{Flags, Register, SetFlags},
        Cpu,
    },
    map_fetch_register,
};

use super::FetchRegister;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RotateShiftInstruction {
    /// RLCA
    ///
    /// Rotate A left. Old bit 7 to Carry flag.
    ///
    /// Cycles: 4
    RotateLeftCarryA,
    /// RLA
    ///
    /// Rotate A left through Carry flag.
    ///
    /// Cycles: 4
    RotateLeftA,
    /// RRCA
    ///
    /// Rotate A right. Old bit 0 to Carry flag.
    ///
    /// Cycles: 4
    RotateRightCarryA,
    /// RRA
    ///
    /// Rotate A right through Carry flag.
    ///
    /// Cycles: 4
    RotateRightA,
    /// RLC r
    ///
    /// Rotate r left. Old bit 7 to Carry flag.
    ///
    /// Cycles: 8
    RotateLeftCarryRegister(Register),
    /// RLC (HL)
    ///
    /// Rotate the value at the absolute address HL left. Old bit 7 to Carry flag.
    ///
    /// Cycles: 16
    RotateLeftCarryAddrHL,
    /// RL r
    ///
    /// Rotate r left through Carry flag.
    ///
    /// Cycles: 8
    RotateLeftRegister(Register),
    /// RL (HL)
    ///
    /// Rotate the value at the absolute address HL left through Carry flag.
    ///
    /// Cycles: 16
    RotateLeftAddrHL,
    /// RRC r
    ///
    /// Rotate r right. Old bit 7 to Carry flag.
    ///
    /// Cycles: 8
    RotateRightCarryRegister(Register),
    /// RRC (HL)
    ///
    /// Rotate the value at the absolute address HL right. Old bit 7 to Carry flag.
    ///
    /// Cycles: 16
    RotateRightCarryAddrHL,
    /// RR r
    ///
    /// Rotate r right through Carry flag.
    ///
    /// Cycles: 8
    RotateRightRegister(Register),
    /// RR (HL)
    ///
    /// Rotate the value at the absolute address HL right through Carry flag.
    ///
    /// Cycles: 16
    RotateRightAddrHL,
    /// SLA r
    ///
    /// Shift r left into Carry. LSB of n set to 0.
    ///
    /// Cycles: 8
    ShiftLeftRegister(Register),
    /// SLA (HL)
    ///
    /// Shift the value at the absolute address HL left into Carry. LSB of n set to 0.
    ///
    /// Cycles: 16
    ShiftLeftAddrHL,
    /// SRA r
    ///
    /// Shift r right into Carry. MSB doesn't change.
    ///
    /// Cycles: 8
    ShiftRightRegisterSigned(Register),
    /// SRA (HL)
    ///
    /// Shift the value at the absolute address HL right into Carry. MSB set to zero.
    ///
    /// Cycles: 16
    ShiftRightAddrHLSigned,
    /// SRL r
    ///
    /// Shift r right into Carry. MSB doesn't change.
    ///
    /// Cycles: 8
    ShiftRightRegister(Register),
    /// SRL (HL)
    ///
    /// Shift the value at the absolute address HL right into Carry. MSB set to zero.
    ///
    /// Cycles: 16
    ShiftRightAddrHL,
}

impl RotateShiftInstruction {
    pub const fn fetch_prefixed(_: &Cpu, opcode_id: u8, reg: FetchRegister) -> Option<Self> {
        use RotateShiftInstruction::*;
        match opcode_id {
            // Rotate left
            // 0x00 => Some(reg.map(RotateLeftCarryRegister, RotateLeftCarryAddrHL)),
            0x00 => Some(map_fetch_register!(
                reg,
                RotateLeftCarryRegister,
                RotateLeftCarryAddrHL
            )),
            0x10 => Some(map_fetch_register!(
                reg,
                RotateLeftRegister,
                RotateLeftAddrHL
            )),
            // Rotate right
            0x08 => Some(map_fetch_register!(
                reg,
                RotateRightCarryRegister,
                RotateRightCarryAddrHL
            )),
            0x18 => Some(map_fetch_register!(
                reg,
                RotateRightRegister,
                RotateRightAddrHL
            )),
            // Shift left
            0x20 => Some(map_fetch_register!(reg, ShiftLeftRegister, ShiftLeftAddrHL)),
            // Shift right with MSB unchanged
            0x28 => Some(map_fetch_register!(
                reg,
                ShiftRightRegisterSigned,
                ShiftRightAddrHLSigned
            )),
            // Shift right with MSB = 0
            0x38 => Some(map_fetch_register!(
                reg,
                ShiftRightRegister,
                ShiftRightAddrHL
            )),

            _ => None,
        }
    }

    pub const fn fetch(_: &Cpu, opcode: u8) -> Option<Self> {
        use RotateShiftInstruction::*;

        match opcode {
            0x07 => Some(RotateLeftCarryA),
            0x17 => Some(RotateLeftA),
            0x0F => Some(RotateRightCarryA),
            0x1F => Some(RotateRightA),
            _ => None,
        }
    }

    pub fn execute(self, cpu: &mut Cpu) {
        // all opcodes are either not prefixed and just operate on A and take 4 cycles
        // or are prefixed and take 8 / 16 cycles
        // so no cycle adjust needed
        match self {
            RotateShiftInstruction::RotateLeftCarryA => {
                let value = cpu.get_reg_a();
                let (value, flags) = Self::rotate_carry(value, true);
                cpu.put_reg_a(value);
                cpu.set_flags(flags);
            }
            RotateShiftInstruction::RotateLeftA => {
                let value = cpu.get_reg_a();
                let carry = cpu.get_flag(Flags::Carry);
                let (value, flags) = Self::rotate(value, carry, true);
                cpu.put_reg_a(value);
                cpu.set_flags(flags);
            }
            RotateShiftInstruction::RotateRightCarryA => {
                let value = cpu.get_reg_a();
                let (value, flags) = Self::rotate_carry(value, false);
                cpu.put_reg_a(value);
                cpu.set_flags(flags);
            }
            RotateShiftInstruction::RotateRightA => {
                let value = cpu.get_reg_a();
                let carry = cpu.get_flag(Flags::Carry);
                let (value, flags) = Self::rotate(value, carry, false);
                cpu.put_reg_a(value);
                cpu.set_flags(flags);
            }
            RotateShiftInstruction::RotateLeftCarryRegister(reg) => {
                let value = cpu.get_reg(reg);
                let (value, flags) = Self::rotate_carry(value, true);
                cpu.put_reg(reg, value);
                cpu.set_flags(flags);
            }
            RotateShiftInstruction::RotateLeftCarryAddrHL => {
                let value = cpu.get_at_hl();
                let (value, flags) = Self::rotate_carry(value, true);
                cpu.put_at_hl(value);
                cpu.set_flags(flags);
            }
            RotateShiftInstruction::RotateLeftRegister(reg) => {
                let value = cpu.get_reg(reg);
                let carry = cpu.get_flag(Flags::Carry);
                let (value, flags) = Self::rotate(value, carry, true);
                cpu.put_reg(reg, value);
                cpu.set_flags(flags);
            }
            RotateShiftInstruction::RotateLeftAddrHL => {
                let value = cpu.get_at_hl();
                let carry = cpu.get_flag(Flags::Carry);
                let (value, flags) = Self::rotate(value, carry, true);
                cpu.put_at_hl(value);
                cpu.set_flags(flags);
            }
            RotateShiftInstruction::RotateRightCarryRegister(reg) => {
                let value = cpu.get_reg(reg);
                let (value, flags) = Self::rotate_carry(value, false);
                cpu.put_reg(reg, value);
                cpu.set_flags(flags);
            }
            RotateShiftInstruction::RotateRightCarryAddrHL => {
                let value = cpu.get_at_hl();
                let (value, flags) = Self::rotate_carry(value, false);
                cpu.put_at_hl(value);
                cpu.set_flags(flags);
            }
            RotateShiftInstruction::RotateRightRegister(reg) => {
                let value = cpu.get_reg(reg);
                let carry = cpu.get_flag(Flags::Carry);
                let (value, flags) = Self::rotate(value, carry, false);
                cpu.put_reg(reg, value);
                cpu.set_flags(flags);
            }
            RotateShiftInstruction::RotateRightAddrHL => {
                let value = cpu.get_at_hl();
                let carry = cpu.get_flag(Flags::Carry);
                let (value, flags) = Self::rotate(value, carry, false);
                cpu.put_at_hl(value);
                cpu.set_flags(flags);
            }
            RotateShiftInstruction::ShiftLeftRegister(reg) => {
                let value = cpu.get_reg(reg);
                let (value, flags) = Self::shift(value, false, true);
                cpu.put_reg(reg, value);
                cpu.set_flags(flags);
            }
            RotateShiftInstruction::ShiftLeftAddrHL => {
                let value = cpu.get_at_hl();
                let (value, flags) = Self::shift(value, false, true);
                cpu.put_at_hl(value);
                cpu.set_flags(flags);
            }
            RotateShiftInstruction::ShiftRightRegisterSigned(reg) => {
                let value = cpu.get_reg(reg);
                let (value, flags) = Self::shift(value, true, false);
                cpu.put_reg(reg, value);
                cpu.set_flags(flags);
            }
            RotateShiftInstruction::ShiftRightAddrHLSigned => {
                let value = cpu.get_at_hl();
                let (value, flags) = Self::shift(value, true, false);
                cpu.put_at_hl(value);
                cpu.set_flags(flags);
            }
            RotateShiftInstruction::ShiftRightRegister(reg) => {
                let value = cpu.get_reg(reg);
                let (value, flags) = Self::shift(value, false, false);
                cpu.put_reg(reg, value);
                cpu.set_flags(flags);
            }
            RotateShiftInstruction::ShiftRightAddrHL => {
                let value = cpu.get_at_hl();
                let (value, flags) = Self::shift(value, false, false);
                cpu.put_at_hl(value);
                cpu.set_flags(flags);
            }
        }
    }

    // RotateCarry means 8 bits rotation, the bits are rotated and one is copied to carry
    // Rotate means 9 bits rotation: shift, set empty bit to carry, then set carry to popped bit
    fn rotate_carry(n: u8, left: bool) -> (u8, SetFlags) {
        let (value, carry) = if left {
            let carry = n & 0b10000000 != 0;
            let value = u8::rotate_left(n, 1);
            (value, carry)
        } else {
            let carry = n & 0b00000001 != 0;
            let value = u8::rotate_right(n, 1);
            (value, carry)
        };
        let zero = value == 0;
        let flags = SetFlags {
            zero,
            carry,
            ..Default::default()
        };
        (value, flags)
    }

    fn rotate(n: u8, old_carry: bool, left: bool) -> (u8, SetFlags) {
        let (carry_bit, carry) = match (old_carry, left) {
            (true, true) => (0b00000001, n & 0b10000000 != 0),
            (true, false) => (0b10000000, n & 0b00000001 != 0),
            (false, true) => (0, n & 0b10000000 != 0),
            (false, false) => (0, n & 0b00000001 != 0),
        };

        let shifted = if left { n << 1 } else { n >> 1 };
        let value = shifted | carry_bit;
        let zero = value == 0;
        let flags = SetFlags {
            zero,
            carry,
            ..Default::default()
        };
        (value, flags)
    }

    fn shift(n: u8, signed: bool, left: bool) -> (u8, SetFlags) {
        let carry = if left {
            n & 0b10000000 != 0
        } else {
            n & 0b00000001 != 0
        };

        let value = match (signed, left) {
            (true, false) => {
                let signed = i8::from_be_bytes([n]);
                let [shifted] = i8::to_be_bytes(signed >> 1);
                shifted
            }
            (false, false) => n >> 1,
            (_, true) => n << 1,
        };

        let zero = value == 0;

        let flags = SetFlags {
            zero,
            carry,
            ..Default::default()
        };

        (value, flags)
    }
}
