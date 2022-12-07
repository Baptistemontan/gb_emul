use crate::cpu::{Cpu, registers::{LongRegister, Register, Registers}};

use self::{
    arithmetic::ArithmeticInstruction, bit::BitInstruction, control_flow::ControlFlowInstruction,
    load::LoadInstruction, miscellaneous::MiscInstruction, rotate_shift::RotateShiftInstruction,
};

pub mod arithmetic;
pub mod bit;
pub mod control_flow;
pub mod load;
pub mod miscellaneous;
pub mod rotate_shift;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Load(LoadInstruction),
    Arithmetic(ArithmeticInstruction),
    Misc(MiscInstruction),
    RotateShift(RotateShiftInstruction),
    Bit(BitInstruction),
    ControlFlow(ControlFlowInstruction),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FetchRegister {
    Register(Register),
    AddrHL
}

impl From<u8> for FetchRegister {
    fn from(value: u8) -> Self {
        let reg = Registers::REGISTERS[value as usize];
        if reg == Register::F {
            FetchRegister::AddrHL
        } else {
            FetchRegister::Register(reg)
        }
    }
}

#[macro_export]
/// Made to do the same thing as FetchRegister::map,
/// but usable in a const context 
/// When const trait impl stabilized ? :(
macro_rules! map_fetch_register {
    ($reg:ident, $if_reg:path, $if_hl:ident) => {
        {
            match $reg {
                FetchRegister::Register(reg) => $if_reg(reg),
                FetchRegister::AddrHL => $if_hl,
            }
        }
    };
}

impl FetchRegister {
    pub fn map<F, T>(self, if_reg: F, if_hl: T) -> T 
        where F: FnOnce(Register) -> T,
    {
        match self {
            FetchRegister::Register(reg) => if_reg(reg),
            FetchRegister::AddrHL => if_hl,
        }
    }
}

impl Instruction {
    pub fn fetch(cpu: &Cpu) -> Option<Self> {
        let opcode = cpu.current_byte();
        if opcode == 0xCB {
            let opcode = cpu.get_relative(1);
            let reg = (opcode & 0b00000111).into();
            let opcode_id = opcode & 0b11111000;
            MiscInstruction::fetch_prefixed(cpu, opcode_id, reg).map(Instruction::Misc)
                .or_else(|| RotateShiftInstruction::fetch_prefixed(cpu, opcode_id, reg).map(Instruction::RotateShift))
                .or_else(|| BitInstruction::fetch_prefixed(cpu, opcode_id, reg).map(Instruction::Bit))
        } else {
            LoadInstruction::fetch(cpu, opcode).map(Instruction::Load)
                .or_else(|| ArithmeticInstruction::fetch(cpu, opcode).map(Instruction::Arithmetic))
                .or_else(|| MiscInstruction::fetch(cpu, opcode).map(Instruction::Misc))
                .or_else(|| RotateShiftInstruction::fetch(cpu, opcode).map(Instruction::RotateShift))
                .or_else(|| ControlFlowInstruction::fetch(cpu, opcode).map(Instruction::ControlFlow))
        }
    }

    pub const fn size(self) -> u16 {
        match self {
            Instruction::Load(instruction) => instruction.size(),
            Instruction::Arithmetic(instruction) => instruction.size(),
            Instruction::Misc(instruction) => instruction.size(),
            Instruction::RotateShift(instruction) => instruction.size(),
            Instruction::Bit(instruction) => instruction.size(),
            Instruction::ControlFlow(instruction) => instruction.size(),
        }
    }

    pub const fn cycles(self) -> u8 {
        match self {
            Instruction::Load(instruction) => instruction.cycles(),
            Instruction::Arithmetic(instruction) => instruction.cycles(),
            Instruction::Misc(instruction) => instruction.cycles(),
            Instruction::RotateShift(instruction) => instruction.cycles(),
            Instruction::Bit(instruction) => instruction.cycles(),
            Instruction::ControlFlow(instruction) => instruction.cycles(),
        }
    }

    pub fn execute(self, cpu: &mut Cpu) {
        match self {
            Instruction::Load(instruction) => instruction.execute(cpu),
            Instruction::Arithmetic(instruction) => instruction.execute(cpu),
            Instruction::Misc(instruction) => instruction.execute(cpu),
            Instruction::RotateShift(instruction) => instruction.execute(cpu),
            Instruction::Bit(instruction) => instruction.execute(cpu),
            Instruction::ControlFlow(instruction) => instruction.execute(cpu),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::Cpu;

    use super::Instruction;

    #[test]
    fn display_instruction() {
        let mut cpu = Cpu::opcode_filled();
        for i in 0..=0xFF {
            print!("{:#X} : ", cpu.current_byte());
            let instruction = Instruction::fetch(&cpu);
            cpu.advance_by(1);
            if i == 0xCB {
                println!("PREFIX CB");
                continue;
            }
            if let Some(inst) = instruction {
                println!("{:?} | Cycles: {} | Size: {}", inst, inst.cycles(), inst.size());
            } else {
                println!("Unknown");
            }
        }
        
        for _ in 0..=0xFF {
            print!("CB {:#X} : ", cpu.get_relative(1));
            let instruction = Instruction::fetch(&cpu);
            cpu.advance_by(2);
            if let Some(inst) = instruction {
                println!("{:?} | Cycles: {} | Size: {}", inst, inst.cycles(), inst.size());
            } else {
                println!("Unknown");
            }
        }

        print!("0x10 0x00 : ");
        if let Some(inst) = Instruction::fetch(&cpu) {
            println!("{:?} | Cycles: {} | Size: {}", inst, inst.cycles(), inst.size());
        } else {
            println!("Unknown");
        }
    }
}