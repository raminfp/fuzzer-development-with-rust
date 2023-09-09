use std::fs;
use std::io::Write;

fn main() {

    let corpus_dir = "hybrid_corpus";
    fs::create_dir_all(corpus_dir).unwrap();

    // Valid samples
    let valid_png = include_bytes!("valid.png");
    fs::write(format!("{}/valid.png", corpus_dir), valid_png).unwrap();

    let valid_jpg = include_bytes!("valid.jpg");
    fs::write(format!("{}/valid.jpg", corpus_dir), valid_jpg).unwrap();

    // Invalid samples
    let invalid_png = corrupt_png(valid_png);
    fs::write(format!("{}/invalid.png", corpus_dir), invalid_png).unwrap();

    let truncated_jpg = truncate_sample(valid_jpg, 100);
    fs::write(format!("{}/truncated.jpg", corpus_dir), truncated_jpg).unwrap();

    println!("Created hybrid corpus in {}", corpus_dir);
}

// Corrupt valid PNG
fn corrupt_png(png: &[u8]) -> Vec<u8> {
    let mut corrupted = png.to_vec();
    corrupted[5] = 0; // corrupt signature
    corrupted
}

// Truncate sample 
fn truncate_sample(data: &[u8], len: usize) -> Vec<u8> {
    data[..len].to_vec()
}