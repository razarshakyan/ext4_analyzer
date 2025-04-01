use std::process::Command;

fn main() {
    let target = std::env::var("CARGO_CFG_TARGET_OS").unwrap();

    let (rust_version, expected_version) = match target.as_str() {
        "linux" => ("1.81.0", "1.81.0"),
        "windows" => ("1.77.0", "1.77.0"),
        _ => ("stable", "stable"),
    };

    println!("cargo:rerun-if-changed=build.rs");

    let output = Command::new("rustc")
        .arg("--version")
        .output()
        .expect("Failed to execute rustc");

    let output_str = String::from_utf8_lossy(&output.stdout);
    if !output_str.contains(expected_version) {
        println!("Switching Rust version to {}", rust_version);
        if let Err(e) = Command::new("rustup")
            .args(["override", "set", rust_version])
            .status()
        {
            eprintln!("Warning: Failed to set Rust version to {}: {:?}", rust_version, e);
        }
    }
}
