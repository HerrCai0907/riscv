use crate::interpreter::exception::Exception;

use super::Cpu;

enum Instruction {
    Add = 0b0110011,
    Addi = 0b0010011,
}

impl TryFrom<u32> for Instruction {
    type Error = Exception;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        match v {
            x if x == Instruction::Addi as u32 => Ok(Instruction::Addi),
            x if x == Instruction::Add as u32 => Ok(Instruction::Add),
            _ => Err(Exception::UnknownInstructionOpcode { opcode: v }),
        }
    }
}

impl Cpu {
    pub fn execute(&mut self) -> Option<Exception> {
        loop {
            let inst = match self.instructure_fetch() {
                Ok(inst) => inst,
                Err(err) => return Some(err),
            };
            match self.execute_instruction(inst) {
                Ok(_) => (),
                Err(err) => return Some(err),
            };
            self.pc += 4;
        }
    }

    pub fn execute_instruction(&mut self, inst: u32) -> Result<(), Exception> {
        // decode as R-type
        let opcode = Instruction::try_from(inst & 0x7f)?;
        let rd = ((inst >> 7) & 0x1f) as usize;
        let rs1 = ((inst >> 15) & 0x1f) as usize;
        let rs2 = ((inst >> 20) & 0x1f) as usize;
        // let funct3 = (inst >> 12) & 0x7;
        // let funct7 = (inst >> 25) & 0x7f;
        self.regs[0] = 0;
        match opcode {
            Instruction::Addi => {
                let imm = ((inst & 0xfff0_0000) as i64 >> 20) as u64;
                self.regs[rd] = self.regs[rs1].wrapping_add(imm);
            }
            Instruction::Add => {
                self.regs[rd] = self.regs[rs1].wrapping_add(self.regs[rs2]);
            }
        }
        Ok(())
    }
}
