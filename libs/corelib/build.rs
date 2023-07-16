use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::process::Command;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();

    let bitcode_path = Path::new(&out_dir).join("stdlib.bc");

    assert!(Command::new("rustc")
        .args([
            "--emit",
            "llvm-bc",
            "--crate-type",
            "dylib",
            "-o",
            bitcode_path.to_str().unwrap(),
            "./src/lib.rs"
        ])
        .spawn()
        .unwrap()
        .wait_with_output()
        .unwrap()
        .status
        .success());

    let bitcode = {
        let mut file = File::open(bitcode_path).expect("Failed to open file");
        let mut content = Vec::new();
        file.read_to_end(&mut content)
            .expect("Failed to read from file");
        content
    };

    let bitcode_path = Path::new(&out_dir).join("corelib.bc");

    let mut file = File::create(bitcode_path).expect("Failed to create file");

    file.write_all(bitcode.as_slice())
        .expect("Failed to write to file");
}
