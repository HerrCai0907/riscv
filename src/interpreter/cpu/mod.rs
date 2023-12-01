pub mod debug;
pub mod execute;

pub struct Cpu {
    pub regs: [u64; 32], // RISC-V has 32 registers
    pub pc: u64,
    pub dram: Vec<u8>,
}

impl Cpu {
    pub fn new(code: Vec<u8>) -> Self {
        const DRAM_SIZE: u64 = 1024 * 1024 * 128;
        let mut regs = [0; 32];
        regs[2] = DRAM_SIZE - 1;
        Self {
            regs,
            pc: 0,
            dram: code,
        }
    }

    pub fn instructure_fetch(&self) -> u32 {
        let index = self.pc as usize;
        let inst = self.dram[index] as u32
            | ((self.dram[index + 1] as u32) << 8)
            | ((self.dram[index + 2] as u32) << 16)
            | ((self.dram[index + 3] as u32) << 24);
        return inst;
    }
}
