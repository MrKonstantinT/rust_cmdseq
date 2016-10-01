fn main() {
    let mut c = 1;
    println!("Printing arguments passed:");
    // Skip the first argument. It tells us the path to this executable.
    for argument in std::env::args().skip(1) {
        println!("{}: {}", c, argument);
        c += 1;
    }
}
