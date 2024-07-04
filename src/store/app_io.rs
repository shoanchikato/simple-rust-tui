use serde_json::{from_str, to_string};
use std::fs::File;

use crate::fun::post_options::PostOptions;
use crate::model::post::Post;
use crate::store::file_io::FileIO;

const FILE_NAME: &str = "post_db.json";

pub trait AppIO {
    fn on_load(&mut self);
    fn on_end(&mut self);
}

pub struct AppRW<F, P> {
    file_io: Box<F>,
    post_rw: Box<P>,
}

impl<F: FileIO, P: PostOptions> AppRW<F, P> {
    pub fn new(file_io: Box<F>, post_rw: Box<P>) -> Self {
        AppRW { file_io, post_rw }
    }
}

impl<F: FileIO, P: PostOptions> AppIO for AppRW<F, P> {
    fn on_load(&mut self) {
        let file_path = FILE_NAME;

        let mut load_posts = || {
            let string_posts = self.file_io.read_file(file_path);

            if let Ok(mut posts) = from_str::<Vec<Post>>(&string_posts) {
                self.post_rw.add_all(&mut posts);
            }
        };

        if File::open(file_path).is_ok() || self.file_io.create_file(file_path).is_ok() {
            load_posts();
        } else {
            eprintln!("Error creating new file");
        }
    }

    fn on_end(&mut self) {
        let file_path = FILE_NAME;

        let write_posts = || {
            let posts = self.post_rw.get_all();

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

impl <F: FileIO, P: PostOptions> PostOptions for AppRW <F, P> {
    fn show_posts(&mut self) {
        self.post_rw.show_posts()
    }

    fn write_post(&mut self) {
        self.post_rw.write_post()
    }

    fn edit_post(&mut self) {
        self.post_rw.edit_post()
    }

    fn remove_post(&mut self) {
        self.post_rw.remove_post()
    }

    fn get_all(&self) -> &Vec<Post> {
        self.post_rw.get_all()
    }

    fn add_all(&mut self, posts: &mut Vec<Post>) {
        self.post_rw.add_all(posts);
    }

    fn get_response(&mut self, question: &str) -> String {
        self.post_rw.get_response(question)
    }
}
