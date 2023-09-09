use rand::Rng;
use std::{
    fs::{self, File},
    io::{Write, Read},
    path::{Path, PathBuf},
};

struct Fuzzer {
    corpus: Vec<PathBuf>,
}

impl Fuzzer {
    fn new(input_dir: &str) -> Fuzzer {
        let mut fuzzer = Fuzzer {
            corpus: Vec::new(),
        };
        fuzzer.populate_corpus(input_dir);
        fuzzer
    }

    fn populate_corpus(&mut self, input_dir: &str) {
        // Walk directory to add seed files
        let paths = fs::read_dir(input_dir).unwrap();
        for path in paths {
            self.corpus.push(path.unwrap().path());
        }
    }

    fn fuzz(&mut self) -> Vec<u8> {
        let seed = self.pick_seed();
        let mut buffer = self.mutate_seed(&seed);

        let new_file = self.write_seed(&buffer);
        self.corpus.push(new_file);

        buffer
    }

    fn mutate_seed(&self, seed: &Path) -> Vec<u8> {
        let mut buffer = self.read_seed(seed);
        self.mutate(&mut buffer);
        buffer
    }

    fn pick_seed(&mut self) -> PathBuf {
        let index = rand::thread_rng().gen_range(0..self.corpus.len());
        self.corpus.swap_remove(index)
    }
    fn read_seed(&self, path: &Path) -> Vec<u8> {
        let mut file = File::open(path).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        buffer
    }

    fn mutate(&self, buffer: &mut Vec<u8>) {
        let offset = rand::thread_rng().gen_range(0..buffer.len());
        let val = rand::thread_rng().gen::<u8>();
        buffer[offset] = val;
    }

    fn write_seed(&self, buffer: &[u8]) -> PathBuf {
        let mut path = PathBuf::new();
        path.push("corpus");
        path.push(format!("{}.bin", rand::random::<u32>()));
        let mut file = File::create(&path).unwrap();
        file.write_all(buffer).unwrap();
        path
    }
}

fn main() {
    let mut fuzzer = Fuzzer::new("seeds");

    // Add initial seed
    let seed = b"Hello";
    let path = fuzzer.write_seed(seed);
    fuzzer.corpus.push(path);

    // Fuzz
    for _ in 0..100 {
        let input = fuzzer.fuzz();
        // Test input...

        println!("Fuzzed input: {:?}", input);
    }
}
