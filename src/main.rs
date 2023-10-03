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
    // database.write_all(b"writing to file.").unwrap();
    // println!("{:?}", &database);

    // delete_database();
    create_columns(&mut database);
    create_record(&mut database);
}

fn create_database() ->  fs::File {
    // essentially it would create a directory?
    let path: &Path = Path::new("./database");
    fs::create_dir_all(path).unwrap();  // Actuallly returns ()
    
    let file_path: &Path = Path::new("./database/db.shwrm");
    if file_path.try_exists().unwrap() {
        let file: fs::File = fs::File::create(file_path).unwrap();
        return file;
    } else {
        let file: fs::File = fs::File::options().append(true).open(file_path).unwrap();
        todo!("This shouldn't work, you need fs::OpenOptions::new().append(true).open(...)");
        return file;
    }
    // Ok(())
}

fn delete_database() {
    let mut database_path: &Path = Path::new("./database");
    fs::remove_dir_all(database_path).unwrap()
}

fn create_table(database: &mut fs::File) {
    // unimplemented!();
    database.write(b"columns").unwrap();
    ()
}

fn create_columns(database: &mut fs::File) {
    // unimplemented!();
    database.write_all(b"| names | address |\n").unwrap();
    database.
    ()
}

fn create_record(database: &mut fs::File) {
    // unimplemented!();
    database.write_all(b"| Jane Doe | T23 YT44|\n").unwrap();
    ()
}

fn create_records() {
    unimplemented!();
}