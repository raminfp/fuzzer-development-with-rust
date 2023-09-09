use rand::{thread_rng, Rng};
use std::collections::HashSet;

fn function_1(input: &str) -> bool {
    input.len() > 5
}

fn function_2(input: &str) -> i32 {
    if input.contains("a") {
        1
    } else {
        -1
    }
}

struct FunctionCoverageFuzzer {
    coverage: HashSet<usize>,
}

impl FunctionCoverageFuzzer {

    fn new() -> Self {
        Self {
            coverage: HashSet::new(),
        }
    }

    fn fuzz(&mut self) {
        let mut rng = thread_rng();

        loop {
            let rng = rng.clone(); // clone rng here
            let input: String = rng.sample_iter(&rand::distributions::Alphanumeric)
                .take(30)
                .map(char::from)
                .collect();

            if function_1(&input) {
                self.coverage.insert(1);
            }

            let num = function_2(&input);
            if num != 0 {
                self.coverage.insert(2);
            }

            if self.coverage.len() == 2 {
                break;
            }
        }
    }

}


fn main() {
    let mut fuzzer = FunctionCoverageFuzzer::new();
    fuzzer.fuzz();

    println!("Coverage: {:?}", fuzzer.coverage);
}