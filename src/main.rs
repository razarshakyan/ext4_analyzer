pub fn print_rustc_version() {
    use std::process::{Command};
    let output = Command::new("rustc")
        .arg("--version")
        .output();

    match output {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout);
            println!("Rust compiler version: {}", version.trim());
        }
        Ok(output) => {
            eprintln!("Failed to execute rustc: {}", String::from_utf8_lossy(&output.stderr));
        }
        Err(e) => {
            eprintln!("Error running rustc: {}", e);
        }
    }
}

#[cfg(target_os = "linux")]
mod lin {
    use std::path::{PathBuf};
    use ext4_view::{Ext4};
    use std::{fs};
    use crate::print_rustc_version;

    pub fn run() -> Result<(), Box<dyn std::error::Error>> {
        let path = PathBuf::from(r"/dev/nvme0n1p7");
        let fs = Ext4::load_from_path(&path)?;

        let file_data: Vec<u8> = fs.read("/etc/passwd")?;
        let _ = fs::write("/home/mri.txt", &file_data);
        print_rustc_version();
        Ok(())
    }
}

#[cfg(target_os = "windows")]
mod win {
    use crate::print_rustc_version;

    #[no_mangle]
    pub extern "C" fn compute_vals() -> (i32, f64) {
        let i = 44;
        let f = 1.12;
        (i, f)
    }
    pub fn run() {
        let (i, f) = compute_vals();
        println!("{} {}", i, f);
        print_rustc_version()
    }
}

fn main() {
    #[cfg(target_os = "linux")]
    {
        let a = lin::run();
        println!("dbg {:?}", a);
    }

    #[cfg(target_os = "windows")]
    {   
        let _ = win::run();
    }
}
