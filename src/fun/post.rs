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

    let post = Post::new(
        repo.get_all().len() + 1,
        answers[0].clone(),
        answers[1].clone(),
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

    let id = id - 1;

    let mut post = match repo.get_one(id) {
        Some(post) => post,
        None => {
            eprintln!("Post with {} id, not found", id);
            return;
        }
    };

    let message = format!("{}\n{}\n{}\n", "What do you want to edit?", "title", "body",);

    let input = get_response(&message);

    match input.trim() {
        "title" => edit_title(&mut post),
        "body" => edit_body(&mut post),
        _ => return,
    }
}

fn edit_title(post: &mut Post) {
    let input = get_response("Enter the new title");
    post.title = input;
}

fn edit_body(post: &mut Post) {
    let input = get_response("Enter the new body");
    post.body = input;
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

    let id = id - 1;

    repo.remove(id);
}
