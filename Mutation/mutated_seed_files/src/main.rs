use rand::{Rng, SeedableRng};
use rand::distributions::Uniform;
use rand::rngs::StdRng;
use std::fs;
use std::io::{self};
use std::error::Error;

const MUTATION_AMOUNT: i32 = 10; // The range of mutations (-10 to +10)

fn main() -> Result<(), Box<dyn Error>> {
    // Read the seed file
    let original_seed = read_seed_file("original_seed.txt")?;

    // Generate and write the mutated seed files
    for i in 1..=5 {
        let mutated_seed = mutate_seed(&original_seed)?;
        write_seed_file(mutated_seed, format!("mutated_seed_{}.txt", i))?;
    }

    Ok(())
}

fn read_seed_file(filename: &str) -> Result<Vec<i32>, Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;
    let seed: Vec<i32> = contents
        .lines()
        .map(|line| line.trim().parse::<i32>())
        .collect::<Result<_, _>>()?;
    Ok(seed)
}

fn mutate_seed(seed: &[i32]) -> Result<Vec<i32>, Box<dyn Error>> {
    let mut rng = StdRng::from_entropy();
    let mutation_range = Uniform::new_inclusive(-MUTATION_AMOUNT, MUTATION_AMOUNT);

    let mutated_seed: Vec<i32> = seed
        .iter()
        .map(|&num| num + rng.sample(mutation_range))
        .collect();

    Ok(mutated_seed)
}

fn write_seed_file(seed: Vec<i32>, filename: String) -> Result<(), io::Error> {
    let contents = seed
        .iter()
        .map(|num| num.to_string())
        .collect::<Vec<String>>()
        .join("\n");

    fs::write(filename.clone(), contents)?;
    println!("{} created successfully!", filename);
    Ok(())
}
