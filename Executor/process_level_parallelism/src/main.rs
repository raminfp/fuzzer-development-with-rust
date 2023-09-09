use rayon::prelude::*;
use std::time::Duration;
use rand::Rng;

fn main() {
    // Number of parallel fuzzing processes
    let num_processes = 4;

    // Launch multiple processes in parallel
    (0..num_processes).into_par_iter().for_each(|process_id| {

        let mut rng = rand::thread_rng();

        // Fuzzing loop for each process
        loop {
            // Generate a random input
            let random_input: Vec<u8> = (0..rng.gen_range(1..50)).map(|_| rng.gen()).collect();

            let result = test_input(&random_input);
            if result {
                println!("Process {}: Found a vulnerability with input {:?}", process_id, random_input);
            }
        }
    });
}

// Target func
fn test_input(input: &[u8]) -> bool {
    std::thread::sleep(Duration::from_millis(10));
    input.len() > 30
}
