extern crate getopts;

use std::env;
use getopts::Options;
use std::process::Command;
use cmdseq::CmdSeq;
use string_utils::{build_command, collect_between_white};
use cookie_file::CookieFile;

mod cmdseq;
mod string_utils;
mod cookie_file;

fn print_usage() {
    println!("Usage: cmdseq [-d <count dir>] <count1> <cmd1> [... <countn> <cmdn>]");
}

fn process_cookie(dir: &str, command_pairs: &str, args: &Vec<String>) -> (Vec<CmdSeq>, usize) {
    let cookie = CookieFile::new(dir, command_pairs);
    let index = cookie.read_cookie();
    let (cmd_list, number_of_operations) = cmdseq::get_command_list(args);
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
                    print_usage();
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
    let passed_arguments: Vec<String> = env::args().skip(1).collect();
    let mut opts = Options::new();
    opts.optopt("d", "", "Sets the directory for the cookie file.", "DIR");
    opts.optflag("h", "", "Prints this usage message.");
    let option_matches = match opts.parse(&passed_arguments) {
        Ok(o_match) => o_match,
        Err(_) => panic!("Failed to parse options."),
    };
    if option_matches.opt_present("h") {
        // Print usage for help flag.
        print_usage();
        return;
    }
    // Select directory for our cookies.
    let cookie_directory = match option_matches.opt_str("d") {
        Some(user_dir) => String::from(user_dir),
        None => String::from("/tmp"),
    };
    // Ensure we have (num command) pairs.
    if option_matches.free.len() % 2 != 0 {
        print_usage();
        return;
    }
    let free_args = option_matches.free.clone();
    let command_pairs: String = free_args.join(" ");
    println!("{}", command_pairs); // Show the user what the program will actually work with.
    let (cmd_list, index) = process_cookie(&cookie_directory, &command_pairs, &free_args);
    pick_command_and_execute(&cmd_list, index);
}
