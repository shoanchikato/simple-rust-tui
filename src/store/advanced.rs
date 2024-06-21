use serde_json::{from_str, to_string};
use std::fs::File;

use crate::model::post::Post;
use crate::repo::post::PostRepo;
use crate::store::base::{create_file, read_file, write_file};

pub fn on_load(repo: &mut PostRepo) {
    let file_path = "post_db.json";

    let mut load_posts = || {
        let string_posts = read_file(file_path);

        if let Ok(posts) = from_str::<Vec<Post>>(&string_posts) {
            posts.into_iter().for_each(|post| repo.add(post));
        }
    };

    if let Ok(_) = File::open(file_path) {
        load_posts();
    } else if let Ok(_) = create_file(file_path) {
        load_posts();
    } else {
        eprintln!("Error creating new file");
    }
}

pub fn on_end(repo: &PostRepo) {
    let file_path = "post_db.json";

    let write_posts = || {
        let posts = repo.get_all();

        if let Ok(string_posts) = to_string(posts) {
            write_file(file_path, string_posts);
        }
    };

    if let Ok(_) = create_file(file_path) {
        write_posts();
    } else {
        eprintln!("Error creating new file");
    }
}
