use rand::Rng;
use std::fs;
use std::io::Write;


fn main() {
    let mut rng = rand::thread_rng();
    let corpus_dir = "corpus";
    fs::create_dir_all(corpus_dir).unwrap();

    for i in 1..100 {
        let mut content = Vec::new();
        // Generate random length content
        let length = rng.gen_range(0..1000);
        for _ in 0..length {
            content.push(rng.gen::<u8>());
        }
        // Write content to a file
        let filename = format!("{}/{}", corpus_dir, i);
        let mut file = fs::File::create(&filename).unwrap();
        file.write_all(&content).unwrap();
    }
    println!("Generated corpus files in {}", corpus_dir);
}
