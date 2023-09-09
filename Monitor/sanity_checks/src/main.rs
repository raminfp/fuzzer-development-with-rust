use std::io;
use io::Write;
use rand::Rng;
use std::time::{Duration, Instant};
use sysinfo::{System, SystemExt};
use sysinfo::CpuExt;
use std::thread::sleep_ms;



struct PerfStats {
    start: Instant,
    fuzz_iterations: u32,
    crashes: u32,
    cpu_usage: f32,
}

impl PerfStats {

    fn new() -> Self {
        let t = System::new_all();
        Self {
            start: Instant::now(),
            fuzz_iterations: 0,
            crashes: 0,
            cpu_usage:  t.global_cpu_info().cpu_usage(),
        }
    }

    fn record_crash(&mut self) {
        self.crashes += 1;
    }

    fn finish(&self) {
        let duration = self.start.elapsed();
        println!("Iterations: {}, Crashes detected: {}, Elapsed time: {:.2?}, CPU usage: {}",
                 self.fuzz_iterations, self.crashes, duration, self.cpu_usage);
    }
}

// Fuzz target
fn fuzz(data: &[u8]) {
    sleep_ms(10000);
    if data[0] == 0 {
        panic!("Crash!");
    }
}


fn fuzz_with_timeout(data: &[u8]) {

    let start = Instant::now();
    fuzz(data);
    let duration = start.elapsed();
    match duration {
        d if d > Duration::from_secs(5) => {
            println!("Fuzz timeout!");
        },
        _ => (),
    }

}

fn main() {

    let mut rng = rand::thread_rng();
    let mut stats = PerfStats::new();

    loop {

        let mut bytes = [0u8; 32];
        rng.fill(&mut bytes);
        stats.fuzz_iterations += 1;

        // Timeout check
        fuzz_with_timeout(&bytes);

        if let Err(_) = std::panic::catch_unwind(|| {
            fuzz(&bytes);
        }) {
            stats.record_crash();
        }
        stats.finish();

    }
}