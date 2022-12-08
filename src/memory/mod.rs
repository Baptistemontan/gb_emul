use std::ops::Range;

use self::memory_section::MemorySection;

pub mod memory_section;

#[derive(Debug, Default)]
pub struct Memory {
    rom: MemorySection<{ Self::ROM_BANK_SIZE }>,
    switchable_rom: MemorySection<{ Self::SWITCHABLE_ROM_BANK_SIZE }>,
    vram: MemorySection<{ Self::VRAM_SIZE }>,
    switchable_ram: MemorySection<{ Self::SWITCHABLE_RAM_BANK_SIZE }>,
    internal_ram: MemorySection<{ Self::INTERNAL_RAM_SIZE }>,
    internal_ram_echo: MemorySection<{ Self::INTERNAL_RAM_ECHO_SIZE }>,
    oam: MemorySection<{ Self::OAM_SIZE }>,
    empty: MemorySection<{ Self::EMPTY_SIZE }>,
    io_ports: MemorySection<{ Self::IO_PORTS_SIZE }>,
    empty_two: MemorySection<{ Self::EMPTY_TWO_SIZE }>,
    internal_ram_two: MemorySection<{ Self::INTERNAL_RAM_TWO_SIZE }>,
    interrupt_enable_register: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Bank {
    Rom,
    SwitchableRom,
    Vram,
    SwitchableRam,
    InternalRam,
    InternalRamEcho,
    Oam,
    Empty,
    IOPorts,
    EmptyTwo,
    InternalRamTwo,
}

impl Bank {
    pub const fn from_addr(addr: u16) -> Option<(Self, u16)> {
        match addr {
            Memory::ROM_BANK_START..=Memory::ROM_BANK_END => {
                Some((Bank::Rom, addr - Memory::ROM_BANK_START))
            }
            Memory::SWITCHABLE_ROM_BANK_START..=Memory::SWITCHABLE_ROM_BANK_END => Some((
                Bank::SwitchableRom,
                addr - Memory::SWITCHABLE_ROM_BANK_START,
            )),
            Memory::VRAM_START..=Memory::VRAM_END => Some((Bank::Vram, addr - Memory::VRAM_START)),
            Memory::SWITCHABLE_RAM_BANK_START..=Memory::SWITCHABLE_RAM_BANK_END => Some((
                Bank::SwitchableRam,
                addr - Memory::SWITCHABLE_RAM_BANK_START,
            )),
            Memory::INTERNAL_RAM_START..=Memory::INTERNAL_RAM_END => {
                Some((Bank::InternalRam, addr - Memory::INTERNAL_RAM_START))
            }
            Memory::INTERNAL_RAM_ECHO_START..=Memory::INTERNAL_RAM_ECHO_END => Some((
                Bank::InternalRamEcho,
                addr - Memory::INTERNAL_RAM_ECHO_START,
            )),
            Memory::OAM_START..=Memory::OAM_END => Some((Bank::Oam, addr - Memory::OAM_START)),
            Memory::EMPTY_START..=Memory::EMPTY_END => {
                Some((Bank::Empty, addr - Memory::EMPTY_START))
            }
            Memory::IO_PORTS_START..=Memory::IO_PORTS_END => {
                Some((Bank::IOPorts, addr - Memory::IO_PORTS_START))
            }
            Memory::EMPTY_TWO_START..=Memory::EMPTY_TWO_END => {
                Some((Bank::EmptyTwo, addr - Memory::EMPTY_TWO_START))
            }
            Memory::INTERNAL_RAM_TWO_START..=Memory::INTERNAL_RAM_TWO_END => {
                Some((Bank::InternalRamTwo, addr - Memory::INTERNAL_RAM_TWO_START))
            },
            0xFFFF => None
        }
    }
}

impl Memory {
    const ROM_BANK_START: u16 = 0x0000;
    const SWITCHABLE_ROM_BANK_START: u16 = 0x4000;
    const VRAM_START: u16 = 0x8000;
    const SWITCHABLE_RAM_BANK_START: u16 = 0xA000;
    const INTERNAL_RAM_START: u16 = 0xC000;
    const INTERNAL_RAM_ECHO_START: u16 = 0xE000;
    const OAM_START: u16 = 0xFE00;
    const EMPTY_START: u16 = 0xFEA0;
    const IO_PORTS_START: u16 = 0xFF00;
    const EMPTY_TWO_START: u16 = 0xFF4C;
    const INTERNAL_RAM_TWO_START: u16 = 0xFF80;
    const INTERRUPT_ENABLE_REGISTER_START: u16 = 0xFFFF;

    const ROM_BANK_SIZE: usize = (Self::SWITCHABLE_ROM_BANK_START - Self::ROM_BANK_START) as usize;
    const SWITCHABLE_ROM_BANK_SIZE: usize = (Self::VRAM_START - Self::SWITCHABLE_ROM_BANK_START) as usize;
    const VRAM_SIZE: usize = (Self::SWITCHABLE_RAM_BANK_START - Self::VRAM_START) as usize;
    const SWITCHABLE_RAM_BANK_SIZE: usize =
        (Self::INTERNAL_RAM_START - Self::SWITCHABLE_RAM_BANK_START) as usize;
    const INTERNAL_RAM_SIZE: usize = (Self::INTERNAL_RAM_ECHO_START - Self::INTERNAL_RAM_START) as usize;
    const INTERNAL_RAM_ECHO_SIZE_U16: u16 = Self::OAM_START - Self::INTERNAL_RAM_ECHO_START;
    const INTERNAL_RAM_ECHO_SIZE: usize = Self::INTERNAL_RAM_ECHO_SIZE_U16 as usize;
    const OAM_SIZE: usize = (Self::EMPTY_START - Self::OAM_START) as usize;
    const EMPTY_SIZE: usize = (Self::IO_PORTS_START - Self::EMPTY_START) as usize;
    const IO_PORTS_SIZE: usize = (Self::EMPTY_TWO_START - Self::IO_PORTS_START) as usize;
    const EMPTY_TWO_SIZE: usize = (Self::INTERNAL_RAM_TWO_START - Self::EMPTY_TWO_START) as usize;
    const INTERNAL_RAM_TWO_SIZE: usize =
        (Self::INTERRUPT_ENABLE_REGISTER_START - Self::INTERNAL_RAM_TWO_START) as usize;

    const ROM_BANK_END: u16 = Self::SWITCHABLE_ROM_BANK_START - 1;
    const SWITCHABLE_ROM_BANK_END: u16 = Self::VRAM_START - 1;
    const VRAM_END: u16 = Self::SWITCHABLE_RAM_BANK_START - 1;
    const SWITCHABLE_RAM_BANK_END: u16 = Self::INTERNAL_RAM_START - 1;
    const INTERNAL_RAM_END: u16 = Self::INTERNAL_RAM_ECHO_START - 1;
    const INTERNAL_RAM_ECHO_END: u16 = Self::OAM_START - 1;
    const OAM_END: u16 = Self::EMPTY_START - 1;
    const EMPTY_END: u16 = Self::IO_PORTS_START - 1;
    const IO_PORTS_END: u16 = Self::EMPTY_TWO_START - 1;
    const EMPTY_TWO_END: u16 = Self::INTERNAL_RAM_TWO_START - 1;
    const INTERNAL_RAM_TWO_END: u16 = Self::INTERRUPT_ENABLE_REGISTER_START - 1;

    pub fn get(&self, addr: u16) -> u8 {
        if let Some((bank, addr)) = Bank::from_addr(addr) {
            match bank {
                Bank::Rom => self.rom.get(addr),
                Bank::SwitchableRom => self.switchable_rom.get(addr),
                Bank::Vram => self.vram.get(addr),
                Bank::SwitchableRam => self.switchable_ram.get(addr),
                Bank::InternalRam => self.internal_ram.get(addr),
                Bank::InternalRamEcho => self.internal_ram_echo.get(addr),
                Bank::Oam => self.oam.get(addr),
                Bank::Empty => self.empty.get(addr),
                Bank::IOPorts => self.io_ports.get(addr),
                Bank::EmptyTwo => self.empty_two.get(addr),
                Bank::InternalRamTwo => self.internal_ram_two.get(addr),
            }
        } else {
            self.interrupt_enable_register
        }
    }

    pub fn put(&mut self, addr: u16, value: u8) {
        if let Some((bank, addr)) = Bank::from_addr(addr) {
            match bank {
                Bank::Rom => self.rom.set(addr, value),
                Bank::SwitchableRom => self.switchable_rom.set(addr, value),
                Bank::Vram => self.vram.set(addr, value),
                Bank::SwitchableRam => self.switchable_ram.set(addr, value),
                Bank::InternalRam => self.internal_ram.set(addr, value),
                Bank::InternalRamEcho => self.internal_ram_echo.set(addr, value),
                Bank::Oam => self.oam.set(addr, value),
                Bank::Empty => self.empty.set(addr, value),
                Bank::IOPorts => self.io_ports.set(addr, value),
                Bank::EmptyTwo => self.empty_two.set(addr, value),
                Bank::InternalRamTwo => self.internal_ram_two.set(addr, value),
            }
        } else {
            self.interrupt_enable_register = value;
        }
    }
}
