use std::{
    fs::create_dir,
    io::{Read, Write},
    path::{Path, PathBuf},
    process::Command,
};

use riscv::start;

fn compile_assembly(file: &str, assembly: &str) -> Vec<u8> {
    let base_path: PathBuf = Path::new("target").join("tests");
    if !base_path.exists() {
        create_dir(&base_path).expect("create");
    }
    let assembly_file = base_path.join(&(file.to_string() + ".s"));
    let object_file = base_path.join(&(file.to_string() + ".o"));
    let binary_file = base_path.join(&(file.to_string() + ".bin"));
    std::fs::File::create(&assembly_file)
        .expect("create failed")
        .write_all(assembly.as_bytes())
        .expect("write failed");
    Command::new("clang")
        .args([
            "-Wl,-Ttext=0x0",
            "-nostdlib",
            "--target=riscv64",
            "-march=rv64g",
            "-mno-relax",
            "-o",
            &object_file.to_str().unwrap(),
            &assembly_file.to_str().unwrap(),
        ])
        .output()
        .expect("clang error");
    Command::new("llvm-objcopy")
        .args([
            "-O",
            "binary",
            &object_file.to_str().unwrap(),
            &binary_file.to_str().unwrap(),
        ])
        .output()
        .expect("llvm-binary error");
    let mut code = Vec::new();
    std::fs::File::open(binary_file)
        .expect("open failed")
        .read_to_end(&mut code)
        .expect("read failed");
    code
}

#[test]
fn test_add_instruction() {
    let code = compile_assembly(
        "add",
        "
addi x29, x0, 0x5
addi x30, x0, 0x10
add x31, x30, x29",
    );
    let cpu = start(code).0;
    assert_eq!(cpu.regs[29], 0x5);
    assert_eq!(cpu.regs[30], 0x10);
    assert_eq!(cpu.regs[31], 0x15);
}
