use crate::fun::string_io::StringIO;

pub trait UserIO {
    fn get_response(&mut self, question: &str) -> String;
}

pub struct UserRW {
    string_io: Box<dyn StringIO>,
}

impl UserRW {
    pub fn new(string_io: Box<dyn StringIO>) -> Self {
        UserRW { string_io }
    }
}

impl UserIO for UserRW {
    fn get_response(&mut self, question: &str) -> String {
        let mut input = String::new();
        input.clear();

        self.string_io.write_to_string(question);
        self.string_io.read_from_string(&mut input);

        input.trim().to_string()
    }
}
