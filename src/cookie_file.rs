// Module cookie_file
use ::std::fs::{ File, OpenOptions };
use ::std::error::Error;
use ::std::process::{ Command, Stdio };
use ::std::io::prelude::*;

pub struct CookieFile {
    cookie_path: String,
}

impl CookieFile {
    pub fn new(directory: &str, command: &str) -> CookieFile {
        let mut c_p = String::with_capacity(directory.len() + 24);
        let mut cookie = CookieFile { cookie_path: "Nil".to_string() };
        c_p = c_p + directory + "/cookie." + &cookie.get_hash_extention(command);
        cookie.cookie_path = c_p;
        cookie.initialse_cookie();
        cookie
    }

    fn initialse_cookie(&self) {
        // Below we open the file but if it doesn't exist we create it automatically with read
        // and write access.
        let mut file = match OpenOptions::new().read(true).write(true).create(true).open(&self.cookie_path) {
            Ok(file) => file,
            Err(why) => panic!("Failed to open file: {}\nReason: {}", &self.cookie_path, why.description()),
        };
        let mut file_data = String::new();
        file.read_to_string(&mut file_data).expect("Failed to read from our cookie.");
        if file_data == "" { // If the file was newly created it initialse it.
            file.write(b"0\n").expect("Failed to initialse file data.");
        } // Leave it untouched if find out its not empty.
        // Files are automatically closed when they go out of scope. Hooray!
    }

    fn get_hash_extention(&self, to_hash: &str) -> String {
        let hash_program = Command::new("sha256sum").stdin(Stdio::piped()).stdout(Stdio::piped()).spawn().expect("Failed to spawn sha256sum");
        hash_program.stdin.unwrap().write_all(to_hash.as_bytes()).expect("Failed to input into sha256sum");
        let mut hash_raw = String::new();
        hash_program.stdout.unwrap().read_to_string(&mut hash_raw).expect("Failed to read output from sha256sum");
        hash_raw.chars().take(16).collect()
    }

    pub fn read_cookie(&self) -> usize {
        let mut cookie = File::open(&self.cookie_path).expect("Could not open our cookie to read from it.");
        let mut file_data = String::new();
        cookie.read_to_string(&mut file_data).expect("Failed to read from our cookie.");
        file_data.trim_right().parse().expect("Something went wrong with parsing the cookie contents.")
    }

    pub fn update_cookie(&self, content: usize) {
        // Using create than open because it will truncate it for us and we won't have to worry
        // about our buffer being too small.
        let mut cookie = File::create(&self.cookie_path).expect("Could not open or truncate our cookie.");
        let string = content.to_string() + "\n";
        cookie.write(string.as_bytes()).expect("Failed to update the cookie.");
    }
}
