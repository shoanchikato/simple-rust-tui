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
        match self.posts.get(id) {
            Some(_) => {
                self.posts.remove(id);
            }
            None => {
                eprint!("Post with id {}, not found", id);
                return;
            }
        }
    }

    pub fn edit(&mut self, index: usize, post: &Post) {
        match self.posts.get_mut(index) {
            Some(old_post) => {
                old_post.title = if post.title.clone().is_empty() {
                    old_post.title.clone()
                } else {
                    post.title.clone()
                };

                old_post.body = if post.body.clone().is_empty() {
                    old_post.body.clone()
                } else {
                    post.body.clone()
                };
            }

            None => {
                eprint!("Post with id {}, not found", index);
                return;
            }
        }
    }
}
