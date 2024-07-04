use std::mem;

use crate::fun::user_io::UserIO;
use crate::model::post::Post;
use crate::repo::post::PostIO;

pub trait PostOptions {
    fn show_posts(&mut self);
    fn write_post(&mut self);
    fn edit_post(&mut self);
    fn remove_post(&mut self);
    fn get_all(&self) -> &Vec<Post>;
    fn add_all(&mut self, posts: &mut Vec<Post>);
    fn get_response(&mut self, question: &str) -> String;
}

pub struct PostFun<P: PostIO, U: UserIO> {
    repo: Box<P>,
    user_io: Box<U>,
}

impl<P: PostIO, U: UserIO> PostFun<P, U> {
    pub fn new(repo: Box<P>, user_io: Box<U>) -> Self {
        PostFun { repo, user_io }
    }
}

impl<P: PostIO, U: UserIO> PostOptions for PostFun<P, U> {
    fn show_posts(&mut self) {
        println!("\nPOSTS:");
        println!("======\n");
        self.repo
            .get_all()
            .iter()
            .for_each(|post| println!("{post}"))
    }

    fn write_post(&mut self) {
        let questions = ["What's the title?", "What's the body?"];

        let mut answers: Vec<String> = vec![];

        questions.iter().for_each(|question| {
            let input = self.user_io.get_response(question);
            answers.push(input);
        });

        let posts = self.repo.get_all();
        let new_id = if let Some(post) = posts.last() {
            post.id + 1
        } else {
            1
        };

        let post = Post::new(
            new_id,
            mem::take(&mut answers[0]),
            mem::take(&mut answers[1]),
        );

        self.repo.add(post);
    }

    fn edit_post(&mut self) {
        let input = self.user_io.get_response("Which post do you want to edit?");

        let id: usize = match input.trim().parse() {
            Ok(id) => id,
            Err(_) => {
                eprintln!("Error parsing post id");
                return;
            }
        };

        let message = format!("{}\n{}\n{}\n", "What do you want to edit?", "title", "body",);

        let input = self.user_io.get_response(&message);

        if input.as_str() == "title" {
            let input = self.user_io.get_response("Enter the new title");
            self.repo.edit(id, &input, "");
        } else if input.as_str() == "body" {
            let input = self.user_io.get_response("Enter the new body");
            self.repo.edit(id, "", &input);
        }
    }

    fn remove_post(&mut self) {
        let input = self
            .user_io
            .get_response("Which post do you want to remove?");

        let id: usize = match input.trim().parse() {
            Ok(id) => id,
            Err(_) => {
                eprintln!("Error parsing post id");
                return;
            }
        };

        self.repo.remove(id);
    }

    fn get_all(&self) -> &Vec<Post> {
        self.repo.get_all()
    }

    fn add_all(&mut self, posts: &mut Vec<Post>) {
        self.repo.add_all(posts);
    }

    fn get_response(&mut self, question: &str) -> String {
        self.user_io.get_response(question)
    }
}
