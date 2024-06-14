use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use fs_extra::dir::create_all;

fn main() {
    let directory = "path/to/your/directory"; // Replace with the path to your directory

    match organize_files_by_extension(directory) {
        Ok(_) => println!("Files organized successfully!"),
        Err(e) => eprintln!("Error organizing files: {}", e),
    }
}

fn organize_files_by_extension(directory: &str) -> Result<(), Box<dyn std::error::Error>> {
    for entry in WalkDir::new(directory).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                let ext_str = ext.to_string_lossy().to_lowercase();
                let target_dir = format!("{}/{}", directory, ext_str);

                create_all(&target_dir, true)?;
                move_file_to_directory(path, &target_dir)?;
            }
        }
    }
    Ok(())
}

fn move_file_to_directory(file_path: &Path, target_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file_name = file_path.file_name().ok_or("Invalid file name")?;
    let mut target_path = PathBuf::from(target_dir);
    target_path.push(file_name);

    fs::rename(file_path, target_path)?;
    Ok(())
}
