mod interpreter;
use interpreter::cpu::Cpu;

pub fn start(code: Vec<u8>) -> Cpu {
    let mut cpu = Cpu::new(code);
    while cpu.pc < cpu.dram.len() as u64 {
        let inst = cpu.instructure_fetch();
        cpu.execute(inst);
        // cpu.dump_registers();
        cpu.pc += 4;
    }
    cpu
}
