use rand::Rng;
use std::collections::HashSet;
use rand::prelude::IteratorRandom;

struct Fuzzer {
    corpus: HashSet<Vec<u8>>
}

impl Fuzzer {
    fn new() -> Fuzzer {
        Fuzzer {
            corpus: HashSet::new(),
        }
    }

    fn fuzz(&mut self) -> Vec<u8> {
        let mut buffer = Vec::new();
        if let Some(seed) = self.corpus.iter().choose(&mut rand::thread_rng()) {
            buffer.extend_from_slice(seed);
        }

        for _ in 0..100 {
            let r = rand::thread_rng().gen_range(0..=255);
            buffer.push(r as u8)
        }
        self.corpus.insert(buffer.clone());
        buffer
    }

}

fn main() {
    let mut fuzzer = Fuzzer::new();
    let seed = b"hello";
    fuzzer.corpus.insert(seed.to_vec());
    for _ in 0..100 {
        let input = fuzzer.fuzz();
        println!("{:?}", input);
    }
}
