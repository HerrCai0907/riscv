mod utils;
use riscv::interpreter::{cpu::Cpu, DRAM_BASE};
use utils::compile_assembly::compile_assembly;

#[test]
fn test_add_instruction() {
    let code = compile_assembly(function_name!(), "add x31, x30, x29");
    let mut cpu = Cpu::new(code);
    cpu.write_reg(30, 0x5);
    cpu.write_reg(29, 0x10);

    cpu.execute();

    assert_eq!(cpu.read_reg(31), 0x15);
}

#[test]
fn test_addi_instruction() {
    let code = compile_assembly(function_name!(), "addi x31, x0, 0x34");
    let mut cpu = Cpu::new(code);

    cpu.execute();

    assert_eq!(cpu.read_reg(31), 0x34);
}

#[test]
fn test_auipc_instruction() {
    let code = compile_assembly(function_name!(), "auipc x31, 0x7");
    let mut cpu = Cpu::new(code);
    let pc = cpu.pc;

    cpu.execute();

    assert_eq!(cpu.read_reg(31), pc + (0x7 << 12));
}

#[test]
fn test_lui_instruction() {
    let code = compile_assembly(function_name!(), "lui x10, 0x7");
    let mut cpu = Cpu::new(code);

    cpu.execute();

    assert_eq!(cpu.read_reg(10), 0x7 << 12);
}

#[test]
fn test_jal_instruction() {
    let code = compile_assembly(function_name!(), "jal x10, 0x16");
    let mut cpu = Cpu::new(code);
    let pc = cpu.pc;

    cpu.execute();

    assert_eq!(cpu.read_reg(10), pc + 4);
    assert_eq!(cpu.pc, pc + 0x16);
}

#[test]
fn test_jalr_instruction() {
    let code = compile_assembly(function_name!(), "jalr x10, 0x16(x11)");
    let mut cpu = Cpu::new(code);
    let pc = cpu.pc;
    cpu.write_reg(11, 0x100);

    cpu.execute();

    assert_eq!(cpu.read_reg(10), pc + 4);
    assert_eq!(cpu.pc, 0x100 + 0x16);
}

#[test]
fn test_beq_instruction() {
    {
        // eq, jump to new address
        let code = compile_assembly(function_name!(), "beq x1, x2, 16");
        let mut cpu = Cpu::new(code);
        let pc = cpu.pc;
        cpu.write_reg(1, 0x100);
        cpu.write_reg(2, 0x100);

        cpu.execute();

        assert_eq!(cpu.pc, pc + 16);
    }
    {
        // ne, execute next instruction
        let code = compile_assembly(function_name!(), "beq x1, x2, 16");
        let mut cpu = Cpu::new(code);
        let pc = cpu.pc;
        cpu.write_reg(1, 0x100);
        cpu.write_reg(2, 0x0);

        cpu.execute();

        assert_eq!(cpu.pc, pc + 4);
    }
}

#[test]
fn test_bne_instruction() {
    {
        // ne, jump to new address
        let code = compile_assembly(function_name!(), "bne x1, x2, 16");
        let mut cpu = Cpu::new(code);
        let pc = cpu.pc;
        cpu.write_reg(1, 0x100);
        cpu.write_reg(2, 0x0);

        cpu.execute();

        assert_eq!(cpu.pc, pc + 16);
    }
    {
        // eq, execute next instruction
        let code = compile_assembly(function_name!(), "bne x1, x2, 16");
        let mut cpu = Cpu::new(code);
        let pc = cpu.pc;
        cpu.write_reg(1, 0x100);
        cpu.write_reg(2, 0x100);

        cpu.execute();

        assert_eq!(cpu.pc, pc + 4);
    }
}

#[test]
fn test_blt_instruction() {
    {
        // less than, jump to new address
        let code = compile_assembly(function_name!(), "blt x1, x2, 16");
        let mut cpu = Cpu::new(code);
        let pc = cpu.pc;
        cpu.write_reg(1, 0x99);
        cpu.write_reg(2, 0x100);

        cpu.execute();

        assert_eq!(cpu.pc, pc + 16);
    }
    {
        // execute next instruction
        let code = compile_assembly(function_name!(), "blt x1, x2, 16");
        let mut cpu = Cpu::new(code);
        let pc = cpu.pc;
        cpu.write_reg(1, 0x100);
        cpu.write_reg(2, 0x100);

        cpu.execute();

        assert_eq!(cpu.pc, pc + 4);
    }
    {
        // execute next instruction
        let code = compile_assembly(function_name!(), "blt x1, x2, 16");
        let mut cpu = Cpu::new(code);
        let pc = cpu.pc;
        cpu.write_reg(1, 0x101);
        cpu.write_reg(2, 0x100);

        cpu.execute();

        assert_eq!(cpu.pc, pc + 4);
    }
}

#[test]
fn test_bge_instruction() {
    {
        // greate than, jump to new address
        let code = compile_assembly(function_name!(), "bge x1, x2, 16");
        let mut cpu = Cpu::new(code);
        let pc = cpu.pc;
        cpu.write_reg(1, 0x101);
        cpu.write_reg(2, 0x100);

        cpu.execute();

        assert_eq!(cpu.pc, pc + 16);
    }
    {
        // eq, jump to new address
        let code = compile_assembly(function_name!(), "bge x1, x2, 16");
        let mut cpu = Cpu::new(code);
        let pc = cpu.pc;
        cpu.write_reg(1, 0x100);
        cpu.write_reg(2, 0x100);

        cpu.execute();

        assert_eq!(cpu.pc, pc + 16);
    }
    {
        // execute next instruction
        let code = compile_assembly(function_name!(), "bge x1, x2, 16");
        let mut cpu = Cpu::new(code);
        let pc = cpu.pc;
        cpu.write_reg(1, 0x99);
        cpu.write_reg(2, 0x100);

        cpu.execute();

        assert_eq!(cpu.pc, pc + 4);
    }
}

#[test]
fn test_bltu_instruction() {
    {
        // less than, jump to new address
        let code = compile_assembly(function_name!(), "bltu x1, x2, 16");
        let mut cpu = Cpu::new(code);
        let pc = cpu.pc;
        cpu.write_reg(1, 0x99);
        cpu.write_reg(2, 0x100);

        cpu.execute();

        assert_eq!(cpu.pc, pc + 16);
    }
    {
        // execute next instruction
        let code = compile_assembly(function_name!(), "bltu x1, x2, 16");
        let mut cpu = Cpu::new(code);
        let pc = cpu.pc;
        cpu.write_reg(1, 0x100);
        cpu.write_reg(2, 0x100);

        cpu.execute();

        assert_eq!(cpu.pc, pc + 4);
    }
    {
        // execute next instruction
        let code = compile_assembly(function_name!(), "bltu x1, x2, 16");
        let mut cpu = Cpu::new(code);
        let pc = cpu.pc;
        cpu.write_reg(1, 0x101);
        cpu.write_reg(2, 0x100);

        cpu.execute();

        assert_eq!(cpu.pc, pc + 4);
    }
}

#[test]
fn test_bgeu_instruction() {
    {
        // greate than, jump to new address
        let code = compile_assembly(function_name!(), "bgeu x1, x2, 16");
        let mut cpu = Cpu::new(code);
        let pc = cpu.pc;
        cpu.write_reg(1, 0x101);
        cpu.write_reg(2, 0x100);

        cpu.execute();

        assert_eq!(cpu.pc, pc + 16);
    }
    {
        // eq, jump to new address
        let code = compile_assembly(function_name!(), "bgeu x1, x2, 16");
        let mut cpu = Cpu::new(code);
        let pc = cpu.pc;
        cpu.write_reg(1, 0x100);
        cpu.write_reg(2, 0x100);

        cpu.execute();

        assert_eq!(cpu.pc, pc + 16);
    }
    {
        // execute next instruction
        let code = compile_assembly(function_name!(), "bgeu x1, x2, 16");
        let mut cpu = Cpu::new(code);
        let pc = cpu.pc;
        cpu.write_reg(1, 0x99);
        cpu.write_reg(2, 0x100);

        cpu.execute();

        assert_eq!(cpu.pc, pc + 4);
    }
}

#[test]
fn test_bltu_blt_instruction() {
    {
        // 100 > -1, execute next instruction
        let code = compile_assembly(function_name!(), "blt x1, x2, 16");
        let mut cpu = Cpu::new(code);
        let pc = cpu.pc;
        cpu.write_reg(1, 0x100);
        cpu.write_reg(2, u64::MAX);

        cpu.execute();

        assert_eq!(cpu.pc, pc + 4);
    }
    {
        // 100 < MAX, jump to new address
        let code = compile_assembly(function_name!(), "bltu x1, x2, 16");
        let mut cpu = Cpu::new(code);
        let pc = cpu.pc;
        cpu.write_reg(1, 0x100);
        cpu.write_reg(2, u64::MAX);

        cpu.execute();

        assert_eq!(cpu.pc, pc + 16);
    }
}

#[test]
fn test_load_instruction() {
    {
        let code = compile_assembly(function_name!(), "lbu x1, 16(x2)");
        let mut cpu = Cpu::new(code);
        cpu.write_reg(2, DRAM_BASE);
        cpu.bus
            .store(DRAM_BASE + 16, 64, 0x01234567)
            .expect("store");

        cpu.execute();

        assert_eq!(cpu.read_reg(1), 0x67);
    }
    {
        let code = compile_assembly(function_name!(), "lhu x1, 16(x2)");
        let mut cpu = Cpu::new(code);
        cpu.write_reg(2, DRAM_BASE);
        cpu.bus
            .store(DRAM_BASE + 16, 64, 0x01234567)
            .expect("store");
        cpu.execute();

        assert_eq!(cpu.read_reg(1), 0x4567);
    }
    {
        let code = compile_assembly(function_name!(), "lwu x1, 16(x2)");
        let mut cpu = Cpu::new(code);
        cpu.write_reg(2, DRAM_BASE);
        cpu.bus
            .store(DRAM_BASE + 16, 64, 0x01234567)
            .expect("store");
        cpu.execute();

        assert_eq!(cpu.read_reg(1), 0x01234567);
    }
    {
        let code = compile_assembly(function_name!(), "lwu x1, 16(x2)");
        let mut cpu = Cpu::new(code);
        cpu.write_reg(2, DRAM_BASE);
        cpu.bus
            .store(DRAM_BASE + 16, 64, 0xf1234567)
            .expect("store");
        cpu.execute();

        assert_eq!(cpu.read_reg(1), 0x00000000_f1234567);
    }
    {
        let code = compile_assembly(function_name!(), "lw x1, 16(x2)");
        let mut cpu = Cpu::new(code);
        cpu.write_reg(2, DRAM_BASE);
        cpu.bus
            .store(DRAM_BASE + 16, 64, 0xf1234567)
            .expect("store");
        cpu.execute();

        assert_eq!(cpu.read_reg(1), 0xffffffff_f1234567);
    }
}

#[test]
fn test_store_instruction() {
    {
        let code = compile_assembly(function_name!(), "sb x1, 16(x2)");
        let mut cpu = Cpu::new(code);
        cpu.write_reg(2, DRAM_BASE);
        cpu.write_reg(1, 0x01234567);

        cpu.execute();

        assert_eq!(cpu.bus.load(DRAM_BASE + 16, 64).expect("load"), 0x67);
    }
    {
        let code = compile_assembly(function_name!(), "sh x1, 16(x2)");
        let mut cpu = Cpu::new(code);
        cpu.write_reg(2, DRAM_BASE);
        cpu.write_reg(1, 0x01234567);

        cpu.execute();

        assert_eq!(cpu.bus.load(DRAM_BASE + 16, 64).expect("load"), 0x4567);
    }
    {
        let code = compile_assembly(function_name!(), "sw x1, 16(x2)");
        let mut cpu = Cpu::new(code);
        cpu.write_reg(2, DRAM_BASE);
        cpu.write_reg(1, 0x01234567);

        cpu.execute();

        assert_eq!(cpu.bus.load(DRAM_BASE + 16, 64).expect("load"), 0x01234567);
    }
}
