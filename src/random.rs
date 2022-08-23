use std::fmt::Write;
use std::process;

use getrandom::getrandom;

pub fn generate_random_string() -> String {
    let mut string = String::new();
    let buf = generate_random_buf();
    for num in &buf {
        write!(string, "{:02x}", num).expect("Failed to write hex number");
    }
    string
}

fn generate_random_buf() -> [u8; 4] {
    let mut buf = [0_u8; 4];
    if let Ok(()) = getrandom(&mut buf) {
        buf
    } else {
        let process_bytes = process::id().to_be_bytes();
        buf[..4].clone_from_slice(&process_bytes);
        buf
    }
}
