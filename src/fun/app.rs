use crate::fun::post_options::PostOptions;
use crate::store::app_io::AppIO;

pub struct App<'a, P: PostOptions, A: AppIO> {
    post_rw: &'a mut P,
    app_io: &'a mut A,
}

impl<'a, P: PostOptions, A: AppIO> App<'a, P, A> {
    pub fn new(post_rw: &'a mut P, app_io: &'a mut A) -> Self {
        App { post_rw, app_io }
    }
}

impl<'a, P: PostOptions, A: AppIO> App<'a, P, A> {
    fn show_options(&mut self) -> u8 {
        let options = format!(
            "\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n",
            "Choose an option:",
            "1 -> show posts",
            "2 -> add post",
            "3 -> clear",
            "4 -> edit",
            "5 -> remove",
            "0 -> exit",
        );

        let input = self.post_rw.get_response(&options);

        match input.trim().parse() {
            Ok(option) => option,
            Err(_) => {
                eprintln!("Error parsing post id");
                3
            }
        }
    }

    fn clear_screen(&self) {
        print!("\x1B[2J\x1B[H");
    }

    pub fn run(&mut self) {
        self.app_io.on_load(self.post_rw);

        loop {
            let option = self.show_options();

            match option {
                0 => break,
                1 => self.post_rw.show_posts(),
                2 => self.post_rw.write_post(),
                3 => self.clear_screen(),
                4 => self.post_rw.edit_post(),
                5 => self.post_rw.remove_post(),
                _ => continue,
            }
        }
        self.app_io.on_end(self.post_rw);
    }
}
