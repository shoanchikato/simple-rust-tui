use serde_json::{from_str, to_string};
use std::fs::File;

use crate::fun::post_options::PostOptions;
use crate::model::post::Post;
use crate::store::file_io::FileIO;

const FILE_NAME: &str = "post_db.json";

pub trait AppIO {
    fn on_load<P: PostOptions>(&mut self, post_rw: &mut P);
    fn on_end<P: PostOptions>(&mut self, post_rw: &mut P);
}

pub struct AppRW<'a, F: FileIO> {
    file_io: &'a F,
}

impl<'a, F: FileIO> AppRW<'a, F> {
    pub fn new(file_io: &'a F) -> Self {
        AppRW { file_io }
    }
}

impl<'a, F: FileIO> AppIO for AppRW<'a, F> {
    fn on_load<P: PostOptions>(&mut self, post_rw: &mut P) {
        let file_path = FILE_NAME;

        let mut load_posts = || {
            let string_posts = self.file_io.read_file(file_path);

            if let Ok(mut posts) = from_str::<Vec<Post>>(&string_posts) {
                post_rw.add_all(&mut posts);
            }
        };

        if File::open(file_path).is_ok() || self.file_io.create_file(file_path).is_ok() {
            load_posts();
        } else {
            eprintln!("Error creating new file");
        }
    }

    fn on_end<P: PostOptions>(&mut self, post_rw: &mut P) {
        let file_path = FILE_NAME;

        let write_posts = || {
            let posts = post_rw.get_all();

            if let Ok(string_posts) = to_string(posts) {
                self.file_io.write_file(file_path, string_posts);
            }
        };

        if self.file_io.create_file(file_path).is_ok() {
            write_posts();
        } else {
            eprintln!("Error creating new file");
        }
    }
}
