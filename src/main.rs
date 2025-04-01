#[cfg(target_os = "linux")]
mod lin {
    use std::path::PathBuf;
    use ext4_view::Ext4;
    use std::{fs};
    
    pub fn run() -> Result<(), Box<dyn std::error::Error>> {
        let path = PathBuf::from(r"/dev/nvme0n1p7");
        let fs = Ext4::load_from_path(&path)?;

        let file_data: Vec<u8> = fs.read("/etc/passwd")?;
        let _ = fs::write("/home/mri.txt", &file_data);
        Ok(())
    }
}

#[cfg(target_os = "windows")]
mod win {
    #[no_mangle]
    pub extern "C" fn compute_vals() -> (i32, f64) {
        let i = 44;
        let f = 1.12;
        (i, f)
    }
    pub fn run() {
        let (i, f) = compute_vals();
        println!("{} {}", i, f);
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
