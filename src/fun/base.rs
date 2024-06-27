use std::io::{stdin, stdout, Write, BufRead, BufReader};

pub fn write_to_string<W: Write>(writer: &mut W, text: &str) {
    match writer.write(format!("{text}\n").as_bytes()) {
        Ok(_) => {}
        Err(_) => {
            eprintln!("Error writing output");
            return;
        }
    };

    match writer.flush() {
        Ok(_) => {}
        Err(_) => {
            eprintln!("Error flushing message");
            return;
        }
    };
}

pub fn read_from_string<R: BufRead>(reader: &mut R, mut text: &mut String) {
    match reader.read_line(&mut text) {
        Ok(_) => {}
        Err(_) => {
            eprintln!("Error reading input");
            return;
        }
    };
}

pub fn get_response(question: &str) -> String {
    let mut input = String::new();
    input.clear();

    let mut stdout = stdout();
    let stdin = stdin();
    let mut stdin_reader = BufReader::new(stdin.lock());

    write_to_string(&mut stdout, question);
    read_from_string(&mut stdin_reader, &mut input);

    input.trim().to_string()
}
