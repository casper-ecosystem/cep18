use std::{env, fs, path::PathBuf, process::Command};

// Build system
const CONTRACT_ROOT: &str = "../contract";
const CONTRACT_CARGO_TOML: &str = "../contract/Cargo.toml";
const CONTRACT_LIB_RS: &str = "../contract/src/lib.rs";
const BUILD_ARGS: [&str; 4] = ["build", "--release", "-p", "contract"];
const WASM_FILENAME: &str = "contract.wasm";
const ORIGINAL_WASM_DIR: &str = "../target/wasm32-unknown-unknown/release";
const NEW_WASM_DIR: &str = "wasm";

fn main() {
    // Watch contract source files for changes.
    println!("cargo:rerun-if-changed={}", CONTRACT_CARGO_TOML);
    println!("cargo:rerun-if-changed={}", CONTRACT_LIB_RS);

    // Build the contract.
    let output = Command::new("cargo")
        .current_dir(CONTRACT_ROOT)
        .args(&BUILD_ARGS)
        .output()
        .expect("Expected to build Wasm contracts");
    assert!(
        output.status.success(),
        "Failed to build Wasm contracts:\n{:?}",
        output
    );

    // // Move the compiled Wasm file to our own build folder ("wasm/contract.wasm").
    let new_wasm_dir = env::current_dir().unwrap().join(NEW_WASM_DIR);
    let _ = fs::create_dir(&new_wasm_dir);
    let original_wasm_file = PathBuf::from(ORIGINAL_WASM_DIR).join(WASM_FILENAME);
    let copied_wasm_file = new_wasm_dir.join(WASM_FILENAME);
    fs::copy(original_wasm_file, copied_wasm_file).unwrap();
}
