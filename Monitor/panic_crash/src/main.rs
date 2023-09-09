use rand::Rng;
use std::panic;


// The target function we want to fuzz
fn fuzz_target(data: &[u8]) {
    // Code that might panic
    if data[0] == 0 {
        panic!("Crash!");
        // sotre in file
    }

    // Do something with input data
}

fn main() {

    let mut rng = rand::thread_rng();
    let mut iterations = 0;
    let mut crashes = 0;

    println!("Starting fuzzer");

    loop {
        let mut bytes = [0u8; 64];

        rng.fill(&mut bytes);

        iterations += 1;

        let result = panic::catch_unwind(|| {
            fuzz_target(&bytes);
        });

        if result.is_err() {
            crashes += 1;
            println!("Crash detected on iteration {}", iterations);
            // Can log crashing input here
        }

        // Add mutations of bytes here

        if iterations % 1000 == 0 {
            println!("Iterations: {}, Crashes: {}", iterations, crashes);
        }
    }
}