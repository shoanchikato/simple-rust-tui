use simple_rust_tui::fun::advanced::{clear_screen, show_options};
use simple_rust_tui::fun::post::{edit_post, remove_post, show_posts, write_post};
use simple_rust_tui::repo::post::PostRepo;
use simple_rust_tui::store::advanced::{on_end, on_load};

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
