use crate::cpu::{
    registers::{LongRegister, Register, Registers, SetFlags},
    Cpu,
};

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
    LoadFromSPPlusnIntoHL(i8),
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
            (r1, r2) => Some(LoadInstruction::LoadRegister(r1, r2)),
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
    
    fn add_delta_to_addr(addr: u16, delta: i8) -> (u16, SetFlags) {
        let neg = delta.is_negative();
        let [delta_byte] = i8::to_be_bytes(delta);
        let delta_byte: u16 = delta_byte.into();
        let [delta] = i8::to_be_bytes(delta.abs());
        let delta: u16 = delta.into();
        let result = if neg {
            addr - delta
        } else {
            addr + delta
        };

        let carry = (addr ^ delta_byte ^ result) & 0x0100 == 0x0100;
        let half_carry = (addr ^ delta_byte ^ result) & 0x0010 == 0x0010;

        let flags = SetFlags {
            carry,
            half_carry,
            ..Default::default()
        };

        (result, flags)
    }

    pub fn fetch(cpu: &mut Cpu, opcode: u8) -> Option<Self> {
        use LoadInstruction::*;

        match opcode {
            0x08 => Some(LoadSPIntoAddrnn(cpu.advance_long())),
            0x22 => Some(LoadFromAIntoAddrHLInc),
            0x2A => Some(LoadFromAddrHLIntoAInc),
            0x32 => Some(LoadFromAIntoAddrHLDec),
            0x3A => Some(LoadFromAddrHLIntoADec),
            0x40..=0x7F => Self::fetch_load_r1_r2(opcode),
            0xE0 => Some(LoadFromAIntoAddrn(cpu.advance())),
            0xE2 => Some(LoadIntoAddrCFromA),
            0xEA => Some(LoadIntoAddrnnFromA(cpu.advance_long())),
            0xF0 => Some(LoadFromAddrnIntoA(cpu.advance())),
            0xF2 => Some(LoadFromAddrCIntoA),
            0xF9 => Some(LoadFromHLIntoSP),
            0xF8 => {
                let byte = cpu.advance();
                Some(LoadFromSPPlusnIntoHL(i8::from_be_bytes([byte])))
            },
            0xFA => Some(LoadIntoAFromAddrnn(cpu.advance_long())),
            x if x & 0b11001111 == 0x0A => Some(LoadIntoAFromAddr(Self::fetch_long_register(x))),
            x if x & 0b11001111 == 0x02 => Some(LoadIntoAddrFromA(Self::fetch_long_register(x))),
            x if x & 0b11000111 == 0x06 => Some(Self::fetch_load_immediate(x, cpu.advance())),
            x if x & 0b11001111 == 0x01 => Some(LoadImmediateLong(
                Self::fetch_long_register(x),
                cpu.advance_long(),
            )),
            x if x & 0b11001111 == 0xC5 => Some(Push(Self::fetch_long_register(x))),
            x if x & 0b11001111 == 0xC1 => Some(Pop(Self::fetch_long_register(x))),
            _ => None,
        }
    }

    pub fn execute(self, cpu: &mut Cpu) {
        match self {
            LoadInstruction::LoadImmediate(reg, n) => {
                cpu.put_reg(reg, n);
            },
            LoadInstruction::LoadRegister(r1, r2) => {
                cpu.put_reg(r1, cpu.get_reg(r2));
            },
            LoadInstruction::LoadFromHLAddr(reg) => {
                let value = cpu.get_at_hl();
                cpu.put_reg(reg, value);
            },
            LoadInstruction::LoadIntoHLAddr(reg) => {
                let value = cpu.get_reg(reg);
                cpu.put_at_hl(value);
            },
            LoadInstruction::LoadIntoHLAddrn(n) => {
                cpu.put_at_hl(n);
            },
            LoadInstruction::LoadIntoAFromAddr(lr) => {
                let addr = cpu.get_long_reg(lr);
                let value = cpu.get_memory(addr);
                cpu.put_reg(Register::A, value);
            },
            LoadInstruction::LoadIntoAFromAddrnn(addr) => {
                let value = cpu.get_memory(addr);
                cpu.put_reg(Register::A, value);
            },
            LoadInstruction::LoadIntoAddrFromA(lr) => {
                let addr = cpu.get_long_reg(lr);
                let value = cpu.get_reg_a();
                cpu.put_memory(addr, value);
            },
            LoadInstruction::LoadIntoAddrnnFromA(addr) => {
                let value = cpu.get_reg_a();
                cpu.put_memory(addr, value);
            },
            LoadInstruction::LoadFromAddrCIntoA => {
                let c_reg = cpu.get_reg(Register::C);
                let addr = u16::from_be_bytes([0xFF, c_reg]);
                let value = cpu.get_memory(addr);
                cpu.put_reg_a(value);
            },
            LoadInstruction::LoadIntoAddrCFromA => {
                let c_reg = cpu.get_reg(Register::C);
                let addr = u16::from_be_bytes([0xFF, c_reg]);
                let value = cpu.get_reg_a();
                cpu.put_memory(addr, value);
            },
            LoadInstruction::LoadFromAddrHLIntoADec => {
                let addr = cpu.get_long_reg(LongRegister::HL);
                let value = cpu.get_memory(addr);
                cpu.put_reg_a(value);
                cpu.put_long_reg(LongRegister::HL, addr - 1);
            },
            LoadInstruction::LoadFromAIntoAddrHLDec => {
                let addr = cpu.get_long_reg(LongRegister::HL);
                let value = cpu.get_reg_a();
                cpu.put_memory(addr, value);
                cpu.put_long_reg(LongRegister::HL, addr - 1);
            },
            LoadInstruction::LoadFromAddrHLIntoAInc => {
                let addr = cpu.get_long_reg(LongRegister::HL);
                let value = cpu.get_memory(addr);
                cpu.put_reg_a(value);
                cpu.put_long_reg(LongRegister::HL, addr + 1);
            },
            LoadInstruction::LoadFromAIntoAddrHLInc => {
                let addr = cpu.get_long_reg(LongRegister::HL);
                let value = cpu.get_reg_a();
                cpu.put_memory(addr, value);
                cpu.put_long_reg(LongRegister::HL, addr + 1);
            },
            LoadInstruction::LoadFromAIntoAddrn(n) => {
                let addr = u16::from_be_bytes([0xFF, n]);
                let value = cpu.get_reg_a();
                cpu.put_memory(addr, value);
            },
            LoadInstruction::LoadFromAddrnIntoA(n) => {
                let addr = u16::from_be_bytes([0xFF, n]);
                let value = cpu.get_memory(addr);
                cpu.put_reg_a(value);
            },
            LoadInstruction::LoadImmediateLong(reg, value) => {
                cpu.put_long_reg(reg, value);
            },
            LoadInstruction::LoadFromHLIntoSP => {
                // 2 machine cycle but only one W/R, so need to explicitly cycle
                cpu.cycle();
                let value = cpu.get_long_reg(LongRegister::HL);
                cpu.put_long_reg(LongRegister::SP, value);
            }
            LoadInstruction::LoadFromSPPlusnIntoHL(delta) => {
                let sp = cpu.get_long_reg(LongRegister::SP);
                let (value, flags) = Self::add_delta_to_addr(sp, delta);
                cpu.set_flags(flags);
                cpu.put_long_reg(LongRegister::HL, value);
            },
            LoadInstruction::LoadSPIntoAddrnn(addr) => {
                let value = cpu.get_long_reg(LongRegister::SP);
                cpu.put_long_at(addr, value);
            },
            LoadInstruction::Push(long_reg) => {
                // 4 machine cycle but only 3 W/R, so need to explicitly cycle
                cpu.cycle();
                let value = cpu.get_long_reg(long_reg);
                cpu.push_stack(value);
            },
            LoadInstruction::Pop(long_reg) => {
                let value = cpu.pop_stack();
                cpu.put_long_reg(long_reg, value);
            },
        }
    }
}
