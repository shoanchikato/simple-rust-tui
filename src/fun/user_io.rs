use crate::fun::string_io::StringIO;

pub trait UserIO {
    fn get_response(&mut self, question: &str) -> String;
}

pub struct UserRW<'a> {
    string_io: &'a mut dyn StringIO,
}

impl<'a> UserRW<'a> {
    pub fn new(string_io: &'a mut dyn StringIO) -> Self {
        UserRW { string_io }
    }
}

impl<'a> UserIO for UserRW<'a> {
    fn get_response(&mut self, question: &str) -> String {
        let mut input = String::new();
        input.clear();

        self.string_io.write_to_string(question);
        self.string_io.read_from_string(&mut input);

        input.trim().to_string()
    }
}
