use walkdir::WalkDir;
use std::fs;
use std::io::Read;
use anyhow::Result;

pub fn start_indexing() -> Result<()> {
    let folder_path = std::env::current_dir().unwrap().join("../../files-to-index");
    for entry in WalkDir::new(folder_path) {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let mut file = fs::File::open(path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            println!("File: {:?}, Content: {}", path, contents);
        }
    }
    Ok(())
}
