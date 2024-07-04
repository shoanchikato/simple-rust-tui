use std::fs::{read_to_string, File, OpenOptions};
use std::io::{Error, Write};
use std::path::Path;

pub trait FileIO {
    fn create_file(&self, file_path: &str) -> Result<File, Error>;
    fn write_file(&self, file_path: &str, contents: String);
    fn read_file(&self, file_path: &str) -> String;
}

pub struct FileRW;

impl Default for FileRW {
    fn default() -> Self {
        Self::new()
    }
}

impl FileRW {
    pub fn new() -> Self {
        FileRW
    }
}

impl FileIO for FileRW {
    fn create_file(&self, file_path: &str) -> Result<File, Error> {
        let path = Path::new(file_path);
        let _ = path.display();

        File::create(path)
    }

    fn write_file(&self, file_path: &str, contents: String) {
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(file_path);

        match file {
            Ok(mut file) => {
                if let Err(e) = file.write_all(contents.as_bytes()) {
                    eprintln!("Failed to write to file: {}", e);
                }
            }
            Err(e) => eprintln!("Failed to open or create file: {}", e),
        }
    }

    fn read_file(&self, file_path: &str) -> String {
        let path = Path::new(file_path);
        let display = path.display();

        if !path.is_file() {
            eprintln!("file doesn't exist");
            return String::from("");
        }

        match read_to_string(path) {
            Err(why) => {
                eprintln!("couldn't read {}: {}", display, why);
                String::from("")
            }
            Ok(file_contents) => file_contents,
        }
    }
}
