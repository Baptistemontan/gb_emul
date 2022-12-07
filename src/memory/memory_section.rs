use super::Memory;

#[derive(Debug)]
pub struct MemorySection<const N: usize> {
    mem: [u8; N],
}

impl<const N: usize> MemorySection<N> {
    pub fn new() -> Self {
        MemorySection { mem: [0; N] }
    }

    pub fn get(&self, addr: usize) -> u8 {
        self.mem[addr]
    }

    pub fn set(&mut self, addr: usize, value: u8) {
        self.mem[addr] = value;
    }
}

impl<const N: usize> Default for MemorySection<N> {
    fn default() -> Self {
        Self::new()
    }
}
