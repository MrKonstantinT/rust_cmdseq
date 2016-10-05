struct CmdSeq {
    times_before_next: usize,
    cmd: String,
}

fn count_white_space(string: &str) -> usize {
    let mut num_white: usize = 0;
    let mut ignoring_white = false;
    for c in string.chars() {
        if !ignoring_white && c == ' ' { num_white = num_white + 1; }
        else if c == '"' { ignoring_white = !ignoring_white; }
    }
    num_white
}

fn collect_between_white(string: &str, start_white: usize, end_white: usize) -> String {
    let mut collecting = if start_white < 1 { true } else { false };
    let mut ignoring_white = false;
    let mut count_white = 0;
    let mut collection = String::new();
    for c in string.chars() {
        if c == '"' {
            ignoring_white = !ignoring_white;
            if collecting { collection.push(c); } // Quotes are kept when we are collecting
        } else if !ignoring_white && c == ' ' {
            count_white = count_white + 1;
            if !collecting && count_white == start_white { collecting = !collecting; }
            else if count_white == end_white { return String::from(collection.trim_left()); } // Rid of extra space in beginning when collecting up until end.
            else { collection.push(c); } // Don't skip spaces between start and end whites.
        } else if collecting { collection.push(c); }
    }
    String::from(collection.trim_left())
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

fn get_command_list(command: &str) -> Vec<CmdSeq>{
    let mut command_list: Vec<CmdSeq> = Vec::new();
    let mut num = 0;
    while num <= count_white_space(command) { // Using 'while' loop as 'step_by()' has issues atm.
        command_list.push(CmdSeq {
            times_before_next: collect_between_white(command, num, num + 1).parse().expect("Usage: cmdseq [-d <count dir>] <count1> <cmd1> [... <countn> <cmdn>]"),
            cmd: collect_between_white(command, num + 1, num + 2)
        });
        num = num + 2;
    }
    command_list
}

fn main() {
    // Skip the first argument. It tells us the path to this executable.
    let passed_arguments = std::env::args().skip(1);
    if passed_arguments.len() <= 0 || passed_arguments.len() % 2 != 0 {
        println!("Usage: cmdseq [-d <count dir>] <count1> <cmd1> [... <countn> <cmdn>]");
        return; // terminate program.
    }
    let mut user_command = build_command(passed_arguments);
    if user_command.starts_with("-d ") {
        load_cookie(&collect_between_white(&user_command, 1, 2));
        user_command = collect_between_white(&user_command, 2, 0); // Strip the this '-d' flag.
    } else { load_cookie("/tmp"); }
    for cmdseq in get_command_list(&user_command) {
        println!("CmdSeq\n  times_before_next: {}\n  cmd: {}", cmdseq.times_before_next, cmdseq.cmd);
    }
}
