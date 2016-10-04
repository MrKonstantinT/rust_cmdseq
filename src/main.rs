fn main() {
    let mut c = 1;
    // Skip the first argument. It tells us the path to this executable.
    let passed_arguments = std::env::args().skip(1);
    if passed_arguments.len() <= 0 || passed_arguments.len() % 2 != 0 {
        println!("Usage: cmdseq [-d <count dir>] <count1> <cmd1> [... <countn> <cmdn>]");
        return; // terminate program.
    }
    println!("Printing arguments passed:");
    for argument in passed_arguments {
        println!("{}: {}", c, argument);
        c += 1;
    }
}
