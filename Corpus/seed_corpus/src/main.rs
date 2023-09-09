use std::fs;
use std::io::Write;

fn main() {
    let corpus_dir = "seed_corpus";
    fs::create_dir(corpus_dir).unwrap();

    // A valid text sample
    let mut file = fs::File::create(format!("{}/seed1.txt", corpus_dir)).unwrap();
    file.write_all(b"Hello").unwrap();
    // Another valid text sample
    let mut file1 = fs::File::create(format!("{}/seed2.txt", corpus_dir)).unwrap();
    file1.write_all(b"Hello").unwrap();
    // A short binary file
    let mut file3 = fs::File::create(format!("{}/seed3.txt", corpus_dir)).unwrap();
    file3.write_all(&[0x01, 0x02, 0x02]).unwrap();

    println!("Created seed corpus in {}", corpus_dir);

}
