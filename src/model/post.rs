use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    pub id: usize,
    pub title: String,
    pub body: String,
}

impl Post {
    pub fn new(id: usize, title: String, body: String) -> Post {
        Post { id, title, body }
    }
}

impl Display for Post {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "Post {}\ntitle: {}\nbody: {}\n",
            self.id, self.title, self.body
        )
    }
}
