use std::io::{BufRead, Write};

pub trait StringIO {
    fn write_to_string(&mut self, text: &str);
    fn read_from_string(&mut self, text: &mut String);
}

pub struct StringRW {
    reader: Box<dyn BufRead>,
    writer: Box<dyn Write>,
}

impl StringRW {
    pub fn new(reader: Box<dyn BufRead>, writer: Box<dyn Write>) -> Self {
        StringRW { reader, writer }
    }
}

impl StringIO for StringRW {
    fn write_to_string(&mut self, text: &str) {
        match self.writer.write_all(format!("\n{text}\n").as_bytes()) {
            Ok(_) => {}
            Err(_) => {
                eprintln!("Error writing output");
                return;
            }
        }

        match self.writer.flush() {
            Ok(_) => {}
            Err(_) => {
                eprintln!("Error flushing message");
            }
        }
    }

    fn read_from_string(&mut self, text: &mut String) {
        match self.reader.read_line(text) {
            Ok(_) => {}
            Err(_) => {
                eprintln!("Error reading input");
            }
        }
    }
}
