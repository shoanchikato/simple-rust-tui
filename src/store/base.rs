use std::fs::{read_to_string, File, OpenOptions};
use std::io::{Error, Write};
use std::path::Path;

pub fn create_file(file_path: &str) -> Result<File, Error> {
    let path = Path::new(file_path);
    let _ = path.display();

    File::create(path)
}

pub fn write_file(file_path: &str, contents: String) {
    let file = OpenOptions::new().write(true).create(true).open(file_path);

    match file {
        Ok(mut file) => {
            if let Err(e) = file.write_all(contents.as_bytes()) {
                eprintln!("Failed to write to file: {}", e);
            }
        }
        Err(e) => eprintln!("Failed to open or create file: {}", e),
    }
}

pub fn read_file(file_path: &str) -> String {
    let path = Path::new(file_path);
    let display = path.display();

    match path.is_file() {
        false => {
            eprintln!("file doesn't exist");
            return String::from("");
        }
        _ => {}
    }

    match read_to_string(&path) {
        Err(why) => {
            eprintln!("couldn't read {}: {}", display, why);
            return String::from("");
        }
        Ok(file_contents) => file_contents,
    }
}
