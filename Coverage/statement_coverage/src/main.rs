use rand::Rng;
use std::collections::HashSet;

fn process(input: &str) -> bool {
    let _a = input.contains('a');
    let _b = input.len() > 10;
    let _c = input.is_empty();

    _a || _b || _c
}

struct StatementCoverageFuzzer {
    coverage: HashSet<usize>,
}

impl StatementCoverageFuzzer {
    fn new() -> Self {
        Self {
            coverage: HashSet::new(),
        }
    }

    fn fuzz(&mut self) {
        let mut rng = rand::thread_rng();

        loop {
            let mut input = String::new();
            for _ in 0..20 {
                input.push(rng.gen());
            }
            let stmts = self.analyze(&input);

            self.coverage.extend(stmts);

            if self.coverage.len() == 3 {
                break;
            }
        }
    }

    fn analyze(&self, input: &str) -> Vec<usize> {
        let mut covered = vec![];

        let a = input.contains('a');
        covered.push(0);

        let b = input.len() > 10;
        covered.push(1);

        let c = input.is_empty();
        covered.push(2);

        covered
    }

}

fn main() {
    let mut fuzzer = StatementCoverageFuzzer::new();
    fuzzer.fuzz();

    println!("Coverage: {:?}", fuzzer.coverage);
}
