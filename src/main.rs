use simple_rust_tui::fun::app::App;
use simple_rust_tui::fun::post_options::PostFun;
use simple_rust_tui::fun::string_io::StringRW;
use simple_rust_tui::fun::user_io::UserRW;
use simple_rust_tui::repo::post::PostRW;
use simple_rust_tui::store::app_io::AppRW;
use simple_rust_tui::store::file_io::FileRW;
use std::io::{stdin, stdout, BufReader};

fn main() {
    let mut stdout = stdout();
    let stdin = stdin();
    let mut stdin_reader = BufReader::new(stdin.lock());

    let mut string_io = StringRW::new(&mut stdin_reader, &mut stdout);
    let mut user_response = UserRW::new(&mut string_io);

    let file_io = FileRW::new();
    let mut post_io = PostRW::new(vec![]);
    let mut post_fun = PostFun::new(&mut post_io, &mut user_response);

    let mut app_io = AppRW::new(&file_io);
    let mut app = App::new(&mut post_fun, &mut app_io);

    app.run();
}
