use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;


#[derive(Debug)]
struct Record<T> {
    fields: Vec<T>
}

fn main() {
    println!("Hello, world!");
    let mut database: fs::File = create_database();
    database.write_all(b"writing to file.").unwrap();
    println!("{:?}", database);

    delete_database();
}

fn create_database() ->  fs::File {
    // essentially it would create a directory?
    let path: &Path = Path::new("./database");
    fs::create_dir_all(path).unwrap();  // Actuallly returns ()
    
    let file_path: &Path = Path::new("./database/db.shwrm");
    let file: fs::File = fs::File::create(file_path).unwrap();
    return file;
    // Ok(())
}

fn delete_database() {
    let database_path: &Path = Path::new("./database");
    fs::remove_dir_all(database_path).unwrap()
}

fn create_table() {
    unimplemented!();
}

fn create_columns() {
    unimplemented!();
}

fn create_record() {
    unimplemented!();
}

fn create_records() {
    unimplemented!();
}