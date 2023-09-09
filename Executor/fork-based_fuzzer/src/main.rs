use rand::Rng;
use std::process::{Command, ExitStatus, Stdio};
use std::io::Write;
use std::os::unix::ffi::OsStringExt;
use std::io::{ErrorKind};
use std::os::unix::process::ExitStatusExt;

const MAX_LEN: usize = 200;

fn main() {
    println!("Starting fuzzer...");
    let mut iteration = 0;
    // fuzzing loop until found a crash
    loop {
        let random_string = generate_random_string(MAX_LEN);
        iteration += 1;
        println!("Test case #{}", iteration);
        let status = run_target(&random_string);
        handle_child_exit(status);

    }
}

fn generate_random_string(length: usize) -> String {
    let mut rng = rand::thread_rng();
    std::iter::repeat_with(|| rng.sample(rand::distributions::Alphanumeric))
        .map(char::from)
        .take(length)
        .collect()
}

fn run_target(testcase: &str) -> ExitStatus {

    let output = Command::new("./target1")
        .arg(testcase)
        .output()
        .expect("failed to execute process");
    output.status
}

fn handle_child_exit(status_code: ExitStatus) {

    let exit_code = status_code.signal();
    match exit_code {
        Some(exit_code) => {
            println!("Crashed! {}", exit_code);
        }
        None => {
            println!("Exited due to signal");
        }
    }

}
