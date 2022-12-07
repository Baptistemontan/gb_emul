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
    pub fn from_addr(addr: u16) -> Option<(Self, usize)> {
        let addr: usize = addr.into();
        let bank = if Memory::ROM_BANK_RANGE.contains(&addr) {
            (Bank::Rom, addr - Memory::ROM_BANK_START)
        } else if Memory::SWITCHABLE_ROM_BANK_RANGE.contains(&addr) {
            (
                Bank::SwitchableRom,
                addr - Memory::SWITCHABLE_ROM_BANK_START,
            )
        } else if Memory::VRAM_RANGE.contains(&addr) {
            (Bank::Vram, addr - Memory::VRAM_START)
        } else if Memory::SWITCHABLE_RAM_BANK_RANGE.contains(&addr) {
            (
                Bank::SwitchableRam,
                addr - Memory::SWITCHABLE_RAM_BANK_START,
            )
        } else if Memory::INTERNAL_RAM_RANGE.contains(&addr) {
            let addr = addr - Memory::INTERNAL_RAM_START;
            if addr < Memory::INTERNAL_RAM_ECHO_SIZE {
                (Bank::InternalRamEcho, addr)
            } else {
                (Bank::InternalRam, addr)
            }
        } else if Memory::INTERNAL_RAM_ECHO_RANGE.contains(&addr) {
            (Bank::InternalRamEcho, addr - Memory::INTERNAL_RAM_ECHO_START)
        } else if Memory::OAM_RANGE.contains(&addr) {
            (Bank::Oam, addr - Memory::OAM_START)
        } else if Memory::EMPTY_RANGE.contains(&addr) {
            (Bank::Empty, addr - Memory::EMPTY_START)
        } else if Memory::IO_PORTS_RANGE.contains(&addr) {
            (Bank::IOPorts, addr - Memory::IO_PORTS_START)
        } else if Memory::EMPTY_TWO_RANGE.contains(&addr) {
            (Bank::EmptyTwo, addr - Memory::EMPTY_TWO_START)
        } else if Memory::INTERNAL_RAM_TWO_RANGE.contains(&addr) {
            (Bank::InternalRamTwo, addr - Memory::INTERNAL_RAM_TWO_START)
        } else {
            return None;
        };
        Some(bank)
    }
}

impl Memory {
    const ROM_BANK_START: usize = 0x0000;
    const SWITCHABLE_ROM_BANK_START: usize = 0x4000;
    const VRAM_START: usize = 0x8000;
    const SWITCHABLE_RAM_BANK_START: usize = 0xA000;
    const INTERNAL_RAM_START: usize = 0xC000;
    const INTERNAL_RAM_ECHO_START: usize = 0xE000;
    const OAM_START: usize = 0xFE00;
    const EMPTY_START: usize = 0xFEA0;
    const IO_PORTS_START: usize = 0xFF00;
    const EMPTY_TWO_START: usize = 0xFF4C;
    const INTERNAL_RAM_TWO_START: usize = 0xFF80;
    const INTERRUPT_ENABLE_REGISTER_START: usize = 0xFFFF;

    const ROM_BANK_SIZE: usize = Self::SWITCHABLE_ROM_BANK_START - Self::ROM_BANK_START;
    const SWITCHABLE_ROM_BANK_SIZE: usize = Self::VRAM_START - Self::SWITCHABLE_ROM_BANK_START;
    const VRAM_SIZE: usize = Self::SWITCHABLE_RAM_BANK_START - Self::VRAM_START;
    const SWITCHABLE_RAM_BANK_SIZE: usize =
        Self::INTERNAL_RAM_START - Self::SWITCHABLE_RAM_BANK_START;
    const INTERNAL_RAM_SIZE: usize = Self::INTERNAL_RAM_ECHO_START - Self::INTERNAL_RAM_START;
    const INTERNAL_RAM_ECHO_SIZE: usize = Self::OAM_START - Self::INTERNAL_RAM_ECHO_START;
    const OAM_SIZE: usize = Self::EMPTY_START - Self::OAM_START;
    const EMPTY_SIZE: usize = Self::IO_PORTS_START - Self::EMPTY_START;
    const IO_PORTS_SIZE: usize = Self::EMPTY_TWO_START - Self::IO_PORTS_START;
    const EMPTY_TWO_SIZE: usize = Self::INTERNAL_RAM_TWO_START - Self::EMPTY_TWO_START;
    const INTERNAL_RAM_TWO_SIZE: usize =
        Self::INTERRUPT_ENABLE_REGISTER_START - Self::INTERNAL_RAM_TWO_START;

    const ROM_BANK_RANGE: Range<usize> = Self::ROM_BANK_START..Self::SWITCHABLE_ROM_BANK_START;
    const SWITCHABLE_ROM_BANK_RANGE: Range<usize> =
        Self::SWITCHABLE_ROM_BANK_START..Self::VRAM_START;
    const VRAM_RANGE: Range<usize> = Self::VRAM_START..Self::SWITCHABLE_ROM_BANK_START;
    const SWITCHABLE_RAM_BANK_RANGE: Range<usize> =
        Self::SWITCHABLE_RAM_BANK_START..Self::INTERNAL_RAM_START;
    const INTERNAL_RAM_RANGE: Range<usize> =
        Self::INTERNAL_RAM_START..Self::INTERNAL_RAM_ECHO_START;
    const INTERNAL_RAM_ECHO_RANGE: Range<usize> = Self::INTERNAL_RAM_ECHO_START..Self::OAM_START;
    const OAM_RANGE: Range<usize> = Self::OAM_START..Self::EMPTY_START;
    const EMPTY_RANGE: Range<usize> = Self::EMPTY_START..Self::IO_PORTS_START;
    const IO_PORTS_RANGE: Range<usize> = Self::IO_PORTS_START..Self::EMPTY_TWO_START;
    const EMPTY_TWO_RANGE: Range<usize> = Self::EMPTY_TWO_START..Self::INTERNAL_RAM_TWO_START;
    const INTERNAL_RAM_TWO_RANGE: Range<usize> =
        Self::INTERNAL_RAM_TWO_START..Self::INTERRUPT_ENABLE_REGISTER_START;

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
                Bank::InternalRamEcho => {
                    self.internal_ram_echo.set(addr, value);
                    self.internal_ram.set(addr, value);
                }
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
