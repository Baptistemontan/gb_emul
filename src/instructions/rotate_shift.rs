use crate::{cpu::{registers::Register, Cpu}, map_fetch_register};

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
    ShiftRightRegister(Register),
    /// SRA (HL)
    ///
    /// Shift the value at the absolute address HL right into Carry. MSB set to zero.
    ///
    /// Cycles: 16
    ShiftRightAddrHL,
    /// SRL r
    ///
    /// Shift r right into Carry. MSB doesn't change.
    ///
    /// Cycles: 8
    ShiftRightRegisterZero(Register),
    /// SRL (HL)
    ///
    /// Shift the value at the absolute address HL right into Carry. MSB set to zero.
    ///
    /// Cycles: 16
    ShiftRightAddrHLZero,
}

impl RotateShiftInstruction {


    pub const fn fetch_prefixed(_: &Cpu, opcode_id: u8, reg: FetchRegister) -> Option<Self> {
        use RotateShiftInstruction::*;
        match opcode_id {
            // Rotate left
            // 0x00 => Some(reg.map(RotateLeftCarryRegister, RotateLeftCarryAddrHL)),
            0x00 => Some(map_fetch_register!(reg, RotateLeftCarryRegister, RotateLeftCarryAddrHL)),
            0x10 => Some(map_fetch_register!(reg, RotateLeftRegister, RotateLeftAddrHL)),
            // Rotate right
            0x08 => Some(map_fetch_register!(reg, RotateRightCarryRegister, RotateRightCarryAddrHL)),
            0x18 => Some(map_fetch_register!(reg, RotateRightRegister, RotateRightAddrHL)),
            // Shift left
            0x20 => Some(map_fetch_register!(reg, ShiftLeftRegister, ShiftLeftAddrHL)),
            // Shift right with MSB unchanged
            0x28 => Some(map_fetch_register!(reg, ShiftRightRegister, ShiftRightAddrHL)),
            // Shift right with MSB = 0
            0x38 => Some(map_fetch_register!(reg, ShiftRightRegisterZero, ShiftRightAddrHLZero)),

            _ => None
        }
    }

    pub const fn fetch(_: &Cpu, opcode: u8) -> Option<Self>{
        use RotateShiftInstruction::*;

        match opcode {
            0x07 => Some(RotateLeftCarryA),
            0x17 => Some(RotateLeftA),
            0x0F => Some(RotateRightCarryA),
            0x1F => Some(RotateRightA),
            _ => None
        }
    }

    pub const fn size(self) -> u16 {
        1
    }

    pub const fn cycles(self) -> u8 {
        match self {
            RotateShiftInstruction::RotateLeftCarryA => 4,
            RotateShiftInstruction::RotateLeftA => 4,
            RotateShiftInstruction::RotateRightCarryA => 4,
            RotateShiftInstruction::RotateRightA => 4,
            RotateShiftInstruction::RotateLeftCarryRegister(_) => 8,
            RotateShiftInstruction::RotateLeftCarryAddrHL => 16,
            RotateShiftInstruction::RotateLeftRegister(_) => 8,
            RotateShiftInstruction::RotateLeftAddrHL => 16,
            RotateShiftInstruction::RotateRightCarryRegister(_) => 8,
            RotateShiftInstruction::RotateRightCarryAddrHL => 16,
            RotateShiftInstruction::RotateRightRegister(_) => 8,
            RotateShiftInstruction::RotateRightAddrHL => 16,
            RotateShiftInstruction::ShiftLeftRegister(_) => 8,
            RotateShiftInstruction::ShiftLeftAddrHL => 16,
            RotateShiftInstruction::ShiftRightRegister(_) => 8,
            RotateShiftInstruction::ShiftRightAddrHL => 16,
            RotateShiftInstruction::ShiftRightRegisterZero(_) => 8,
            RotateShiftInstruction::ShiftRightAddrHLZero => 16,
        }
    }

    pub fn execute(self, cpu: &mut Cpu) {
        todo!()
    }

}

