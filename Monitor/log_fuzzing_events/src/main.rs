#[macro_use]
extern crate log;
extern crate simplelog;
use simplelog::*;
use std::fs::File;
use rand::Rng;

// Fuzz target
fn fuzz(data: &[u8]) {
    if data[0] == 0 {
        panic!("Crash!");
    }
}

fn main() {

    // Initialize logger
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Info, Config::default(), File::create("fuzz.log").unwrap()),
        ]
    ).unwrap();

    let mut rng = rand::thread_rng();

    loop {
        let mut bytes = [0u8; 64];
        rng.fill(&mut bytes);

        info!("Starting iteration");

        if let Err(_) = std::panic::catch_unwind(|| {
            fuzz(&bytes);
        }) {
            error!("Fuzzer crashed!");
        } else {
            info!("Iteration completed successfully");
        }
    }
}