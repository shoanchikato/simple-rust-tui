use hello_world_one::fun::advanced::{clear_screen, show_options};
use hello_world_one::fun::post::{edit_post, remove_post, show_posts, write_post};
use hello_world_one::repo::post::PostRepo;
use hello_world_one::store::advanced::{on_end, on_load};

fn main() {
    run_app();
}

fn run_app() {
    let mut repo = PostRepo::new();
    on_load(&mut repo);
    loop {
        let option = show_options();

        match option {
            0 => break,
            1 => show_posts(&repo),
            2 => write_post(&mut repo),
            3 => clear_screen(),
            4 => edit_post(&mut repo),
            5 => remove_post(&mut repo),
            _ => continue,
        }
    }
    on_end(&repo);
}
