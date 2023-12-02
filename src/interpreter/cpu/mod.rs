use super::{bus::Bus, exception::Exception, DRAM_BASE, DRAM_END};
pub mod debug;
pub mod execute;

pub struct Cpu {
    pub regs: [u64; 32], // RISC-V has 32 registers
    pub pc: u64,
    pub bus: Bus,
}

impl Cpu {
    pub fn new(code: Vec<u8>) -> Self {
        let mut regs = [0; 32];
        regs[2] = DRAM_END;
        Self {
            regs,
            pc: DRAM_BASE,
            bus: Bus::new(code),
        }
    }

    pub fn write_reg(&mut self, reg: usize, value: u64) {
        self.regs[reg] = value;
    }
    pub fn read_reg(&self, reg: usize) -> u64 {
        self.regs[reg]
    }

    pub fn load(&mut self, addr: u64, size: u64) -> Result<u64, Exception> {
        self.bus.load(addr, size)
    }

    pub fn store(&mut self, addr: u64, size: u64, value: u64) -> Result<(), Exception> {
        self.bus.store(addr, size, value)
    }

    pub fn instructure_fetch(&mut self) -> Result<u32, Exception> {
        self.bus.load(self.pc, 32).map(|inst| inst as u32)
    }
}
