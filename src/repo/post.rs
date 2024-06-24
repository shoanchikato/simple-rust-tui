use std::mem;

use crate::model::post::Post;

pub struct PostRepo {
    posts: Vec<Post>,
}

impl PostRepo {
    pub fn new() -> PostRepo {
        PostRepo { posts: vec![] }
    }

    pub fn get_all(&self) -> &Vec<Post> {
        &self.posts
    }

    pub fn get_one(&mut self, index: usize) -> Option<&mut Post> {
        self.posts.get_mut(index)
    }

    pub fn add(&mut self, post: Post) {
        self.posts.push(post);
    }

    pub fn remove(&mut self, id: usize) {
        match self.posts.iter().position(|post| post.id == id) {
            Some(index) => {
                self.posts.remove(index);
            }
            None => {
                eprintln!("Post with id {}, not found", id);
                return;
            }
        }
    }

    pub fn edit(&mut self, id: usize, title: &str, body: &str) {
        match self.posts.iter_mut().find(|post| post.id == id) {
            Some(post) => {
                post.title = if title.is_empty() {
                    mem::take(&mut post.title)
                } else {
                    String::from(title)
                };

                post.body = if body.is_empty() {
                    mem::take(&mut post.body)
                } else {
                    String::from(body)
                };
            }
            None => {
                eprintln!("Post with id {}, not found", id);
                return;
            }
        }
    }
}
