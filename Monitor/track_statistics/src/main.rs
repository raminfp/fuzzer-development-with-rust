use rand::Rng;
use std::time::Instant;

struct FuzzerStats {
    start_time: Instant,
    num_iterations: usize,
    crashes_detected: usize,
}

// The target function we want to fuzz
fn fuzz_target(data: &[u8]) {
    // Code that might panic
    if data[0] == 0 {
        panic!("Crash!");
    }
    // Do something with input data
}

fn main() {

    let mut rng = rand::thread_rng();

    let mut stats = FuzzerStats {
        start_time: Instant::now(),
        num_iterations: 0,
        crashes_detected: 0,
    };

    loop {

        let mut bytes = [0u8; 64];

        rng.fill(&mut bytes);

        stats.num_iterations += 1;

        if let Err(_) = std::panic::catch_unwind(|| {
            fuzz_target(&bytes);
        }) {
            stats.crashes_detected += 1;
        }

        if stats.num_iterations % 100 == 0 {
            let elapsed = stats.start_time.elapsed();
            println!("Iterations: {}, Crashes: {}, Duration: {:?}",
                     stats.num_iterations, stats.crashes_detected, elapsed);
        }

    }

}