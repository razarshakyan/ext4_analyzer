use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let target = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    println!("target OS {}", target.as_str());

    let desired_version = match target.as_str() {
        "linux" => "1.81.0",
        "windows" => "1.77.0",
        _ => "stable",
    };

    let prepare_build = Command::new("bash")
        .arg("prepare_build.sh")
        .status()
        .expect("Failed to execute prepare_build.sh");
    if !prepare_build.success() {
        panic!("prepare_build.sh failed");
    }
    println!("prepare_build succeeded");

    let output = Command::new("rustc")
        .arg("--version")
        .output()
        .expect("Failed to execute rustc");
    let version_info = String::from_utf8_lossy(&output.stdout);

    if !version_info.contains(desired_version) {
        println!(
            "cargo:warning=Expected Rust version {} but found: {}",
            desired_version, version_info
        );
    }
}
