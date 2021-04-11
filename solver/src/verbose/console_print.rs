use std::io::{self, Write};

pub fn console_print(prefix: &str, text: String, verbose: bool) {
    // if vervose activated and verbose == true
    print!("{}{}", prefix, text);
    io::stdout().flush().expect("error: can't flush stdout");
}