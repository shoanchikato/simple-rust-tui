use std::mem;

use crate::fun::base::get_response;
use crate::model::post::Post;
use crate::repo::post::PostRepo;

pub fn show_posts(repo: &PostRepo) {
    println!("\nPOSTS:");
    println!("======\n");
    repo.get_all().iter().for_each(|post| println!("{post}"))
}

pub fn write_post(repo: &mut PostRepo) {
    let questions = vec!["What's the title?", "What's the body?"];

    let mut answers: Vec<String> = vec![];

    eprintln!("");
    questions.iter().for_each(|question| {
        let input = get_response(question);
        answers.push(input);
    });

    let posts = repo.get_all();
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

    repo.add(post);
}

pub fn edit_post(repo: &mut PostRepo) {
    let input = get_response("Which post do you want to edit?");

    let id: usize = match input.trim().parse() {
        Ok(id) => id,
        Err(_) => {
            eprintln!("Error parsing post id");
            return;
        }
    };

    let message = format!("{}\n{}\n{}\n", "What do you want to edit?", "title", "body",);

    let input = get_response(&message);

    match input.as_str() {
        "title" => {
            let input = get_response("Enter the new title");
            repo.edit(id, &input, "");
        }
        "body" => {
            let input = get_response("Enter the new body");
            repo.edit(id, "", &input);
        }
        _ => return,
    }
}

pub fn remove_post(repo: &mut PostRepo) {
    let input = get_response("Which post do you want to remove?");

    let id: usize = match input.trim().parse() {
        Ok(id) => id,
        Err(_) => {
            eprintln!("Error parsing post id");
            return;
        }
    };

    repo.remove(id);
}
