use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

// let input: u8 = 44; // Binary: 00101100
// let bit_position = 3;
//
// // Flipping the 4th bit using bitwise XOR

// let mutated_byte = input ^ (1 << bit_position);
//
// println!("Original byte: {:08b}", input);
// println!("Mutated byte:  {:08b}", mutated_byte);


// Function to mutate a byte array randomly
fn random_mutate_bytes(input: &[u8]) -> Vec<u8> {
    let mut rng = StdRng::from_entropy();
    let mutation_type = rng.gen_range(0..=2); // 0: Byte Flip, 1: Byte Insertion, 2: Byte Deletion

    match mutation_type {
        0 => {
            // Byte Flip: Choose a random byte and flip one bit randomly
            let position = rng.gen_range(0..input.len());
            let bit_position = rng.gen_range(0..8);
            let mutated_byte = input[position] ^ (1 << bit_position);

            input
                .iter()
                .enumerate()
                .map(|(i, &byte)| if i == position { mutated_byte } else { byte })
                .collect()
        }
        1 => {
            // Byte Insertion: Insert a random byte at a random position
            let position = rng.gen_range(0..=input.len());
            let new_byte: u8 = rng.gen();
            input
                .iter()
                .take(position)
                .chain(std::iter::once(&new_byte))
                .chain(input.iter().skip(position))
                .cloned()
                .collect()
        }
        2 => {
            // Byte Deletion: Remove a random byte at a random position
            if input.is_empty() {
                input.to_vec()
            } else {
                let position = rng.gen_range(0..input.len());
                input
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| *i != position)
                    .map(|(_, &byte)| byte)
                    .collect()
            }
        }
        _ => input.to_vec(),
    }
}

fn main() {
    let original_data: Vec<u8> = vec![0x01, 0x23, 0x45, 0x67, 0x89];
    println!("Original Data: {:?}", original_data);

    for _ in 0..10 {
        let mutated_data = random_mutate_bytes(&original_data);
        println!("Mutated Data:  {:?}", mutated_data);
    }
}
