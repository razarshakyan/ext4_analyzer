extern crate ext4;

use std::io::{Read, Write, BufReader, BufWriter};
use std::fs::{File, OpenOptions};

fn create_stat_file(file_name: &str) -> File {
    OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(file_name)
        .expect("FAILED: File Creation")
}

fn write_inode_into_dir(mut device: std::fs::File) {
    let mut ctr = 0;
    let super_block = ext4::SuperBlock::new(&mut device).unwrap();
    let root = super_block.root().unwrap();

    super_block.walk(&root, "", &mut |fs, path, inode, enhanced| {
        if ctr == 5 {
            return Ok(false);
        }

        ctr += 1;
        println!(
            "<{}> {}: {:?} {:?}",
            inode.number, path, enhanced, inode.stat
        );

        if inode.stat.extracted_type == ext4::FileType::RegularFile {
            if let Ok(file) = fs.open(&inode) {
                let mut reader = BufReader::new(file);
                let output_file = File::create(format!("inode_output_{}.txt", inode.number))?;
                let mut writer = BufWriter::new(output_file);

                let mut buffer = [0u8; 4096];
                while let Ok(bytes_read) = reader.read(&mut buffer) {
                    if bytes_read == 0 {
                        break;
                    }
                    writer.write_all(&buffer[..bytes_read])?;
                }
                writer.flush()?;
            } else {
                eprintln!("Failed to open inode {}", inode.number);
            }
        }

        Ok(true) // Return true to continue the walk.
    }).unwrap_or_else(|err| {
        eprintln!("Error during walk: {:?}", err);
        false
    });
}

fn extract_inode_info(
    mut device: std::fs::File,
    root_path: &str)
{
    let super_block = ext4::SuperBlock::new(&mut device).unwrap();
    let target_inode =
        super_block.resolve_path(root_path).unwrap().inode;
    let root_inode = super_block.load_inode(target_inode).unwrap();
    let mut file = create_stat_file("inodes.txt");
    let _ = super_block.walk(&root_inode, root_path, &mut |_, path, _, _| {
        let target_inode = super_block.resolve_path(path).unwrap().inode;
        let root_inode = super_block.load_inode(target_inode).unwrap();
        let inode_stats = format!(
            "Path : {},\n\
            File Mode : {},\n\
            UID : {},\n\
            Size : {},\n\
            Inode Number : {}\n",
            path, root_inode.stat.file_mode,
            root_inode.stat.uid, root_inode.stat.size,
            root_inode.number
        );

        if let Err(e) = file.write_all(inode_stats.as_bytes()) {
            eprintln!("FAILED : File Write {}", e);
            return Ok(false);
        }
        Ok(true)
    });
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: {} <filesystem> <start_dir> <artifact>", args[0]);
        std::process::exit(1);
    }

    let fs_path = &args[1];
    let start_dir = &args[2];
    let artifact = &args[3];

    let device =
        std::fs::File::open(fs_path.as_str()).unwrap();
    println!("device : {:?}", device);

    match artifact.to_lowercase().as_str() {
        "inodes" => extract_inode_info(device, start_dir),
        "write_inode" => write_inode_into_dir(device),
        _ => eprintln!("Invalid Artifact : Use [inodes] or [write_inode]"),
    }
}
