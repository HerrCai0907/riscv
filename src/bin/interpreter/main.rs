use riscv::interpreter::cpu::Cpu;
use std::io::Read;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!(
            "Usage:\n\
            - cargo run <filename>"
        );
        return;
    }
    let mut file = std::fs::File::open(&args[1]).unwrap_or_else(|error| {
        println!("cannot open file '{}': {:}", args[1], error);
        std::process::exit(1);
    });
    let mut code = Vec::new();
    file.read_to_end(&mut code).unwrap_or_else(|error| {
        println!("cannot read file '{}': {:}", args[1], error);
        std::process::exit(1);
    });

    match Cpu::new(code).execute() {
        Some(e) => panic!("{:?}", e),
        None => (),
    };
}
