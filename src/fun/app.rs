use crate::fun::post_options::PostOptions;
use crate::store::app_io::AppIO;

pub struct App<'a, A> {
    app_io: &'a mut A,
}

impl<'a, A: AppIO + PostOptions> App<'a, A> {
    pub fn new(app_io: &'a mut A) -> Self {
        App { app_io }
    }
}

impl<'a, A: AppIO + PostOptions> App<'a, A>  {
    fn show_options(&mut self) -> u8 {
        let options = format!(
            "{}\n{}\n{}\n{}\n{}\n{}\n{}\n",
            "Choose an option:",
            "1 -> show posts",
            "2 -> add post",
            "3 -> clear",
            "4 -> edit",
            "5 -> remove",
            "0 -> exit",
        );

        let input = self.app_io.get_response(&options);

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
        self.app_io.on_load();

        loop {
            let option = self.show_options();

            match option {
                0 => break,
                1 => self.app_io.show_posts(),
                2 => self.app_io.write_post(),
                3 => self.clear_screen(),
                4 => self.app_io.edit_post(),
                5 => self.app_io.remove_post(),
                _ => continue,
            }
        }
        self.app_io.on_end();
    }
}
