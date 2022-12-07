use crate::help_traits::AccesBigEndianBytesU16;

#[derive(Debug, Default)]
pub struct Registers {
    af: u16,
    bc: u16,
    de: u16,
    hl: u16,
    sp: u16,
    pc: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Register {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LongRegister {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
}

/// Flags are contained in the F register
///
/// |7|6|5|4|3|2|1|0|
/// |-|-|-|-|-|-|-|-|
/// |Z|N|H|C|0|0|0|0|
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Flags {
    /// Zero Flag (Z)
    ///
    /// This bit is set when the result of a math operation
    ///  is zero or two values match when using the CP instruction.
    Zero,
    /// Substract Flag (N)
    ///
    /// This bit is set if a subtraction was performed
    /// in the last math instruction.
    Substract,
    /// Half Carrt Flag (H)
    ///
    /// This bit is set if a carry occurred from the lower
    /// nibble in the last math operation
    HalfCarry,
    /// Carry Flag (C)
    ///
    /// This bit is set if a carry occurred from the last
    /// math operation or if register A is the smaller value
    /// when executing the CP instruction.
    Carry,
}

pub struct SetFlags {
    pub zero: bool,
    pub substract: bool,
    pub half_carry: bool,
    pub carry: bool,
}

impl Flags {
    const ZERO_MASK: u8 = 0x80;
    const SUBSTRACT_MASK: u8 = 0x40;
    const HALF_CARRY_MASK: u8 = 0x20;
    const CARRY_MASK: u8 = 0x10;
}

impl Into<u8> for Flags {
    fn into(self) -> u8 {
        match self {
            Flags::Zero => Flags::ZERO_MASK,
            Flags::Substract => Flags::SUBSTRACT_MASK,
            Flags::HalfCarry => Flags::HALF_CARRY_MASK,
            Flags::Carry => Flags::CARRY_MASK,
        }
    }
}

impl From<u8> for SetFlags {
    fn from(flags: u8) -> Self {
        let zero = flags & Flags::ZERO_MASK == Flags::ZERO_MASK;
        let substract = flags & Flags::SUBSTRACT_MASK == Flags::SUBSTRACT_MASK;
        let half_carry = flags & Flags::HALF_CARRY_MASK == Flags::HALF_CARRY_MASK;
        let carry = flags & Flags::CARRY_MASK == Flags::CARRY_MASK;
        SetFlags {
            zero,
            substract,
            half_carry,
            carry,
        }
    }
}

impl Into<u8> for SetFlags {
    fn into(self) -> u8 {
        let mut flags = 0;
        if self.zero {
            flags |= Flags::ZERO_MASK;
        }
        if self.substract {
            flags |= Flags::SUBSTRACT_MASK;
        }
        if self.half_carry {
            flags |= Flags::HALF_CARRY_MASK;
        }
        if self.carry {
            flags |= Flags::CARRY_MASK;
        }
        flags
    }
}

impl Registers {

    pub const REGISTERS: [Register; 8] = [Register::B, Register::C, Register::D, Register::E, Register::H, Register::L, Register::F, Register::A];
    pub const LONG_REGISTERS: [LongRegister; 4] = [LongRegister::BC, LongRegister::DE, LongRegister::HL, LongRegister::SP];

    pub fn get_mut(&mut self, reg: Register) -> &mut u8 {
        match reg {
            Register::A => self.af.get_high_mut(),
            Register::B => self.bc.get_high_mut(),
            Register::C => self.bc.get_low_mut(),
            Register::D => self.de.get_high_mut(),
            Register::E => self.de.get_low_mut(),
            Register::F => self.af.get_low_mut(),
            Register::H => self.hl.get_high_mut(),
            Register::L => self.hl.get_low_mut(),
        }
    }

    pub fn get(&self, reg: Register) -> u8 {
        match reg {
            Register::A => self.af.get_high(),
            Register::B => self.bc.get_high(),
            Register::C => self.bc.get_low(),
            Register::D => self.de.get_high(),
            Register::E => self.de.get_low(),
            Register::F => self.af.get_low(),
            Register::H => self.hl.get_high(),
            Register::L => self.hl.get_low(),
        }
    }

    pub fn set(&mut self, reg: Register, byte: u8) {
        let register = self.get_mut(reg);
        *register = byte;
    }

    pub fn get_long_mut(&mut self, reg: LongRegister) -> &mut u16 {
        match reg {
            LongRegister::AF => &mut self.af,
            LongRegister::BC => &mut self.bc,
            LongRegister::DE => &mut self.de,
            LongRegister::HL => &mut self.hl,
            LongRegister::SP => &mut self.sp,
            LongRegister::PC => &mut self.pc,
        }
    }

    pub fn get_long(&self, reg: LongRegister) -> u16 {
        match reg {
            LongRegister::AF => self.af,
            LongRegister::BC => self.bc,
            LongRegister::DE => self.de,
            LongRegister::HL => self.hl,
            LongRegister::SP => self.sp,
            LongRegister::PC => self.pc,
        }
    }

    pub fn set_long(&mut self, reg: LongRegister, value: u16) {
        let register = self.get_long_mut(reg);
        *register = value;
    }

    pub fn get_flags(&self) -> SetFlags {
        let flags = self.get(Register::F);
        flags.into()
    }

    pub fn get_flag(&self, flag: Flags) -> bool {
        match flag {
            Flags::Zero => self.get_flags().zero,
            Flags::Substract => self.get_flags().substract,
            Flags::HalfCarry => self.get_flags().half_carry,
            Flags::Carry => self.get_flags().carry,
        }
    }

    pub fn set_flag(&mut self, flag: Flags) {
        let flag_reg = self.get_mut(Register::F);
        let mask: u8 = flag.into();
        *flag_reg |= mask;
    }

    pub fn reset_flag(&mut self, flag: Flags) {
        let flag_reg = self.get_mut(Register::F);
        let mask: u8 = flag.into();
        *flag_reg &= !mask;
    }

    pub fn set_flag_to(&mut self, flag: Flags, value: bool) {
        if value {
            self.set_flag(flag);
        } else {
            self.reset_flag(flag);
        }
    }
}
