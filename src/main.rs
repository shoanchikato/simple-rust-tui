use hello_world_one::fun::advanced::{clear_screen, show_options};
use hello_world_one::fun::post::{edit_post, remove_post, show_posts, write_post};

fn main() {
    run_app();
}

fn run_app() {
    let mut posts = vec![];
    loop {
        let option = show_options();

        match option {
            0 => break,
            1 => show_posts(&posts),
            2 => write_post(&mut posts),
            3 => clear_screen(),
            4 => edit_post(&mut posts),
            5 => remove_post(&mut posts),
            _ => continue,
        }
    }
}
