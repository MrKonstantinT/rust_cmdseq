use std::process::Command;
use cmdseq::CmdSeq;
use string_utils::{build_command, collect_between_white};
use cookie_file::CookieFile;

mod cmdseq;
mod string_utils;
mod cookie_file;

fn process_cookie(dir: &str, u_s: &str) -> (Vec<CmdSeq>, usize) {
    let cookie = CookieFile::new(dir, u_s);
    let index = cookie.read_cookie();
    let (cmd_list, number_of_operations) = cmdseq::get_command_list(u_s);
    if index + 1 >= number_of_operations {
        cookie.update_cookie(0); // Perform 'wrap round'
    } else {
        cookie.update_cookie(index + 1);
    }
    (cmd_list, index)
}

fn pick_command_and_execute(cmd_list: &Vec<CmdSeq>, index: usize) {
    let mut accumulator: usize = 0;
    for cmdseq in cmd_list {
        accumulator += cmdseq.get_times_before_next();
        if index < accumulator {
            let mut word_iter = cmdseq.get_cmd().split_whitespace();
            let mut program = Command::new(match word_iter.next() {
                Some(string) => string,
                None => {
                    println!("Usage: cmdseq [-d <count dir>] <count1> <cmd1> [... <countn> \
                              <cmdn>]");
                    return; // terminate program.
                }
            });
            for argument in word_iter {
                program.arg(argument);
            }
            print!("{}",
                   String::from_utf8_lossy(&program.output().unwrap().stdout));
            break;
        }
    }
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
    if user_command.starts_with("-d ") {
        directory = collect_between_white(&user_command, 1, 2);
        user_command = collect_between_white(&user_command, 2, 0); // Strip the this '-d' flag.
    }
    println!("{}", &user_command); // Show the user what the program will actually work with.
    let (cmd_list, index) = process_cookie(&directory, &user_command);
    pick_command_and_execute(&cmd_list, index);
}
