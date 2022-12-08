use crate::cpu::{
    registers::{Flags, Registers, SetFlags, LongRegister},
    Cpu,
};

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
            _ => Carry,
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
    /// Cycles: 16
    JumpImmediate(u16),
    /// JP cc, nn
    ///
    /// If the condition is true, jump to the specified address nn.
    ///
    /// Cycles: 16/12
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
    /// Cycles: 12
    JumpImmediateRelative(i8),
    /// JR cc, n
    ///
    /// If the condition is true, add n to the current address and jump to it.
    ///
    /// Cycles: 12/8
    JumpRelativeCondition(ControlFlowCondition, i8),

    // Calls
    /// CALL nn
    ///
    /// Push address of next instruction onto stack and then jump to the specified address nn.
    ///
    /// Cycles: 24
    CallImmediate(u16),
    /// CALL cc, nn
    ///
    /// If the condition is true, call the specified address nn.
    ///
    /// Cycles: 24/12
    CallImmediateCondition(ControlFlowCondition, u16),

    // Restarts
    /// RST n
    ///
    /// Push present address onto stack. Jump to address $0000 + n.
    ///
    /// Cycles: 16
    Reset(u8),

    // Returns
    /// RET
    ///
    /// Pop two bytes from stack & jump to that address.
    ///
    /// Cycles: 16
    Return,
    /// RET cc
    ///
    /// If the condition is true, return.
    ///
    /// Cycles: 20/8
    ReturnCondition(ControlFlowCondition),
    /// RETI
    ///
    /// Return and enable interrupts.
    ///
    /// Cycles: 16
    ReturnEnableInterrupt,
}

impl ControlFlowInstruction {

    pub fn fetch(cpu: &mut Cpu, opcode: u8) -> Option<Self> {
        use ControlFlowInstruction::*;
        let cc = ((opcode & 0b00011000) >> 3).into();
        match opcode {
            0xC3 => Some(JumpImmediate(cpu.advance_long())),
            x if x & 0b11100111 == 0xC2 => Some(JumpImmediateCondition(cc, cpu.advance_long())),
            0xE9 => Some(JumpAddrHL),
            0x18 => Some(JumpImmediateRelative(i8::from_be_bytes([cpu.advance()]))),
            x if x & 0b11100111 == 0x20 => Some(JumpRelativeCondition(cc, i8::from_be_bytes([cpu.advance()]))),
            0xCD => Some(CallImmediate(cpu.advance_long())),
            x if x & 0b11100111 == 0xC4 => Some(CallImmediateCondition(cc, cpu.advance_long())),
            x if x & 0b11000111 == 0b11000111 => Some(Reset(x & 0b00111000)),
            0xC9 => Some(Return),
            x if x & 0b11100111 == 0xC0 => Some(ReturnCondition(cc)),
            0xD9 => Some(ReturnEnableInterrupt),
            _ => None,
        }
    }

    fn exec_cc(this: Self, cc: ControlFlowCondition, cpu: &mut Cpu) -> bool {
        let flags = cpu.get_flags();
        let jump = cc.check_condition(flags);
        if jump {
            this.execute(cpu);
        }
        jump
    }

    pub fn execute(self, cpu: &mut Cpu) {
        match self {
            ControlFlowInstruction::JumpImmediate(addr) => {
                cpu.set_pc(addr);
                cpu.cycle(); // 3 cycle fetch, but 4 cycle instruction
            },
            ControlFlowInstruction::JumpImmediateCondition(cc, addr) => {
                Self::exec_cc(ControlFlowInstruction::JumpImmediate(addr), cc, cpu);
            },
            ControlFlowInstruction::JumpAddrHL => {
                let addr = cpu.get_long_reg(LongRegister::HL);
                cpu.set_pc(addr);
            },
            ControlFlowInstruction::JumpImmediateRelative(delta) => {
                cpu.move_by(delta.into());
                cpu.cycle();
            },
            ControlFlowInstruction::JumpRelativeCondition(cc, delta) => {
                Self::exec_cc(ControlFlowInstruction::JumpImmediateRelative(delta), cc, cpu);
            },
            ControlFlowInstruction::CallImmediate(addr) => {
                // 3 width instruction + 2 Write, need one more cycle.
                cpu.cycle();
                let pc = cpu.get_pc();
                cpu.push_stack(pc);
                cpu.set_pc(addr);
            },
            ControlFlowInstruction::CallImmediateCondition(cc, addr) => {
                Self::exec_cc(ControlFlowInstruction::CallImmediate(addr), cc, cpu);
            },
            ControlFlowInstruction::Reset(addr) => {
                // turn this instruction into a call
                // 2 less cycle happen during fetch
                // so cycle count is good
                ControlFlowInstruction::CallImmediate(addr.into()).execute(cpu);
            },
            ControlFlowInstruction::Return => {
                let addr = cpu.pop_stack();
                ControlFlowInstruction::JumpImmediate(addr).execute(cpu);
            },
            ControlFlowInstruction::ReturnCondition(cc) => {
                // weird, but if condition not met take 2 cycles, but opcode is 1 wide
                // cycle count is good on jump, but on NOP still lack 1 cycle 
                let jumped = Self::exec_cc(ControlFlowInstruction::Return, cc, cpu);
                if !jumped {
                    cpu.cycle();
                }
            },
            ControlFlowInstruction::ReturnEnableInterrupt => {
                ControlFlowInstruction::Return.execute(cpu);
                cpu.enable_interrupts();
            },
        }
    }
}
