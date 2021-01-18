use clap::{Arg, App};

#[derive(Debug, PartialEq)]
pub struct Config {
    pub input: String,
    pub visualisator: bool
}

impl Config {
    pub fn new() -> Self {
        let matches = App::new("rubik")
            .version("0.1.0")
            .author("Simon Galasso <simon.galasso@hotmail.fr>")
            .about("Solve a rubik")
            .arg(Arg::with_name("input_sequence")
                .required(true)
                .index(1)
                .help("The sequence to shuffle a rubik"))
            .arg(Arg::with_name("visualisator")
                .required(false)
                .short("v")
                .long("visualisator")
                .takes_value(false)
                .help("enable the visualisator"))
            .get_matches();
        return Config {
            input: matches.value_of("input_sequence").unwrap_or("").to_string(),
            visualisator: matches.is_present("visualisator")
        };
    }
}