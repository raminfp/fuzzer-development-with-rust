use rand::{thread_rng, Rng};
use std::collections::HashSet;

fn function(input: &str) -> i32 {

    let mut x = 0;

    if input.starts_with("a") { // block 1
        x += 1;
    } else {
        x -= 1;
    }

    if input.ends_with("z") { // block 2
        x += 2;
    } else {
        x -= 2;
    }

    if input.contains("hello") { // block 3
        x += 3;
    }

    x // return

}

struct BlockCoverageFuzzer {
    coverage: HashSet<usize>,
}

impl BlockCoverageFuzzer {

    fn new() -> Self {
        Self {
            coverage: HashSet::new(),
        }
    }

    fn fuzz(&mut self) {

        let mut rng = thread_rng();

        loop {

            let mut input = String::new();
            for _ in 0..20 {
                input.push(rng.gen());
            }

            let blocks = self.analyze(&input);
            self.coverage.extend(blocks);

            if self.coverage.len() == 3 {
                break;
            }

        }

    }

    fn analyze(&self, input: &str) -> Vec<usize> {

        let mut covered = vec![];

        let x = function(input);

        if x != 0 {
            covered.push(1); // block 1
        }

        if x.abs() == 2 {
            covered.push(2); // block 2
        }

        if x >= 3 {
            covered.push(3); // block 3
        }

        covered

    }

}

fn main() {

    let mut fuzzer = BlockCoverageFuzzer::new();

    println!("Starting fuzzing...");

    fuzzer.fuzz();

    println!("Total blocks covered: {}", fuzzer.coverage.len());
    println!("Coverage: {:?}", fuzzer.coverage);

    if fuzzer.coverage.len() == 3 {
        println!("Achieved full block coverage!");
    } else {
        println!("Did not achieve full coverage");
    }

}