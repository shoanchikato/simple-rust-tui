use std::io::{BufRead, Write};

pub trait StringIO {
    fn write_to_string(&mut self, text: &str);
    fn read_from_string(&mut self, text: &mut String);
}

pub struct StringRW<R: BufRead, W: Write> {
    reader: Box<R>,
    writer: Box<W>,
}

impl<R: BufRead, W: Write> StringRW<R, W> {
    pub fn new(reader: Box<R>, writer: Box<W>) -> Self {
        StringRW { reader, writer }
    }
}

impl<R: BufRead, W: Write> StringIO for StringRW<R, W> {
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
