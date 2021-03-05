use std::fs::File;
use std::io::{Read, Write};
use std::env;
use std::fs;
use std::{mem, slice};


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

fn as_u8_slice(v: &[u32]) -> &[u8] {
    let element_size = mem::size_of::<u32>();
    unsafe { slice::from_raw_parts(v.as_ptr() as *const u8, v.len() * element_size) }
}

fn from_u8(v: Vec<u8>) -> Vec<u32> {
    let data = v.as_ptr();
    let len = v.len();
    let capacity = v.capacity();
    let element_size = mem::size_of::<u32>();

    // Make sure we have a proper amount of capacity (may be overkill)
    assert_eq!(capacity % element_size, 0);
    // Make sure we are going to read a full chunk of stuff
    assert_eq!(len % element_size, 0);

    unsafe {
        // Don't allow the current vector to be dropped
        // (which would invalidate the memory)
        mem::forget(v);

        Vec::from_raw_parts(
            data as *mut u32,
            len / element_size,
            capacity / element_size,
        )
    }
}

pub fn write_u32_vec(filename: &str, v: &[u32]) {
    let mut f: File = File::create(filename).unwrap();
    f.write_all(as_u8_slice(v)).unwrap();
}

pub fn read_u32_vec(filename: &str) -> Vec<u32> {
    let mut f: File = File::open(filename).unwrap();
    let mut bytes: Vec<u8> = Vec::new();

    f.read_to_end(&mut bytes).unwrap();

    return from_u8(bytes)
}