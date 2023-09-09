use rand::{thread_rng, Rng};
use std::collections::HashSet;

fn function(input: &str) -> i32 {

    let mut result = 0;

    if input.len() > 10 {
        result += 1;
    }

    if input.contains('a') {
        result += 2;
    } else {
        result -= 1;
    }

    result

}

struct BranchCoverageFuzzer {
    coverage: HashSet<usize>,
}

impl BranchCoverageFuzzer {

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

            let branches = self.analyze(&input);
            self.coverage.extend(branches);

            if self.coverage.len() == 3 {
                break;
            }

        }
    }

    fn analyze(&self, input: &str) -> Vec<usize> {

        let mut covered = vec![];

        let result = function(input);

        if result > 0 {
            covered.push(1);
        }

        if result == 2 {
            covered.push(2);
        } else if result < 0 {
            covered.push(3);
        }

        covered

    }

}

fn main() {

    let mut fuzzer = BranchCoverageFuzzer::new();

    fuzzer.fuzz();

    println!("Total branches covered: {}", fuzzer.coverage.len());
    println!("Coverage: {:?}", fuzzer.coverage);

}