use std::fs::File;
use std::io::{Read, Write};
use std::env;
use std::fs;

pub fn get_current_path() -> String {
    match env::current_dir() {
        Ok(dir_path) => return dir_path.display().to_string(),
        Err(_e) => return ".".to_string(),
    };
}

pub fn create_dir(path: &str) {
    match fs::create_dir_all(path) {
        Err(e) => panic!(e),
        _ => (),
    }
}

pub fn write_u8_vec(filename: &str, v: &[u8]) {
    let mut f: File = File::create(filename).unwrap();
    f.write_all(v).unwrap();
}

pub fn read_u8_vec(filename: &str) -> Vec<u8> {
    let mut f: File = File::open(filename).unwrap();
    let mut bytes: Vec<u8> = Vec::new();
    f.read_to_end(&mut bytes).unwrap();
    return bytes
}