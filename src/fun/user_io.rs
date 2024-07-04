use crate::fun::string_io::StringIO;

pub trait UserIO {
    fn get_response(&mut self, question: &str) -> String;
}

pub struct UserRW<S: StringIO> {
    string_io: Box<S>,
}

impl<S: StringIO> UserRW<S> {
    pub fn new(string_io: Box<S>) -> Self {
        UserRW { string_io }
    }
}

impl<S: StringIO> UserIO for UserRW<S> {
    fn get_response(&mut self, question: &str) -> String {
        let mut input = String::new();
        input.clear();

        self.string_io.write_to_string(question);
        self.string_io.read_from_string(&mut input);

        input.trim().to_string()
    }
}
