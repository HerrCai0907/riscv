use crate::interpreter::exception::Exception;

use super::Cpu;

mod instruction {
    pub fn get_opcode(inst: u32) -> u32 {
        inst & 0x7f
    }
    pub fn get_rd(inst: u32) -> usize {
        ((inst >> 7) & 0x1f) as usize
    }
    pub fn get_rs1(inst: u32) -> usize {
        ((inst >> 15) & 0x1f) as usize
    }
    pub fn get_rs2(inst: u32) -> usize {
        ((inst >> 20) & 0x1f) as usize
    }
    pub fn get_funct3(inst: u32) -> u32 {
        (inst >> 12) & 0x7
    }
    pub fn get_funct7(inst: u32) -> u32 {
        (inst >> 25) & 0x7f
    }
    pub fn get_shamt(inst: u32) -> u64 {
        ((inst >> 20) & 0b111111) as u64
    }
    pub fn get_shamt_reserved(inst: u32) -> u32 {
        inst >> 26
    }

    /// 20..31:imm[11:0]
    pub fn get_imm_type_i(inst: u32) -> u64 {
        ((inst as i32 as i64) >> 20) as u64
    }
    /// 12..31:imm[31:12]
    pub fn get_imm_type_u(inst: u32) -> u64 {
        (inst & (!0b1111_1111_1111)) as i32 as i64 as u64
    }
    /// 12..19:imm[19:12] & 20:imm[11] & 21..30:imm[10:1] & 31:imm[20]
    pub fn get_imm_type_j(inst: u32) -> u64 {
        let v1: i64 = (inst & 0x8000_0000) as i32 as i64 >> 11; // imm[20]
        let v2: u32 = inst & 0xff000; // imm[19:12]
        let v3: u32 = (inst >> 9) & 0x800; // imm[11]
        let v4: u32 = (inst >> 20) & 0x7fe; // imm[10:1]
        v1 as u64 | (v2 | v3 | v4) as u64
    }
    /// 7:imm[11] & 8..11:imm[4:1] & 25..30::imm[10:5] & 31:imm[12]
    pub fn get_imm_type_b(inst: u32) -> u64 {
        let v1: i64 = (inst & 0x8000_0000) as i32 as i64 >> 19; // 31:imm[12]
        let v2: u32 = (inst & 0x80) << 4; // 7:imm[11]
        let v3: u32 = (inst >> 20) & 0b111_1110_0000; // 25..30::imm[10:5]
        let v4: u32 = (inst >> 7) & 0b1_1110;
        v1 as u64 | (v2 | v3 | v4) as u64
    }
    /// 7..11:imm[4:0] & 25..31:imm[11:5]
    pub fn get_imm_type_s(inst: u32) -> u64 {
        let v1: i64 = (inst & 0xfe000000) as i32 as i64 >> 20;
        let v2 = (inst >> 7) & 0b1_1111;
        v1 as u64 | v2 as u64
    }
    // J 12..19:imm[19:12] & 20:imm[11] & 21..30:imm[10:1] & 31:imm[20]
}

fn sext(value: u64) -> u64 {
    value as u32 as i32 as i64 as u64
}
fn cut_to_u32(value: u64) -> u64 {
    value & 0xffff_ffff
}
fn wrapping_add(a: u64, b: u64) -> u64 {
    a.wrapping_add(b)
}
fn wrapping_sub(a: u64, b: u64) -> u64 {
    a.wrapping_sub(b)
}
fn signed_left_shift(v: u64, shamt: u64) -> u64 {
    (v as i64 >> shamt as i64) as u64
}

impl Cpu {
    fn increase_pc(&mut self) {
        self.pc = wrapping_add(self.pc, 4);
    }
    fn tunning_for_increase_pc(&mut self) {
        self.pc = wrapping_sub(self.pc, 4);
    }
    fn set_pc_with_tunning(&mut self, pc: u64) {
        self.pc = pc;
        self.tunning_for_increase_pc();
    }

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
            self.increase_pc();
        }
    }

    pub fn execute_instruction(&mut self, inst: u32) -> Result<(), Exception> {
        if inst == 0 || inst == 0xffff_ffff {
            return Err(Exception::InvalidInstruction);
        }
        self.regs[0] = 0;
        if inst == 0b000000000001_00000_000_00000_1110011 {
            // ebreak
            return Err(Exception::Breakpoint);
        }
        if inst == 0b000000000000_00000_000_00000_1110011 {
            // ecall
            return Err(Exception::EnvironmentCall);
        }
        match instruction::get_opcode(inst) {
            0b0000011 => {
                let rd = instruction::get_rd(inst);
                let rs1 = instruction::get_rs1(inst);
                let imm = instruction::get_imm_type_i(inst);
                let address = wrapping_add(self.read_reg(rs1), sext(imm));
                match instruction::get_funct3(inst) {
                    // lb
                    0b000 => self.write_reg(rd, sext(self.bus.load(address, 8)?)),
                    // lbu
                    0b100 => self.write_reg(rd, self.bus.load(address, 8)?),
                    // lh
                    0b001 => self.write_reg(rd, sext(self.bus.load(address, 16)?)),
                    // lhu
                    0b101 => self.write_reg(rd, self.bus.load(address, 16)?),
                    // lw
                    0b010 => self.write_reg(rd, sext(self.bus.load(address, 32)?)),
                    // lwu
                    0b110 => self.write_reg(rd, self.bus.load(address, 32)?),
                    // ld
                    0b011 => self.write_reg(rd, self.bus.load(address, 64)?),
                    _ => todo!(),
                };
            }
            0b0100011 => {
                let rs1 = instruction::get_rs1(inst);
                let rs2 = instruction::get_rs2(inst);
                let imm = instruction::get_imm_type_s(inst);
                let address = wrapping_add(self.read_reg(rs1), sext(imm));
                match instruction::get_funct3(inst) {
                    // sb
                    0b000 => self.bus.store(address, 8, self.read_reg(rs2)),
                    // sh
                    0b001 => self.bus.store(address, 16, self.read_reg(rs2)),
                    // sw
                    0b010 => self.bus.store(address, 32, self.read_reg(rs2)),
                    // sd
                    0b011 => self.bus.store(address, 64, self.read_reg(rs2)),
                    _ => todo!(),
                }?
            }
            0b0010011 => {
                let rs1_value = self.read_reg(instruction::get_rs1(inst));
                let shamt = instruction::get_shamt(inst);
                let shamt_reserved = instruction::get_shamt_reserved(inst);
                let imm = sext(instruction::get_imm_type_i(inst));
                let set_rd = &mut |value: u64| self.write_reg(instruction::get_rd(inst), value);
                match instruction::get_funct3(inst) {
                    // addi
                    0b000 => set_rd(wrapping_add(rs1_value, imm)),
                    // stli
                    0b010 => {
                        if (rs1_value as i64) < (imm as i64) {
                            set_rd(1);
                        } else {
                            set_rd(0);
                        };
                    }
                    // sltiu
                    0b011 => set_rd(if rs1_value < imm { 1 } else { 0 }),
                    // xori
                    0b100 => set_rd(rs1_value ^ imm),
                    // ori
                    0b110 => set_rd(rs1_value | imm),
                    // andi
                    0b111 => set_rd(rs1_value & imm),
                    // slli
                    0b001 if (shamt_reserved == 0b0000000) => set_rd(rs1_value << shamt),
                    // srli
                    0b101 if (shamt_reserved == 0b0000000) => set_rd(rs1_value >> shamt),
                    // srai
                    0b101 if (shamt_reserved == 0b0100000) => {
                        set_rd(signed_left_shift(rs1_value, shamt))
                    }
                    _ => todo!(),
                };
            }
            0b0110011 => {
                let rs1_value = self.read_reg(instruction::get_rs1(inst));
                let rs2_value = self.read_reg(instruction::get_rs2(inst));
                let funct7 = instruction::get_funct7(inst);
                let set_rd = &mut |value: u64| self.write_reg(instruction::get_rd(inst), value);
                match instruction::get_funct3(inst) {
                    // add
                    0b000 if (funct7 == 0b0000000) => set_rd(wrapping_add(rs1_value, rs2_value)),
                    // sub
                    0b000 if (funct7 == 0b0100000) => set_rd(wrapping_sub(rs1_value, rs2_value)),
                    // sll
                    0b001 if (funct7 == 0b0000000) => set_rd(rs1_value << rs2_value),
                    // slt
                    0b010 if (funct7 == 0b0000000) => {
                        if (rs1_value as i64) < (rs2_value as i64) {
                            set_rd(1)
                        } else {
                            set_rd(0)
                        }
                    }
                    // sltu
                    0b011 if (funct7 == 0b0000000) => {
                        if rs1_value < rs2_value {
                            set_rd(1)
                        } else {
                            set_rd(0)
                        }
                    }
                    // xor
                    0b100 if (funct7 == 0b0000000) => set_rd(rs1_value ^ rs2_value),
                    // srl
                    0b101 if (funct7 == 0b0000000) => set_rd(rs1_value >> rs2_value),
                    // sra
                    0b101 if (funct7 == 0b0100000) => {
                        set_rd(((rs1_value as i64) >> rs2_value) as u64)
                    }
                    // or
                    0b110 if (funct7 == 0b0000000) => set_rd(rs1_value | rs2_value),
                    // and
                    0b111 if (funct7 == 0b0000000) => set_rd(rs1_value & rs2_value),
                    _ => todo!(),
                }
            }
            0b0011011 => {
                let rs1_value = self.read_reg(instruction::get_rs1(inst));
                let imm = instruction::get_imm_type_i(inst);
                let shamt = instruction::get_shamt(inst);
                let shamt_reserved = instruction::get_shamt_reserved(inst);
                let set_rd = &mut |value: u64| {
                    self.write_reg(instruction::get_rd(inst), sext(cut_to_u32(value)))
                };
                match instruction::get_funct3(inst) {
                    // addiw
                    0b000 => set_rd(wrapping_add(rs1_value, sext(imm))),
                    // slliw
                    0b001 if (shamt_reserved == 0b00_0000) => set_rd(rs1_value << shamt),
                    // srliw
                    0b101 if (shamt_reserved == 0b00_0000) => set_rd(rs1_value >> shamt),
                    // sraiw
                    0b101 if (shamt_reserved == 0b01_0000) => {
                        set_rd(signed_left_shift(rs1_value, shamt))
                    }
                    _ => todo!(),
                }
            }
            0b0111011 => {
                let rs1_value = self.read_reg(instruction::get_rs1(inst));
                let rs2_value = self.read_reg(instruction::get_rs2(inst));
                let funct7 = instruction::get_funct7(inst);
                let set_rd = &mut |value: u64| {
                    self.write_reg(instruction::get_rd(inst), sext(cut_to_u32(value)))
                };
                match instruction::get_funct3(inst) {
                    // addw
                    0b000 if (funct7 == 0b000_0000) => set_rd(wrapping_add(rs1_value, rs2_value)),
                    // subw
                    0b000 if (funct7 == 0b010_0000) => set_rd(wrapping_sub(rs1_value, rs2_value)),
                    // sllw
                    0b001 if (funct7 == 0b000_0000) => set_rd(rs1_value << (rs2_value & 0b1_1111)),
                    // srlw
                    0b101 if (funct7 == 0b000_0000) => set_rd(rs1_value >> (rs2_value & 0b1_1111)),
                    // sraw
                    0b101 if (funct7 == 0b010_0000) => {
                        set_rd(signed_left_shift(rs1_value, rs2_value & 0b1_1111))
                    }
                    _ => todo!(),
                }
            }
            0b0110111 => {
                // lui
                let rd = instruction::get_rd(inst);
                let imm = instruction::get_imm_type_u(inst);
                self.write_reg(rd, sext(imm))
            }
            0b0010111 => {
                // auipc
                let rd = instruction::get_rd(inst);
                let imm = instruction::get_imm_type_u(inst);
                self.write_reg(rd, wrapping_add(self.pc, imm))
            }
            0b1100011 => {
                let imm = instruction::get_imm_type_b(inst);
                let jump_target_pc = wrapping_add(self.pc, sext(imm));
                match instruction::get_funct3(inst) {
                    0b000 => {
                        // beq
                        if self.read_reg(instruction::get_rs1(inst))
                            == self.read_reg(instruction::get_rs2(inst))
                        {
                            self.set_pc_with_tunning(jump_target_pc);
                        }
                    }
                    0b001 => {
                        // bne
                        if self.read_reg(instruction::get_rs1(inst))
                            != self.read_reg(instruction::get_rs2(inst))
                        {
                            self.set_pc_with_tunning(jump_target_pc);
                        }
                    }
                    0b100 => {
                        // blt
                        if (self.read_reg(instruction::get_rs1(inst)) as i64)
                            < (self.read_reg(instruction::get_rs2(inst)) as i64)
                        {
                            self.set_pc_with_tunning(jump_target_pc);
                        }
                    }
                    0b110 => {
                        // bltu
                        if self.read_reg(instruction::get_rs1(inst))
                            < self.read_reg(instruction::get_rs2(inst))
                        {
                            self.set_pc_with_tunning(jump_target_pc);
                        }
                    }
                    0b101 => {
                        // bge
                        if (self.read_reg(instruction::get_rs1(inst)) as i64)
                            >= (self.read_reg(instruction::get_rs2(inst)) as i64)
                        {
                            self.set_pc_with_tunning(jump_target_pc);
                        }
                    }
                    0b111 => {
                        // bgeu
                        if self.read_reg(instruction::get_rs1(inst))
                            >= self.read_reg(instruction::get_rs2(inst))
                        {
                            self.set_pc_with_tunning(jump_target_pc);
                        }
                    }
                    _ => todo!(),
                };
            }
            0b1100111 => match instruction::get_funct3(inst) {
                0b000 => {
                    // jalr
                    let t = self.pc + 4;
                    self.set_pc_with_tunning(wrapping_add(
                        self.read_reg(instruction::get_rs1(inst)),
                        sext(instruction::get_imm_type_i(inst)) & (!1u64),
                    ));
                    self.write_reg(instruction::get_rd(inst), t);
                }
                _ => todo!(),
            },
            0b1101111 => {
                // jal
                self.write_reg(instruction::get_rd(inst), wrapping_add(self.pc, 4));
                self.set_pc_with_tunning(wrapping_add(
                    self.pc,
                    sext(instruction::get_imm_type_j(inst)),
                ))
            }
            // fence
            0b0001111 => (),

            // csrc
            0b1110011 => {
                let csr = instruction::get_imm_type_i(inst) as usize;
                let rs1 = instruction::get_rs1(inst);
                let zimm = rs1 as u64 & 0b11111;
                let rd = instruction::get_rd(inst);
                let t = self.csr.load(csr);
                match instruction::get_funct3(inst) {
                    // csrrw
                    0b001 => self.csr.store(csr, self.read_reg(rs1)),
                    // csrrwi
                    0b101 => self.csr.store(csr, zimm),
                    // csrrs
                    0b010 => self.csr.store(csr, t | self.read_reg(rs1)),
                    // csrrsi
                    0b110 => self.csr.store(csr, t | zimm),
                    // csrrc
                    0b011 => self.csr.store(csr, t & !self.read_reg(rs1)),
                    // csrrci
                    0b111 => self.csr.store(csr, t & !zimm),
                    _ => todo!(),
                }
                self.write_reg(rd, t);
            }
            _ => todo!(),
        };
        Ok(())
    }
}
