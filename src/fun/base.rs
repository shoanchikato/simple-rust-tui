use std::io::{stdin, stdout, Write};

pub fn write_stdout(text: &str) {
    match stdout().write(format!("{text}\n").as_bytes()) {
        Ok(_) => {}
        Err(_) => {
            eprintln!("Error writing output");
            return;
        }
    };

    match stdout().flush() {
        Ok(_) => {}
        Err(_) => {
            eprintln!("Error flushing message");
            return;
        }
    };
}

pub fn read_stdin(mut text: &mut String) {
    match stdin().read_line(&mut text) {
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
    write_stdout(question);
    read_stdin(&mut input);

    input.trim().to_string()
}
