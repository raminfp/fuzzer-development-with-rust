use rand::Rng;
use std::collections::HashSet;
use std::convert::TryInto;
use std::fs;
use std::io::Read;
use rand::prelude::SliceRandom;
use std::hash::Hash;
use std::hash::Hasher;

struct Corpus {
    inputs: Vec<Vec<u8>>,
}



impl Corpus {

    fn new() -> Self {
        Corpus { inputs: vec![] }
    }

    fn add(&mut self, input: Vec<u8>) {
        self.inputs.push(input);
    }

    fn get_random(&self, rng: &mut rand::prelude::ThreadRng) -> Option<Vec<u8>> {
        if self.inputs.is_empty() {
            None
        } else {
            Some(self.inputs.choose(rng).unwrap().clone())
        }
    }

}

struct Coverage {
    cov_set: HashSet<u64>,
}

impl Coverage {

    fn new() -> Self {
        Coverage {
            cov_set: HashSet::new()
        }
    }

    fn add(&mut self, input: &[u8]) -> bool {
        let hash = hash(input);
        if !self.cov_set.contains(&hash) {
            self.cov_set.insert(hash);
            true
        } else {
            false
        }
    }

}

struct Fuzzer {
    corpus: Corpus,
    coverage: Coverage,
    cycles: u32,
}

impl Fuzzer {

    fn new() -> Self {
        Fuzzer {
            corpus: Corpus::new(),
            coverage: Coverage::new(),
            cycles: 0,
        }
    }

    fn fuzz(&mut self) {

        let mut rng = rand::thread_rng();

        loop {

            self.cycles += 1;

            if self.cycles % 100 == 0 {

                println!("Fuzzer cycles: {}", self.cycles);

                println!("Corpus size: {}", self.corpus.inputs.len());

                println!("Unique coverages: {}", self.coverage.cov_set.len());

            }

            // Pick an input from corpus
            let mut input = self.corpus.get_random(&mut rng);

            // No seeds, generate a random input
            if input.is_none() {
                input = Some(random_input(&mut rng));
            }

            // Fuzz the target
            if let Some(mut input) = input {
                input = self.mutate_input(input);
                let output = self.run_target(&input);

                // Update corpus and coverage
                let is_new = self.coverage.add(&output);
                if is_new {
                    self.corpus.add(input);
                }
            }

        }

    }

    fn mutate_input(&self, mut input: Vec<u8>) -> Vec<u8> {

        // Insert random bytes
        let insert_idx = rand::thread_rng().gen_range(0..input.len());
        let insert_bytes: Vec<u8> = rand::thread_rng()
            .gen::<[u8; 16]>()
            .to_vec();
        input.splice(insert_idx..insert_idx, insert_bytes);

        // Delete bytes
        let delete_idx = rand::thread_rng().gen_range(0..input.len());
        let delete_len = rand::thread_rng().gen_range(1..10);
        let end = delete_idx + delete_len;
        if end <= input.len() {
            input.drain(delete_idx..end);
        }

        input
    }

    fn run_target(&self, input: &[u8]) -> Vec<u8> {

        // Run target...

        let mut output = Vec::new();

        // Return dummy output
        output.extend_from_slice(&input);

        output
    }

}

// Generate random input
fn random_input(rng: &mut rand::prelude::ThreadRng) -> Vec<u8> {
    let mut input = Vec::new();
    input.extend_from_slice(&rng.gen::<[u8; 30]>());
    input
}

// Hash function
fn hash(buf: &[u8]) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    let mut hasher = DefaultHasher::new();
    buf.hash(&mut hasher);
    hasher.finish()
}

fn main() {
    println!("Starting fuzzing...");
    let mut fuzzer = Fuzzer::new();
    fuzzer.fuzz();

}