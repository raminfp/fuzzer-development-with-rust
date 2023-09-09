use rand::{thread_rng, Rng};
use std::collections::HashSet;

fn function(input: &str) -> bool {

    let pred1 = input.len() > 10;
    let pred2 = input.contains("foo");
    let pred3 = input.starts_with("bar");

    pred1 || pred2 || pred3

}

struct PredicateCoverageFuzzer {
    coverage: HashSet<usize>,
}

impl PredicateCoverageFuzzer {

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

            let predicates = self.analyze(&input);
            self.coverage.extend(predicates);

            if self.coverage.len() == 3 {
                break;
            }

        }

    }

    fn analyze(&self, input: &str) -> Vec<usize> {

        let mut covered = vec![];

        let res = function(input);

        let pred1 = input.len() > 10;
        if pred1 {
            covered.push(1);
        }

        let pred2 = input.contains("foo");
        if pred2 {
            covered.push(2);
        }

        let pred3 = input.starts_with("bar");
        if pred3 {
            covered.push(3);
        }

        covered

    }

}


fn main() {

    let mut fuzzer = PredicateCoverageFuzzer::new();

    println!("Starting fuzzing...");

    fuzzer.fuzz();

    println!("Total predicates covered: {}", fuzzer.coverage.len());
    println!("Coverage: {:?}", fuzzer.coverage);

    if fuzzer.coverage.len() == 3 {
        println!("Achieved full predicate coverage!");
    } else {
        println!("Did not achieve full coverage");
    }

}