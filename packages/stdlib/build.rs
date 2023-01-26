use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::process::Command;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();

    let bitcode_path = Path::new(&out_dir).join("stdlib.bc");

    Command::new("rustc")
        .args(&["--emit", "llvm-bc", "--crate-type", "dylib", "-o", bitcode_path.to_str().unwrap(), "./src/stdlib.rs"])
        .output()
        .expect("Failed to compile");

    let bitcode = {
        let mut file = File::open(bitcode_path).expect("Failed to open file");
        let mut content = Vec::new();
        file.read_to_end(&mut content).expect("Failed to read from file");
        content
    };

    let bitcode_path = Path::new(&out_dir).join("stdlib_bc.rs");

    let mut file = File::create(bitcode_path)
        .expect("Failed to create file");

    let bitcode = format!("pub const BITCODE: &[u8; {}] = &{:?};\n", bitcode.len(), bitcode);
    file.write_all(bitcode.as_bytes()).expect("Failed to write to file");
}
