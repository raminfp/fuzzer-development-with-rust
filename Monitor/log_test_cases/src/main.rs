use rand::Rng;
use std::fs::File;
use std::io::Write;

// Fuzz target
fn fuzz(data: &[u8]) {
    if data[0] == 0 {
        panic!("Crash!");
    }
}

fn main() {

    let mut rng = rand::thread_rng();

    let mut crashes = 0;

    loop {

        let mut bytes = [0u8; 32];
        rng.fill(&mut bytes);

        if let Err(_) = std::panic::catch_unwind(|| {
            fuzz(&bytes);
        }) {
            crashes += 1;
            println!("Fuzzer crashed!");

            let mut file = File::create("crash-cases.txt").unwrap();
            file.write_all(&bytes).unwrap();
        }

    }

}