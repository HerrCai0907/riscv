mod interpreter;
use interpreter::{cpu::Cpu, exception::Exception};

pub fn start(code: Vec<u8>) -> (Cpu, Exception) {
    let mut cpu = Cpu::new(code);
    loop {
        match cpu.execute() {
            Some(err) => return (cpu, err),
            None => (),
        };
    }
}
