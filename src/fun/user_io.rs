use crate::fun::string_io::StringIO;

pub trait UserIO {
    fn get_response(&mut self, question: &str) -> String;
}

pub struct UserRW<'a, S: StringIO> {
    string_io: &'a mut S,
}

impl<'a, S: StringIO> UserRW<'a, S> {
    pub fn new(string_io: &'a mut S) -> Self {
        UserRW { string_io }
    }
}

impl<'a, S: StringIO> UserIO for UserRW<'a, S> {
    fn get_response(&mut self, question: &str) -> String {
        let mut input = String::new();
        input.clear();

        self.string_io.write_to_string(question);
        self.string_io.read_from_string(&mut input);

        input.trim().to_string()
    }
}
