use std::path::Path;
use std::fs;

fn main() {
    println!("Hello, world!");
    create_database();
}

fn create_database() {
    // essentially it would create a directory?
    let path: &Path = Path::new("./database");
    fs::create_dir_all(path).unwrap();
    // Ok(())
}

