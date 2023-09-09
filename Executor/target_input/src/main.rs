// use std::process::Command;
//
// fn main() {
//     let input_value = "hello";
//
//     let output = Command::new("./target_program")
//         .arg(input_value)
//         .output()
//         .expect("failed to execute process");
//
//     println!("Status: {}", output.status);
//     println!("Stdout: {}", String::from_utf8_lossy(&output.stdout));
//     println!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
// }

use std::process::{Command, Stdio};
use rand::Rng;

const MAX_LEN:usize = 256;

fn generate_random_testcase_byte(length: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut bytes = Vec::with_capacity(length);

    for _ in 0..length {
        bytes.push(rng.gen());
    }

    bytes
}

fn generate_random_string(length: usize) -> String {
    let mut rng = rand::thread_rng();
    std::iter::repeat_with(|| rng.sample(rand::distributions::Alphanumeric))
        .map(char::from)
        .take(length)
        .collect()
}

fn main() {
    // let input_bytes = b"hello";
    // let input_bytes = generate_random_string(MAX_LEN);
    // let input_bytes = b"hello\n".to_vec();
    // let input_str = String::from_utf8(input_bytes.to_vec()).unwrap();

    let random_string = generate_random_string(10);
    let output = Command::new("./target_program")
        .arg(random_string)
        .output()
        .expect("failed to execute process");

    println!("Input bytes: {:x?}", &output);
    // Print stdout and stderr
    println!("Stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
}