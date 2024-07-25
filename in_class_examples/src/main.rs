use std::fs::{OpenOptions,File};
use std::io::Write;

fn append_to_file() {
    let mut file = OpenOptions::new()
        .append(true)
        .open("example.txt")
        .unwrap();

    writeln!(file, "This line is appended to the file.").unwrap();
}

fn main() {
    append_to_file();
    println!("Successfully appended to the file.");
}