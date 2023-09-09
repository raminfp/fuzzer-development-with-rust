use std::process::Command;
use rand::Rng;
use rand::prelude::*;
use rand::seq::SliceRandom;


fn main() {
    loop {
        // Generate a random URL as input for wget.
        let url = generate_random_url();
        println!("{}", url);
        // Run wget with the generated URL as the argument.
        let output = Command::new("wget")
            .arg("--no-check-certificate")
            .arg(url)
            .output();

        // Check for any error output.
        if let Err(err) = output {
            eprintln!("wget failed: {}", err);
        }
    }
}

fn generate_random_url() -> String {
    let mut rng = rand::thread_rng();
    let url_length = rng.gen_range(5..=20); // Generate URL length between 5 and 20 characters.

    let charset: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let url: String = (0..url_length)
        .map(|_| {
            // Generate a random ASCII character between 'a' and 'z'.
            *charset.choose(&mut rng).unwrap()
        })
        .collect();

    format!("http://www.{}.com", url)
}