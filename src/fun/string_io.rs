use std::io::{BufRead, Write};

pub trait StringIO {
    fn write_to_string(&mut self, text: &str);
    fn read_from_string(&mut self, text: &mut String);
}

pub struct StringRW<'a, R: BufRead, W: Write> {
    reader: &'a mut R,
    writer: &'a mut W,
}

impl<'a, R: BufRead, W: Write> StringRW<'a, R, W> {
    pub fn new(reader: &'a mut R, writer: &'a mut W) -> Self {
        StringRW { reader, writer }
    }
}

impl<'a, R: BufRead, W: Write> StringIO for StringRW<'a, R, W> {
    fn write_to_string(&mut self, text: &str) {
        match self.writer.write_all(format!("{text}\n").as_bytes()) {
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
                return;
            }
        }
    }

    fn read_from_string(&mut self, text: &mut String) {
        match self.reader.read_line(text) {
            Ok(_) => {}
            Err(_) => {
                eprintln!("Error reading input");
                return;
            }
        }
    }
}
