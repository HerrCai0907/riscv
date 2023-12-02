use std::fs::create_dir;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};
use std::sync::Once;

static INIT: Once = Once::new();

pub fn compile_assembly(file: &str, assembly: &str) -> Vec<u8> {
    let base_path: PathBuf = Path::new("target").join("tests");
    INIT.call_once(|| {
        if !base_path.exists() {
            create_dir(&base_path).expect("create");
        }
    });
    let assembly_file = base_path.join(&(file.to_string() + ".s"));
    let object_file = base_path.join(&(file.to_string() + ".o"));
    let binary_file = base_path.join(&(file.to_string() + ".bin"));
    std::fs::File::create(&assembly_file)
        .expect("create failed")
        .write_all(assembly.as_bytes())
        .expect("write failed");
    let output = Command::new("clang")
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
    println!("{:?}", output);
    assert_eq!(output.status, ExitStatus::default());
    let output = Command::new("llvm-objcopy")
        .args([
            "-O",
            "binary",
            &object_file.to_str().unwrap(),
            &binary_file.to_str().unwrap(),
        ])
        .output()
        .expect("llvm-binary error");
    println!("{:?}", output);
    assert_eq!(output.status, ExitStatus::default());
    let mut code = Vec::new();
    std::fs::File::open(binary_file)
        .expect("open failed")
        .read_to_end(&mut code)
        .expect("read failed");
    code
}
