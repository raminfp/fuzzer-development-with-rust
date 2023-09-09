use std::thread;
use std::time::Duration;
use rand::Rng;

fn main() {
    // Number of parallel fuzzing threads
    let num_threads = 4;

    // Create a vector to hold the thread handles
    let mut handles = vec![];

    // Launch multiple threads in parallel
    for thread_id in 0..num_threads {
        let handle = thread::spawn(move || {
            let mut rng = rand::thread_rng();
            // Fuzzing loop for each thread
            loop {
                let random_input: Vec<u8> = (0..rng.gen_range(1..50)).map(|_| rng.gen()).collect();
                let result = test_input(&random_input);
                if result {
                    println!("Thread {}: Found a vulnerability with input {:?}", thread_id, random_input);
                    break;
                }
            }
        });

        handles.push(handle);
    }
    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }
}

// Target func
fn test_input(input: &[u8]) -> bool {

    std::thread::sleep(Duration::from_millis(10));
    input.len() > 30
}
