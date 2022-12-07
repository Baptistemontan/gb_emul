use crate::cpu::{registers::{LongRegister, Register, Registers}, Cpu};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadInstruction {
    // 8-bits loads
    /// LD r, n: Load register (immediate)
    ///
    /// Load to the 8-bit register r, the immediate data n.
    ///
    /// Cycles: 8
    LoadImmediate(Register, u8),
    /// LD r, r’: Load register (register)
    ///
    /// Load to the 8-bit register r, data from the 8-bit register r’.
    ///
    /// Cycles: 4
    LoadRegister(Register, Register),
    /// LD r, (HL)
    ///
    /// Load to the 8-bit register r, data at address (HL).
    ///
    /// Cycles: 8
    LoadFromHLAddr(Register),
    /// LD (HL), r
    ///
    /// Load to the absolute address specified by the 16-bit register HL, data from the 8-bit register r.
    ///
    /// Cycles: 8
    LoadIntoHLAddr(Register),
    /// LD (HL), n
    ///
    /// Load to the absolute address specified by the 16-bit register HL, the immediate data n.
    ///
    /// Cycles: 12
    LoadIntoHLAddrn(u8),
    /// LD A, (lr)
    ///
    /// Load to the 8-bit register A, data at address (lr).
    ///
    /// Cycles: 8
    LoadIntoAFromAddr(LongRegister),
    /// LD A, (nn)
    ///
    /// Load to the 8-bit register A, data at address (nn).
    ///
    /// Cycles: 16
    LoadIntoAFromAddrnn(u16),
    /// LD (lr), A
    ///
    /// Load to the absolute address specified by the 16-bit register lr, data from the 8-bit register A.
    ///
    /// Cycles: 8
    LoadIntoAddrFromA(LongRegister),
    /// LD (nn), A
    ///
    /// Load to the absolute address specified by nn, data from the 8-bit register A.
    ///
    /// Cycles: 16
    LoadIntoAddrnnFromA(u16),
    /// LD A, (C)
    ///
    /// Load to the 8-bit A register, data from the address specified by the 8-bit C register
    ///
    /// The full 16-bit absolute address is obtained by setting the most significant byte to 0xFF
    /// and the least significant byte to the value of C, so the possible range is 0xFF00-0xFFFF.
    ///
    /// Cycles: 8
    LoadFromAddrCIntoA,
    /// LD (C), A
    ///
    /// Load to the address specified by the 8-bit C register, data from the 8-bit A register
    ///
    /// The full 16-bit absolute address is obtained by setting the most significant byte to 0xFF
    /// and the least significant byte to the value of C, so the possible range is 0xFF00-0xFFFF.
    ///
    /// Cycles: 8
    LoadIntoAddrCFromA,
    /// LDD A, (HL)
    ///
    /// Load to the 8-bit A register, data from the absolute address specified by the 16-bit register HL.
    /// The value of HL is decremented after the memory read.
    ///
    /// Cycles: 8
    LoadFromAddrHLIntoADec,
    /// LDD (HL), A
    ///
    /// Load to the absolute address specified by the 16-bit register HL, data from the 8-bit A register.
    /// The value of HL is decremented after the memory write.
    ///
    /// Cycles: 8
    LoadFromAIntoAddrHLDec,
    /// LDI A, (HL)
    ///
    /// Load to the 8-bit A register, data from the absolute address specified by the 16-bit register HL.
    /// The value of HL is incremented after the memory read.
    ///
    /// Cycles: 8
    LoadFromAddrHLIntoAInc,
    /// LDI (HL), A
    ///
    /// Load to the absolute address specified by the 16-bit register HL, data from the 8-bit A register.
    /// The value of HL is incremented after the memory write.
    ///
    /// Cycles: 8
    LoadFromAIntoAddrHLInc,
    /// LDH (n), A
    ///
    /// Load to the address specified by the 8-bit immediate data n, data from the 8-bit A register.
    ///
    /// The full 16-bit absolute address is obtained by setting the most significant byte to 0xFF
    /// and the least significant byte to the value of n, so the possible range is 0xFF00-0xFFFF.
    ///
    /// Cycles: 12
    LoadFromAIntoAddrn(u8),
    /// LDH A, (n)
    ///
    /// Load to the 8-bit A register, data from the address specified by the 8-bit immediate data n.
    ///
    /// The full 16-bit absolute address is obtained by setting the most significant byte to 0xFF
    /// and the least significant byte to the value of n, so the possible range is 0xFF00-0xFFFF.
    ///
    /// Cycles: 12
    LoadFromAddrnIntoA(u8),

    // 16-bits loads
    /// LD n, nn
    ///
    /// Load to the 16-bit register rr, the immediate 16-bit data nn.
    ///
    /// Cycles: 12
    LoadImmediateLong(LongRegister, u16),
    /// LDD SP, HL
    ///
    /// Load to the 16-bit SP register, data from the 16-bit HL register
    ///
    /// Cycles: 8
    LoadFromHLIntoSP,
    /// LDHL SP, n
    ///
    /// Load to the 16-bit HL register, the address specified by the 16-bit register SP + n
    ///
    /// Flags:
    /// Z - Reset.
    /// N - Reset.
    /// H - Set or reset according to operation.
    /// C - Set or reset according to operation.
    ///
    /// Cycles: 12
    LoadFromSPPlusnIntoHL(u8),
    /// LD (nn), SP
    ///
    /// Load to the absolute address specified by the 16-bit operand nn, data from the 16-bit SP register.
    ///
    /// Cycles: 20
    LoadSPIntoAddrnn(u16),
    /// PUSH nn
    ///
    /// Decrement Stack Pointer (SP) twice.
    ///
    /// Push to the stack memory, data from the 16-bit register rr.
    ///
    /// Cycles: 16
    Push(LongRegister),
    /// POP nn
    ///
    /// Pops to the 16-bit register rr, data from the stack memory.
    ///
    /// Increment Stack Pointer (SP) twice.
    ///
    /// This instruction does not do calculations that affect flags, but POP AF completely replaces the F register value,
    /// so all flags are changed based on the 8-bit data that is read from memory.
    ///
    /// Cycles: 12
    Pop(LongRegister),
}

impl LoadInstruction {

    fn fetch_load_r1_r2(opcode: u8) -> Option<Self> {
        use Register::*;
        // kind of bad, but F signifies (HL) lol
        let r2 = Registers::REGISTERS[(opcode % 8) as usize];
        let r1 = Registers::REGISTERS[((opcode >> 3) % 8) as usize];
        match (r1, r2) {
            (F, F) => None, // 0x76, this is HALT instruction
            (F, r) => Some(LoadInstruction::LoadIntoHLAddr(r)),
            (r, F) => Some(LoadInstruction::LoadFromHLAddr(r)),
            (r1, r2) => Some(LoadInstruction::LoadRegister(r1, r2))
        }
    }

    fn fetch_load_immediate(opcode: u8, n: u8) -> Self {
        let i = opcode >> 3;
        let r = Registers::REGISTERS[i as usize];
        if r == Register::F {
            LoadInstruction::LoadIntoHLAddrn(n)
        } else {
            LoadInstruction::LoadImmediate(r, n)
        }
    }

    fn fetch_load_immediate_long(opcode: u8, nn: u16) -> Self {
        let i = opcode >> 4;
        let lr = Registers::LONG_REGISTERS[i as usize];
        LoadInstruction::LoadImmediateLong(lr, nn)
    }

    fn fetch_long_register(opcode: u8) -> LongRegister {
        let i = (opcode >> 4) & 0b00000011;
        let mut lr = Registers::LONG_REGISTERS[i as usize];
        if lr == LongRegister::SP {
            lr = LongRegister::AF
        }
        lr
    }

    pub fn fetch(cpu: &Cpu, opcode: u8) -> Option<Self> {
        use LoadInstruction::*;

        match opcode {
            0x08 => Some(LoadSPIntoAddrnn(cpu.get_next_long())),
            0x22 => Some(LoadFromAIntoAddrHLInc),
            0x2A => Some(LoadFromAddrHLIntoAInc),
            0x32 => Some(LoadFromAIntoAddrHLDec),
            0x3A => Some(LoadFromAddrHLIntoADec),
            0x40..=0x7F => Self::fetch_load_r1_r2(opcode),
            0xE0 => Some(LoadFromAIntoAddrn(cpu.get_relative(1))),
            0xE2 => Some(LoadIntoAddrCFromA),
            0xEA => Some(LoadIntoAddrnnFromA(cpu.get_next_long())),
            0xF0 => Some(LoadFromAddrnIntoA(cpu.get_relative(1))),
            0xF2 => Some(LoadFromAddrCIntoA),
            0xF9 => Some(LoadFromHLIntoSP),
            0xF8 => Some(LoadFromSPPlusnIntoHL(cpu.get_relative(1))),
            0xFA => Some(LoadIntoAFromAddrnn(cpu.get_next_long())),
            x if x & 0b11001111 == 0x0A => Some(LoadIntoAFromAddr(Self::fetch_long_register(x))),
            x if x & 0b11001111 == 0x02 => Some(LoadIntoAddrFromA(Self::fetch_long_register(x))),
            x if x & 0b11000111 == 0x06 => Some(Self::fetch_load_immediate(x, cpu.get_relative(1))),
            x if x & 0b11001111 == 0x01 => Some(LoadImmediateLong(Self::fetch_long_register(x), cpu.get_next_long())),
            x if x & 0b11001111 == 0xC5 => Some(Push(Self::fetch_long_register(x))),
            x if x & 0b11001111 == 0xC1 => Some(Pop(Self::fetch_long_register(x))),
            _ => None 
        }
    }

    pub const fn size(self) -> u16 {
        match self {
            LoadInstruction::LoadImmediate(_, _) => 2,
            LoadInstruction::LoadRegister(_, _) => 1,
            LoadInstruction::LoadFromHLAddr(_) => 1,
            LoadInstruction::LoadIntoHLAddr(_) => 1,
            LoadInstruction::LoadIntoHLAddrn(_) => 2,
            LoadInstruction::LoadIntoAFromAddr(_) => 1,
            LoadInstruction::LoadIntoAFromAddrnn(_) => 3,
            LoadInstruction::LoadIntoAddrFromA(_) => 1,
            LoadInstruction::LoadIntoAddrnnFromA(_) => 3,
            LoadInstruction::LoadFromAddrCIntoA => 1,
            LoadInstruction::LoadIntoAddrCFromA => 1,
            LoadInstruction::LoadFromAddrHLIntoADec => 1,
            LoadInstruction::LoadFromAIntoAddrHLDec => 1,
            LoadInstruction::LoadFromAddrHLIntoAInc => 1,
            LoadInstruction::LoadFromAIntoAddrHLInc => 1,
            LoadInstruction::LoadFromAIntoAddrn(_) => 2,
            LoadInstruction::LoadFromAddrnIntoA(_) => 2,
            LoadInstruction::LoadImmediateLong(_, _) => 1,
            LoadInstruction::LoadFromHLIntoSP => 1,
            LoadInstruction::LoadFromSPPlusnIntoHL(_) => 2,
            LoadInstruction::LoadSPIntoAddrnn(_) => 3,
            LoadInstruction::Push(_) => 1,
            LoadInstruction::Pop(_) => 1,
        }
    }

    pub const fn cycles(self) -> u8 {
        match self {
            LoadInstruction::LoadImmediate(_, _) => 8,
            LoadInstruction::LoadRegister(_, _) => 4,
            LoadInstruction::LoadFromHLAddr(_) => 8,
            LoadInstruction::LoadIntoHLAddr(_) => 8,
            LoadInstruction::LoadIntoHLAddrn(_) => 12,
            LoadInstruction::LoadIntoAFromAddr(_) => 8,
            LoadInstruction::LoadIntoAFromAddrnn(_) => 16,
            LoadInstruction::LoadIntoAddrFromA(_) => 8,
            LoadInstruction::LoadIntoAddrnnFromA(_) => 16,
            LoadInstruction::LoadFromAddrCIntoA => 8,
            LoadInstruction::LoadIntoAddrCFromA => 8,
            LoadInstruction::LoadFromAddrHLIntoADec => 8,
            LoadInstruction::LoadFromAIntoAddrHLDec => 8,
            LoadInstruction::LoadFromAddrHLIntoAInc => 8,
            LoadInstruction::LoadFromAIntoAddrHLInc => 8,
            LoadInstruction::LoadFromAIntoAddrn(_) => 12,
            LoadInstruction::LoadFromAddrnIntoA(_) => 12,
            LoadInstruction::LoadImmediateLong(_, _) => 12,
            LoadInstruction::LoadFromHLIntoSP => 8,
            LoadInstruction::LoadFromSPPlusnIntoHL(_) => 12,
            LoadInstruction::LoadSPIntoAddrnn(_) => 20,
            LoadInstruction::Push(_) => 16,
            LoadInstruction::Pop(_) => 12,
        }
    }

    pub fn execute(self, cpu: &mut Cpu) {
        match self {
            LoadInstruction::LoadImmediate(r, n) => todo!(),
            LoadInstruction::LoadRegister(r1, r2) => todo!(),
            LoadInstruction::LoadFromHLAddr(r) => todo!(),
            LoadInstruction::LoadIntoHLAddr(r) => todo!(),
            LoadInstruction::LoadIntoHLAddrn(n) => todo!(),
            LoadInstruction::LoadIntoAFromAddr(lr) => todo!(),
            LoadInstruction::LoadIntoAFromAddrnn(nn) => todo!(),
            LoadInstruction::LoadIntoAddrFromA(lr) => todo!(),
            LoadInstruction::LoadIntoAddrnnFromA(nn) => todo!(),
            LoadInstruction::LoadFromAddrCIntoA => todo!(),
            LoadInstruction::LoadIntoAddrCFromA => todo!(),
            LoadInstruction::LoadFromAddrHLIntoADec => todo!(),
            LoadInstruction::LoadFromAIntoAddrHLDec => todo!(),
            LoadInstruction::LoadFromAddrHLIntoAInc => todo!(),
            LoadInstruction::LoadFromAIntoAddrHLInc => todo!(),
            LoadInstruction::LoadFromAIntoAddrn(n) => todo!(),
            LoadInstruction::LoadFromAddrnIntoA(n) => todo!(),
            LoadInstruction::LoadImmediateLong(lr, nn) => todo!(),
            LoadInstruction::LoadFromHLIntoSP => todo!(),
            LoadInstruction::LoadFromSPPlusnIntoHL(n) => todo!(),
            LoadInstruction::LoadSPIntoAddrnn(nn) => todo!(),
            LoadInstruction::Push(lr) => todo!(),
            LoadInstruction::Pop(lr) => todo!(),
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::cpu::{Cpu, registers::{Register, LongRegister, Registers}};

    use super::LoadInstruction;

    // #[test]
    // /// cover 0x06 to 0x3E with step of 8
    // fn test_fetch_load_immediate() {
    //     fn test_opcode(cpu: &Cpu, reg: Register) {
    //         let result = LoadInstruction::fetch(cpu);
    //         let byte = cpu.get_relative(1);
    //         assert_eq!(result,  Some(LoadInstruction::LoadImmediate(reg, byte)));
    //     }
    //     let mut cpu = Cpu::opcode_filled();
    //     cpu.advance_by(0x06);
    //     for i in 0..6 {
    //         test_opcode(&cpu, Registers::REGISTERS[i]);
    //         cpu.advance_by(0x08);
    //     }
    //     assert_eq!(LoadInstruction::fetch(&cpu), Some(LoadInstruction::LoadIntoHLAddrn(0x37)));
    //     cpu.advance_by(0x08);
    //     assert_eq!(LoadInstruction::fetch(&cpu), Some(LoadInstruction::LoadImmediate(Register::A, 0x3F)));
    // }

}
