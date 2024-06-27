use std::io::{stdin, stdout, Write, BufRead, BufReader};

pub trait StringReadWrite {
    fn write_to_string(&mut self, text: &str);
    fn read_from_string(&mut self, text: &mut String);
}

pub struct StringRW<R: BufRead, W: Write> {
    reader: R,
    writer: W,
}

impl <R: BufRead, W: Write> StringRW<R, W> {
    fn new(reader: R, writer: W) -> Self {
        StringRW { reader, writer }
    }
}

impl <R: BufRead, W: Write> StringReadWrite for StringRW<R, W> {
    
    fn write_to_string(&mut self, text: &str) {
        match self.writer.write(format!("{text}\n").as_bytes()) {
            Ok(_) => {}
            Err(_) => {
                eprintln!("Error writing output");
                return;
            }
        };
    
        match self.writer.flush() {
            Ok(_) => {}
            Err(_) => {
                eprintln!("Error flushing message");
                return;
            }
        };
    }

    fn read_from_string(&mut self, text: &mut String) {
        match self.reader.read_line(text) {
            Ok(_) => {}
            Err(_) => {
                eprintln!("Error reading input");
                return;
            }
        };
    }
}


pub fn get_response(question: &str) -> String {
    let mut input = String::new();
    input.clear();

    let stdout = stdout();
    let stdin = stdin();
    let stdin_reader = BufReader::new(stdin.lock());

    let mut string_io = StringRW::new(stdin_reader, stdout);

    string_io.write_to_string(question);
    string_io.read_from_string(&mut input);

    input.trim().to_string()
}
