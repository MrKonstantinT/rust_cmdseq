use std::error::Error;
use std::process::Command;
use std::io::prelude::*;
use std::process::Stdio;

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
            ignoring_white = !ignoring_white; // Do not allow spaces between quotes to affect the logic of this function.
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
        command = if argument.contains(' ') { command + "\"" + &argument + "\" " }
        else { command + &argument + &" ".to_string() };
    }
    String::from(command.trim_right())
}

fn load_cookie(directory: &str, to_hash: &str) -> std::fs::File {
    // Select hash program here in the future.
    let hash_program = Command::new("sha256sum").stdin(Stdio::piped()).stdout(Stdio::piped()).spawn().expect("Failed to spawn sha256sum");
    hash_program.stdin.unwrap().write_all(to_hash.as_bytes()).expect("Failed to input into sha256sum");
    let mut hash_raw = String::new();
    hash_program.stdout.unwrap().read_to_string(&mut hash_raw).expect("Failed to read output from sha256sum");
    let hash_extention: String = hash_raw.chars().take(16).collect();
    std::fs::create_dir_all(directory).expect("Failed to create directory(ies)");
    let mut file_path = String::new();
    file_path = file_path + directory + "/cookie." + &hash_extention;
    let mut file = match std::fs::OpenOptions::new().read(true).write(true).create(true).open(&file_path) {
        Ok(file) => file,
        Err(why) => panic!("Failed to open file: {}\nReason: {}", file_path, why.description()),
    };
    let mut file_data = String::new();
    file.read_to_string(&mut file_data).expect("Failed to read from our cookie.");
    if file_data == "" {
        file.write(b"0\n").expect("Failed to initialse file data.");
        file.seek(std::io::SeekFrom::End(-2)).expect("Failed to make the file object read from the beginning.");
    } else { file.seek(std::io::SeekFrom::End(-1 * file_data.len() as i64)).expect("Failed to make the file object read from the beginning."); }
    file
}

fn get_command_list(command: &str) -> (Vec<CmdSeq>, usize) {
    let mut command_list: Vec<CmdSeq> = Vec::new();
    // The number of operations we would execute before we need to start from beginning.
    let mut num_operations: usize = 0;
    let mut num = 0;
    while num <= count_white_space(command) { // Using 'while' loop as 'step_by()' has issues atm.
        let t_b_n: usize = collect_between_white(command, num, num + 1).parse().expect("Usage: cmdseq [-d <count dir>] <count1> <cmd1> [... <countn> <cmdn>]");
        num_operations += t_b_n;
        command_list.push(CmdSeq {
            times_before_next: t_b_n,
            cmd: collect_between_white(command, num + 1, num + 2).replace("\"", ""),
        });
        num = num + 2;
    }
    (command_list, num_operations)
}

fn main() {
    // Skip the first argument. It tells us the path to this executable.
    let passed_arguments = std::env::args().skip(1);
    if passed_arguments.len() <= 0 || passed_arguments.len() % 2 != 0 {
        println!("Usage: cmdseq [-d <count dir>] <count1> <cmd1> [... <countn> <cmdn>]");
        return; // terminate program.
    }
    let mut user_command = build_command(passed_arguments);
    let mut directory = String::from("/tmp");
    let mut file_data = String::new();
    if user_command.starts_with("-d ") {
        directory = collect_between_white(&user_command, 1, 2);
        user_command = collect_between_white(&user_command, 2, 0); // Strip the this '-d' flag.
    }
    println!("{}", &user_command); // Show the user what the program will actually work with.
    let file = &mut load_cookie(&directory, &user_command);
    file.read_to_string(&mut file_data).expect("Failed to read from our cookie.");
    file.seek(std::io::SeekFrom::End(-1 * file_data.len() as i64)).expect("Failed to make the file object read from the beginning.");
    let index: usize = file_data.trim_right().parse().expect("Something went wrong with parsing the cookie contents.");
    let (cmd_list, number_of_operations) = get_command_list(&user_command);
    if index + 1 >= number_of_operations { // Perform 'wrap round':
        file.set_len(0).expect("Failed to shirnk file to size 0.");
        file.write(b"0\n").expect("Failed to update cookie.");
    } else {
        let next_index = index + 1;
        let string = next_index.to_string() + "\n";
        file.write(string.as_bytes()).expect("Failed to update cookie.");
    };
    let mut accumulator: usize = 0;
    for cmdseq in cmd_list {
        accumulator += cmdseq.times_before_next;
        if index < accumulator {
            let mut word_iter = cmdseq.cmd.split_whitespace();
            let mut program = Command::new(match word_iter.next() {
                Some(string) => string,
                None => {
                    println!("Usage: cmdseq [-d <count dir>] <count1> <cmd1> [... <countn> <cmdn>]");
                    return; // terminate program.
                },
            });
            for argument in word_iter {
                program.arg(argument);
            }
            print!("{}", String::from_utf8_lossy(&program.output().unwrap().stdout));
            break;
        }
    }
}
