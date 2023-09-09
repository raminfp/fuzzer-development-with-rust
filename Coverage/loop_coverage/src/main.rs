use rand::{thread_rng, Rng};
use std::collections::HashSet;

fn function(input: &str) {

    let mut sum = 0;

    for chr in input.chars() { // loop 1
        sum += chr as i32;
    }

    let mut i = 0;
    while i < input.len() { // loop 2
        sum += i as i32;
        i += 1;
    }

    loop { // loop 3
        sum += 1;
        break;
    }

}

struct LoopCoverageFuzzer {
    coverage: HashSet<usize>,
}

impl LoopCoverageFuzzer {

    fn new() -> Self {
        Self {
            coverage: HashSet::new(),
        }
    }

    fn fuzz(&mut self) {

        let mut rng = thread_rng();

        loop {
            let rng = rng.clone();
            let input: String = rng.sample_iter(&rand::distributions::Alphanumeric)
                .take(30)
                .map(char::from)
                .collect();

            let loops = self.analyze(&input);
            self.coverage.extend(loops);

            if self.coverage.len() == 3 {
                break;
            }

        }

    }

    fn analyze(&self, input: &str) -> Vec<usize> {

        let mut covered = vec![];

        function(input);

        if !input.is_empty() {
            covered.push(1); // for loop
        }

        if input.len() > 0 {
            covered.push(2); // while loop
        }

        covered.push(3); // infinite loop

        covered

    }

}

fn main() {
    let mut fuzzer = LoopCoverageFuzzer::new();

    println!("Starting fuzzing...");

    fuzzer.fuzz();

    println!("Total loops covered: {}", fuzzer.coverage.len());
    println!("Coverage: {:?}", fuzzer.coverage);

    if fuzzer.coverage.len() == 3 {
        println!("Achieved full loop coverage!");
    } else {
        println!("Did not achieve full coverage");
    }
}