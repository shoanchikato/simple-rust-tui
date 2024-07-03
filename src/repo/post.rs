use std::mem;

use crate::model::post::Post;

pub trait PostIO {
    fn get_all(&self) -> &Vec<Post>;
    fn get_one(&mut self, index: usize) -> Option<&mut Post>;
    fn add(&mut self, post: Post);
    fn add_all(&mut self, posts: &mut Vec<Post>);
    fn remove(&mut self, id: usize);
    fn edit(&mut self, id: usize, title: &str, body: &str);
}

pub struct PostRW {
    posts: Vec<Post>,
}

impl PostRW {
    pub fn new(posts: Vec<Post>) -> PostRW {
        PostRW { posts }
    }
}

impl PostIO for PostRW {
    fn get_all(&self) -> &Vec<Post> {
        &self.posts
    }

    fn get_one(&mut self, index: usize) -> Option<&mut Post> {
        self.posts.get_mut(index)
    }

    fn add(&mut self, post: Post) {
        self.posts.push(post);
    }

    fn add_all(&mut self, posts: &mut Vec<Post>) {
        self.posts = mem::take(posts);
    }

    fn remove(&mut self, id: usize) {
        match self.posts.iter().position(|post| post.id == id) {
            Some(index) => {
                self.posts.remove(index);
            }
            None => {
                eprintln!("Post with id {}, not found", id);
            }
        }
    }

    fn edit(&mut self, id: usize, title: &str, body: &str) {
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
            }
        }
    }
}
