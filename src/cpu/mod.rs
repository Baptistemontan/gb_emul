use crate::memory::Memory;

use self::{registers::{Flags, LongRegister, Register, Registers, SetFlags}, cyclic::Cyclic};

pub mod registers;
pub mod cyclic;

#[derive(Debug, Default)]
pub struct Cpu {
    registers: Registers,
    memory: Memory,
    cyclic: Cyclic,
}

impl Cpu {

    /// Cycles: 4
    pub fn current_byte(&mut self) -> u8 {
        let addr = self.get_pc();
        self.get_memory(addr)
    }

    /// Cycles: 4
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

    pub fn put_reg(&mut self, reg: Register, value: u8) {
        *self.get_reg_mut(reg) = value;
    }

    pub fn get_long_reg(&self, reg: LongRegister) -> u16 {
        self.registers.get_long(reg)
    }

    pub fn get_long_reg_mut(&mut self, reg: LongRegister) -> &mut u16 {
        self.registers.get_long_mut(reg)
    }

    pub fn put_long_reg(&mut self, reg: LongRegister, value: u16) {
        *self.get_long_reg_mut(reg) = value;
    }

    /// Cycles: 4
    pub fn get_at_hl(&mut self) -> u8 {
        let hl = self.get_long_reg(LongRegister::HL);
        self.get_memory(hl)
    }

    /// Cycles: 4
    pub fn put_at_hl(&mut self, value: u8) {
        let hl = self.get_long_reg(LongRegister::HL);
        self.put_memory(hl, value);
    }

    pub fn get_reg_a(&self) -> u8 {
        self.get_reg(Register::A)
    }
    
    pub fn put_reg_a(&mut self, value: u8) {
        self.put_reg(Register::A, value);
    }

    /// Cycles: 4
    pub fn get_relative(&mut self, delta: u16) -> u8 {
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
        let pc = self.get_pc_mut();
        *pc = pc.wrapping_add(delta);
    }

    pub fn move_by(&mut self, delta: i16) {
        // well technically addition of signed and unsigned is the same because of the way 2's complement works,
        // so addition of a signed delta is juste add the bits together and let it wrapp overflow, discarding the carry.
        // (when mixed_integrer_ops stable ?)
        let bytes = i16::to_ne_bytes(delta);
        let delta = u16::from_ne_bytes(bytes);
        self.advance_by(delta);
    }

    pub fn get_pc_mut(&mut self) -> &mut u16 {
        self.registers.get_long_mut(LongRegister::PC)
    }

    pub fn set_pc(&mut self, value: u16) {
        *self.get_pc_mut() = value;
    }

    pub fn get_pc(&self) -> u16 {
        self.registers.get_long(LongRegister::PC)
    }

    /// Cycles: 4
    pub fn get_memory(&mut self, addr: u16) -> u8 {
        // memory read is 1 cycle
        self.cycle();
        self.memory.get(addr)
    }

    /// Cycles: 4
    pub fn put_memory(&mut self, addr: u16, value: u8) {
        // memory write is 1 cycle
        self.cycle();
        self.memory.put(addr, value);
    }

    pub fn get_flags(&self) -> SetFlags {
        self.registers.get_flags()
    }

    pub fn set_flags(&mut self, flags: SetFlags) {
        self.registers.set_flags(flags);
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

    /// Cycles: 8
    pub fn get_long_at(&mut self, addr: u16) -> u16 {
        let lsb = self.get_memory(addr);
        let msb = self.get_memory(addr + 1);
        u16::from_be_bytes([msb, lsb])
    }

    /// Cycles: 8
    pub fn put_long_at(&mut self, addr: u16, value: u16) {
        let [msb, lsb] = u16::to_be_bytes(value);
        self.put_memory(addr, lsb);
        self.put_memory(addr + 1, msb);
    }

    /// Cycles: 8
    pub fn get_next_long(&mut self) -> u16 {
        self.get_long_at(self.get_pc() + 1)
    }

    /// Cycles: 8
    pub fn advance_long(&mut self) -> u16 {
        let msb = self.advance();
        let lsb = self.advance();
        u16::from_be_bytes([msb, lsb])
    }

    /// Cycles: 8
    pub fn push_stack(&mut self, value: u16) {
        let sp = self.get_long_reg(LongRegister::SP);
        let addr = sp - 2;
        self.put_long_at(addr, value);
        self.put_long_reg(LongRegister::SP, addr);
    }

    /// Cycles: 8
    pub fn pop_stack(&mut self) -> u16 {
        let sp = self.get_long_reg(LongRegister::SP);
        let value = self.get_long_at(sp);
        self.put_long_reg(LongRegister::SP, sp + 2);
        value
    }

    /// Cycle: 4
    pub fn cycle(&mut self) {
        self.cyclic.cycle()
    }

    pub fn enable_interrupts(&mut self) {
        todo!()
    }

    pub fn disable_interrupts(&mut self) {
        todo!()
    }
}
