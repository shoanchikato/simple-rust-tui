use super::base::get_response;

pub fn show_options() -> u8 {
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

    let input = get_response(&options);

    match input.trim().parse() {
        Ok(option) => option,
        Err(_) => {
            eprintln!("Error parsing post id");
            return 3;
        }
    }
}

pub fn clear_screen() {
    print!("\x1B[2J\x1B[H");
}
