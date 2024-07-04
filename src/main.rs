use simple_rust_tui::fun::app::App;
use simple_rust_tui::fun::post_options::PostFun;
use simple_rust_tui::fun::string_io::StringRW;
use simple_rust_tui::fun::user_io::UserRW;
use simple_rust_tui::repo::post::PostRW;
use simple_rust_tui::store::app_io::AppRW;
use simple_rust_tui::store::file_io::FileRW;
use std::io::{stdin, stdout, BufReader};

fn main() {
    let stdout = stdout();
    let stdin = stdin();
    let stdin_reader = BufReader::new(stdin.lock());

    let string_io = StringRW::new(Box::new(stdin_reader), Box::new(stdout));
    let user_response = UserRW::new(Box::new(string_io));

    let file_io = FileRW::new();
    let post_io = PostRW::new(vec![]);
    let post_fun = PostFun::new(Box::new(post_io), Box::new(user_response));

    let app_io = AppRW::new(Box::new(file_io), Box::new(post_fun));
    let mut app = App::new(Box::new(app_io));

    app.run();
}
