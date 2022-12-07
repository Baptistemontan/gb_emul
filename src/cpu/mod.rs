use crate::memory::Memory;

use self::registers::{Registers, LongRegister, Register, SetFlags, Flags};

pub mod registers;

#[derive(Debug, Default)]
pub struct Cpu {
    registers: Registers,
    memory: Memory,
}

impl Cpu {
    pub fn current_byte(&self) -> u8 {
        let addr = self.get_pc();
        self.get_memory(addr)
    }

    pub fn advance(&mut self) -> u8 {
        let byte = self.current_byte();
        self.advance_by(1);
        byte
    }

    pub fn get_reg(&self, reg: Register) -> u8 {
        self.registers.get(reg)
    }

    pub fn get_reg_mut(&mut self, reg: Register) -> &mut u8 {
        self.registers.get_mut(reg)
    }

    pub fn get_long_reg(&self, reg: LongRegister) -> u16 {
        self.registers.get_long(reg)
    }

    pub fn get_long_reg_mut(&mut self, reg: LongRegister) -> &mut u16 {
        self.registers.get_long_mut(reg)
    }

    pub fn get_relative(&self, delta: u16) -> u8 {
        let cp = self.get_pc();
        self.get_memory(cp + delta)
    }

    #[cfg(test)]
    pub fn opcode_filled() -> Self {
        let mut cpu = Cpu::default();
        for i in 0..=0xFF {
            let addr: u16 = i.into();
            cpu.memory.put(i.into(), i);
            let prefixed_addr = addr * 2 + 0x0100;
            cpu.memory.put(prefixed_addr, 0xCB);
            cpu.memory.put(prefixed_addr + 1, i);
        }
        let stop_addr = 0x0300;
        cpu.memory.put(stop_addr, 0x10);
        cpu.memory.put(stop_addr + 1, 0x00);
        cpu
    }

    pub fn advance_by(&mut self, delta: u16) {
        *self.get_pc_mut() += delta;
    }

    pub fn get_pc_mut(&mut self) -> &mut u16 {
        self.registers.get_long_mut(LongRegister::PC)
    }

    pub fn get_pc(&self) -> u16 {
        self.registers.get_long(LongRegister::PC)
    }

    pub fn get_memory(&self, addr: u16) -> u8 {
        self.memory.get(addr)
    }

    pub fn put_memory(&mut self, addr: u16, value: u8) {
        self.memory.put(addr, value);
    }

    pub fn get_flags(&self) -> SetFlags {
        self.registers.get_flags()
    }

    pub fn get_flag(&self, flag: Flags) -> bool {
        self.registers.get_flag(flag)
    }

    pub fn set_flag(&mut self, flag: Flags) {
        self.registers.set_flag(flag);
    }

    pub fn reset_flag(&mut self, flag: Flags) {
        self.registers.reset_flag(flag);
    }

    pub fn set_flag_to(&mut self, flag: Flags, value: bool) {
        self.registers.set_flag_to(flag, value);
    }

    pub fn get_next_long(&self) -> u16 {
        let msb = self.get_relative(1);
        let lsb = self.get_relative(2);
        u16::from_be_bytes([msb, lsb])
    }
}

