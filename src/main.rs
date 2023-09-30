use std::fs;
use std::io;
use std::path::Path;


#[derive(Debug)]
struct Record<T> {
    fields: Vec<T>
}

fn main() {
    println!("Hello, world!");
    _ = create_database();
}

fn create_database() -> io::Result<()> {
    // essentially it would create a directory?
    let path: &Path = Path::new("./database");
    fs::create_dir_all(path).unwrap();  // Actuallly returns ()
    
    let file_path: &Path = Path::new("./database/db.shwrm");
    let mut file: fs::File = fs::File::create(file_path)?;  // panics if error I think...
    Ok(())
}

