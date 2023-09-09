use rand::Rng;
use std::time::{Instant};
use sysinfo::{System, SystemExt};
use sysinfo::CpuExt;


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

    fn record_cpu(&mut self) {
        let t = System::new_all();
        let cpu = t.global_cpu_info().cpu_usage();
        self.cpu_usage = cpu
    }

}

// Fuzz target
fn fuzz(data: &[u8]) {
    if data[0] == 0 {
        panic!("Crash!");
    }
}

fn main() {


    let mut rng = rand::thread_rng();
    let mut stats = PerfStats::new();

    loop {

        let mut bytes = [0u8; 32];
        rng.fill(&mut bytes);

        stats.fuzz_iterations += 1;
        // stats.record_cpu();
        if let Err(_) = std::panic::catch_unwind(|| {
            fuzz(&bytes);
        }) {
            stats.record_crash();
        }
        stats.finish();

    }
}