use crate::cpu::{registers::{Flags, Registers, SetFlags}, Cpu};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlFlowCondition {
    NotZero,
    Zero,
    NoCarry,
    Carry,
}

impl From<u8> for ControlFlowCondition {
    fn from(cc: u8) -> Self {
        use ControlFlowCondition::*;
        match cc {
            0 => NotZero,
            1 => Zero,
            2 => NoCarry,
            _ => Carry
        }
    }
}

impl ControlFlowCondition {
    fn check_condition(self, flags: SetFlags) -> bool {
        match self {
            ControlFlowCondition::NotZero => !flags.zero,
            ControlFlowCondition::Zero => flags.zero,
            ControlFlowCondition::NoCarry => !flags.carry,
            ControlFlowCondition::Carry => flags.carry,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlFlowInstruction {
    // Jumps
    /// JP nn
    ///
    /// Jump to address nn
    ///
    /// Cycles: 12
    JumpImmediate(u16),
    /// JP cc, nn
    ///
    /// If the condition is true, jump to the specified address nn.
    ///
    /// Cycles: 12
    JumpImmediateCondition(ControlFlowCondition, u16),
    /// JP (HL)
    ///
    /// Jump to the absolute address HL.
    ///
    /// Cycles: 4
    JumpAddrHL,
    /// JR n
    ///
    /// Add n to the current address and jump to it.
    ///
    /// Cycles: 8
    JumpImmediateRelative(i8),
    /// JR cc, n
    ///
    /// If the condition is true, add n to the current address and jump to it.
    ///
    /// Cycles: 8
    JumpRelativeCondition(ControlFlowCondition, i8),

    // Calls
    /// CALL nn
    ///
    /// Push address of next instruction onto stack and then jump to the specified address nn.
    ///
    /// Cycles: 12
    CallImmediate(u16),
    /// CALL cc, nn
    ///
    /// If the condition is true, call the specified address nn.
    ///
    /// Cycles: 12
    CallImmediateCondition(ControlFlowCondition, u16),

    // Restarts
    /// RST n
    ///
    /// Push present address onto stack. Jump to address $0000 + n.
    ///
    /// Cycles: 32
    Reset(u8),

    // Returns
    /// RET
    ///
    /// Pop two bytes from stack & jump to that address.
    ///
    /// Cycles: 8
    Return,
    /// RET cc
    ///
    /// If the condition is true, return.
    ///
    /// Cycles: 8
    ReturnCondition(ControlFlowCondition),
    /// RETI
    ///
    /// Return and enable interrupts.
    ///
    /// Cycles: 8
    ReturnEnableInterrupt,
}

impl ControlFlowInstruction {

    pub fn fetch(cpu: &Cpu, opcode: u8) -> Option<Self> {
        use ControlFlowInstruction::*;
        
        let delta = cpu.get_relative(1);
        let delta = i8::from_be_bytes([delta]);

        let addr = cpu.get_next_long();
        let cc = ((opcode & 0b00011000) >> 3).into();
        match opcode {
            0xC3 => Some(JumpImmediate(addr)),
            x if x & 0b11100111 == 0xC2 => Some(JumpImmediateCondition(cc, addr)),
            0xE9 => Some(JumpAddrHL),
            0x18 => Some(JumpImmediateRelative(delta)),
            x if x & 0b11100111 == 0x20 => Some(JumpRelativeCondition(cc, delta)),
            0xCD => Some(CallImmediate(addr)),
            x if x & 0b11100111 == 0xC4 => Some(CallImmediateCondition(cc, addr)),
            x if x & 0b11000111 == 0b11000111 => Some(Reset(x & 0b00111000)),
            0xC9 => Some(Return),
            x if x & 0b11100111 == 0xC0 => Some(ReturnCondition(cc)),
            0xD9 => Some(ReturnEnableInterrupt),
            _ => None
        }
    }

    pub const fn size(self) -> u16 {
        match self {
            ControlFlowInstruction::JumpImmediate(_) => 3,
            ControlFlowInstruction::JumpImmediateCondition(_, _) => 3,
            ControlFlowInstruction::JumpAddrHL => 1,
            ControlFlowInstruction::JumpImmediateRelative(_) => 2,
            ControlFlowInstruction::JumpRelativeCondition(_, _) => 2,
            ControlFlowInstruction::CallImmediate(_) => 3,
            ControlFlowInstruction::CallImmediateCondition(_, _) => 3,
            ControlFlowInstruction::Reset(_) => 2,
            ControlFlowInstruction::Return => 1,
            ControlFlowInstruction::ReturnCondition(_) => 1,
            ControlFlowInstruction::ReturnEnableInterrupt => 1,
        }
    }

    pub const fn cycles(self) -> u8 {
        match self {
            ControlFlowInstruction::JumpImmediate(_) => 12,
            ControlFlowInstruction::JumpImmediateCondition(_, _) => 12,
            ControlFlowInstruction::JumpAddrHL => 4,
            ControlFlowInstruction::JumpImmediateRelative(_) => 8,
            ControlFlowInstruction::JumpRelativeCondition(_, _) => 8,
            ControlFlowInstruction::CallImmediate(_) => 12,
            ControlFlowInstruction::CallImmediateCondition(_, _) => 12,
            ControlFlowInstruction::Reset(_) => 32,
            ControlFlowInstruction::Return => 8,
            ControlFlowInstruction::ReturnCondition(_) => 8,
            ControlFlowInstruction::ReturnEnableInterrupt => 8,
        }
    }

    pub fn execute(self, cpu: &mut Cpu) {
        todo!()
    }
}

