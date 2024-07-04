use std::io::{BufRead, Write};

pub trait StringIO {
    fn write_to_string(&mut self, text: &str);
    fn read_from_string(&mut self, text: &mut String);
}

pub struct StringRW<'a> {
    reader: &'a mut dyn BufRead,
    writer: &'a mut dyn Write,
}

impl<'a> StringRW<'a> {
    pub fn new(reader: &'a mut dyn BufRead, writer: &'a mut dyn Write) -> Self {
        StringRW { reader, writer }
    }
}

impl<'a> StringIO for StringRW<'a> {
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
