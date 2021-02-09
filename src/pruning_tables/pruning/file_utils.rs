use std::fs::File;
use std::io::{Read, Write};
use std::{mem, slice};

fn u32_as_u8_slice(v: &[u32]) -> &[u8] {
    let element_size = mem::size_of::<u32>();
    unsafe { slice::from_raw_parts(v.as_ptr() as *const u8, v.len() * element_size) }
}

fn u32_from_u8(v: Vec<u8>) -> Vec<u32> {
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
    f.write_all(u32_as_u8_slice(v)).unwrap();
}

pub fn read_u32_vec(filename: &str) -> Vec<u32> {
    let mut f: File = File::open(filename).unwrap();
    let mut bytes: Vec<u8> = Vec::new();

    f.read_to_end(&mut bytes).unwrap();

    return u32_from_u8(bytes)
}

fn i32_as_u8_slice(v: &[i32]) -> &[u8] {
    let element_size = mem::size_of::<i32>();
    unsafe { slice::from_raw_parts(v.as_ptr() as *const u8, v.len() * element_size) }
}

fn i32_from_u8(v: Vec<u8>) -> Vec<i32> {
    let data = v.as_ptr();
    let len = v.len();
    let capacity = v.capacity();
    let element_size = mem::size_of::<i32>();

    // Make sure we have a proper amount of capacity (may be overkill)
    assert_eq!(capacity % element_size, 0);
    // Make sure we are going to read a full chunk of stuff
    assert_eq!(len % element_size, 0);

    unsafe {
        // Don't allow the current vector to be dropped
        // (which would invalidate the memory)
        mem::forget(v);

        Vec::from_raw_parts(
            data as *mut i32,
            len / element_size,
            capacity / element_size,
        )
    }
}

pub fn write_i32_vec(filename: &str, v: &[i32]) {
    let mut f: File = File::create(filename).unwrap();
    f.write_all(i32_as_u8_slice(v)).unwrap();
}

pub fn read_i32_vec(filename: &str) -> Vec<i32> {
    let mut f: File = File::open(filename).unwrap();
    let mut bytes: Vec<u8> = Vec::new();

    f.read_to_end(&mut bytes).unwrap();

    return i32_from_u8(bytes)
}
