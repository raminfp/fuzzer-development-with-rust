use rand::Rng;
use std::collections::HashSet;

/*
fn process(input: &str) -> i32 {
  if input.is_empty() {
    return -1;
  }

  if input.len() > 10 {
    return 1;
  }

  0
}

Input is empty -> return -1
Input length > 10 -> return 1
All other inputs -> return 0

#[test]
fn test_process() {
  assert_eq!(process(""), -1); // path 1
  assert_eq!(process("hello"), 0); // path 3
  assert_eq!(process("long input"), 1); // path 2
}

*/

#[derive(Debug, PartialEq)]
enum Status {
    Ok,
    Err,
}

fn process(input: &str) -> Status {
    if input.is_empty() { // path 0
        Status::Err
    } else if input.len() > 10 { // Path 1
        Status::Ok
    } else {
        Status::Err // Path 2
    }
}

struct PathCoverageFuzzer {
    coverage: HashSet<usize>,
}

impl PathCoverageFuzzer {

    fn new() -> Self {
        Self {
            coverage: HashSet::new(),
        }
    }

    fn fuzz(&mut self) {
        let mut rng = rand::thread_rng();

        loop {
            let input = match rng.gen_range(0..4) {
                0 => "".to_string(), // empty string
                1 => { // long string
                    let mut s = String::new();
                    for _ in 0..20 {
                        s.push(rng.gen::<char>());
                    }
                    s
                }
                _ => "short".to_string(), // short string
            };
            let path = self.analyze(&input);
            println!("Fuzzing: input = {}, path = {}", input, path);

            self.coverage.insert(path);

            if self.coverage.len() == 3 {
                break;
            }
        }
    }

    fn analyze(&self, input: &str) -> usize {
        match process(input) {
            Status::Err if input.is_empty() => 0,
            Status::Ok if input.len() > 10 => 1,
            // Status::Ok if input.contains('a') => 2,
            Status::Err => 2,
            Status::Ok => 3
        }
    }

}

fn main() {
    let mut fuzzer = PathCoverageFuzzer::new();
    fuzzer.fuzz();

    println!("Coverage: {:?}", fuzzer.coverage);
    // Output : Coverage: {2, 0, 1}
}