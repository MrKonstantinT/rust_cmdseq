// Module string_utils
pub fn build_command(arguments: ::std::iter::Skip<::std::env::Args>) -> String {
    let mut command = String::new();
    for argument in arguments {
        command = if argument.contains(' ') {
            command + "\"" + &argument + "\" "
        } else {
            command + &argument + &" ".to_string()
        };
    }
    String::from(command.trim_right())
}

pub fn collect_between_white(string: &str, start_white: usize, end_white: usize) -> String {
    let mut collecting = if start_white < 1 { true } else { false };
    let mut ignoring_white = false;
    let mut count_white = 0;
    let mut collection = String::with_capacity(string.len()); // Benchmark new() against this.
    for c in string.chars() {
        if c == '"' {
            // Do not allow spaces between quotes to affect the logic of this function.
            ignoring_white = !ignoring_white;
            if collecting {
                // Quotes are kept when we are collecting
                collection.push(c);
            }
        } else if !ignoring_white && c == ' ' {
            count_white = count_white + 1;
            if !collecting && count_white == start_white {
                collecting = !collecting;
            } else if count_white == end_white {
                // Rid of extra space in beginning when collecting up until end.
                return String::from(collection.trim_left());
            } else {
                // Don't skip spaces between start and end whites.
                collection.push(c);
            }
        } else if collecting {
            collection.push(c);
        }
    }
    String::from(collection.trim_left())
}

pub fn count_white_space(string: &str) -> usize {
    let mut num_white: usize = 0;
    let mut ignoring_white = false;
    for c in string.chars() {
        if !ignoring_white && c == ' ' {
            num_white = num_white + 1;
        } else if c == '"' {
            ignoring_white = !ignoring_white;
        }
    }
    num_white
}
