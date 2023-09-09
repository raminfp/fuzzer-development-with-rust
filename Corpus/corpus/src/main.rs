use std::fs;
use std::io::Write;

fn generate_corpus() {

    // Create a new directory for the corpus
    fs::create_dir("corpus").unwrap();

    // Generate some valid samples
    let mut f = fs::File::create("corpus/valid_1.txt").unwrap();
    f.write_all(b"This is a valid input").unwrap();

    let mut f = fs::File::create("corpus/valid_2.txt").unwrap();
    f.write_all(b"Another valid sample").unwrap();

    // Generate some invalid/malformed samples
    let mut f = fs::File::create("corpus/invalid_1.txt").unwrap();
    f.write_all(b"Invalid\xFF").unwrap();

    let mut f = fs::File::create("corpus/invalid_2.txt").unwrap();
    f.write_all(b"\x00\x00\x00").unwrap();
}

fn main() {
    generate_corpus();
}