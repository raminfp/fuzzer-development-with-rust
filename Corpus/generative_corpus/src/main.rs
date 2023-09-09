use std::{fs, io};
use rand::Rng;
use rand::rngs::ThreadRng;
use std::fs::File;
use std::io::Write;

fn main() {
    let corpus = generate_corpus();
    store_corpus(&corpus);
}

// Generate corpus samples 
fn generate_corpus() -> Vec<Vec<u8>> {

    let mut corpus = vec![];
    let mut rng = rand::thread_rng();
    // Generate JSON samples
    for i in 0..100 {
        let json = generate_json(&mut rng);
        corpus.push(json);
    }
    // Generate more sample types
    // PNG, CSV, XML, MP4, etc
    corpus
}

// Generate random valid JSON
fn generate_json(rng: &mut ThreadRng) -> Vec<u8> {

    let mut json = vec![];
    // Randomly generate JSON objects
    let num_objects = rng.gen_range(1..10);
    for _ in 0..num_objects {
        json.push(b'{');
        let num_fields = rng.gen_range(1..5);
        for _ in 0..num_fields {
            generate_random_string(rng, &mut json); // key
            json.push(b':');
            generate_random_value(rng, &mut json); // value
            if rng.gen_bool(0.5) {
                json.push(b',');
            }
        }
        json.push(b'}');
    }
    // Add array
    if rng.gen_bool(0.5) {
        json.push(b'[');
        let num_elements = rng.gen_range(1..5);
        for _ in 0..num_elements {
            generate_random_value(rng, &mut json);
            if rng.gen_bool(0.5) {
                json.push(b',');
            }
        }
        json.push(b']');
    }
    json
}

// Generate random strings
fn generate_random_string(rng: &mut ThreadRng, vec: &mut Vec<u8>) {
    // Generate random length and characters
    let len = rng.gen_range(5..20);
    for _ in 0..len {
        vec.push(rng.gen_range(b'a'..b'z'));
    }
}

// Generate random values
fn generate_random_value(rng: &mut ThreadRng, vec: &mut Vec<u8>) {
    // Randomly generate primitive values
    if rng.gen_bool(0.5) {
        vec.extend_from_slice(b"\"string\"");
    } else if rng.gen_bool(0.5) {
        vec.extend_from_slice(b"123");
    } else {
        vec.extend_from_slice(b"true");
    }
}

// Save generated corpus to disk
fn store_corpus(corpus: &[Vec<u8>]) {
    let corpus_dir = "fuzz_corpus";
    create_dir_all(corpus_dir).unwrap();
    for (i, sample) in corpus.iter().enumerate() {
        let file_name = format!("{}/sample_{:03}", corpus_dir, i);
        let mut file = File::create(file_name).unwrap();
        file.write_all(sample).unwrap();
    }
    println!("Stored {} samples in {}", corpus.len(), corpus_dir);
}

fn create_dir_all(path: &str) -> io::Result<()> {
    fs::create_dir_all(path)
}