fn collect_between_white(string: &str, start_white: usize, end_white: usize) -> String {
    let mut collecting = false;
    let mut ignoring_white = false;
    let mut count_white = 0;
    let mut collection = String::new();
    for c in string.chars() {
        if c == '"' {
            ignoring_white = !ignoring_white;
            if collecting { collection.push(c); } // Quotes are kept when we are collecting
        } else if !ignoring_white && c == ' ' {
            count_white = count_white + 1;
            if !collecting && (count_white == start_white || (count_white == 0 && start_white == 0)) { collecting = !collecting; }
            else if count_white == end_white { return collection; }
            else { collection.push(c); } // Don't skip spaces between start and end whites.
        } else if collecting { collection.push(c); }
    }
    return collection;
}

fn build_command(arguments: std::iter::Skip<std::env::Args>) -> String {
    let mut command = String::new();
    for argument in arguments {
        command = if argument.contains(' ') { command + &"\"".to_string() + &argument + &"\" ".to_string() }
        else { command + &argument + &" ".to_string() };
    }
    String::from(command.trim_right())
}

fn load_cookie(directory: &str) { // Will return file handle in future.
    println!("File path: {}", directory);
}



fn main() {
    // Skip the first argument. It tells us the path to this executable.
    let passed_arguments = std::env::args().skip(1);
    if passed_arguments.len() <= 0 || passed_arguments.len() % 2 != 0 {
        println!("Usage: cmdseq [-d <count dir>] <count1> <cmd1> [... <countn> <cmdn>]");
        return; // terminate program.
    }
    let mut user_command = build_command(passed_arguments);
    println!("Built string: {}", user_command);
    if user_command.starts_with("-d ") {
        load_cookie(&collect_between_white(&user_command, 1, 2));
        user_command = collect_between_white(&user_command, 2, 0);
    } else { load_cookie("/tmp"); }
    println!("Finished with: {}", user_command);
}
