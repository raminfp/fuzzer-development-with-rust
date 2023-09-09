use rand::Rng;




fn random_mutate_string(input: &str) -> String {
    let mut rng = rand::thread_rng();
    let mutation_type = rng.gen_range(0..=2); // 0: Insertion, 1: Deletion, 2: Substitution

    match mutation_type {
        0 => {
            // let input = "hello world";
            // let position = 5;
            // let mut_char: char = 'X';
            // let mut_str: String = "X".to_string();
            //
            // let mutated_string = input
            // .chars()                   // "hello world" => ['h', 'e', 'l', 'l', 'o', ' ', 'w', 'o', 'r', 'l', 'd']
            // .take(position)            // ['h', 'e', 'l', 'l', 'o']
            // .chain(mut_str.chars())    // ['h', 'e', 'l', 'l', 'o', 'X']
            // .chain(input.chars().skip(position)) // ['h', 'e', 'l', 'l', 'o', 'X', ' ', 'w', 'o', 'r', 'l', 'd']
            // .collect();                // "helloX world"
            //

            // Insertion: Insert a random character at a random position
            let position = rng.gen_range(0..=input.len());
            let mut_char: char = rng.gen();
            let mut_str: String = mut_char.to_string();
            input
                .chars()
                .take(position)
                .chain(mut_str.chars())
                .chain(input.chars().skip(position))
                .collect()
        }
        1 => {
            // let input = "hello world";
            // let position = 5;
            //
            // let mutated_string = input
            //     .chars()                            // "hello world" => ['h', 'e', 'l', 'l', 'o', ' ', 'w', 'o', 'r', 'l', 'd']
            //     .enumerate()                        // Enumerate the characters, producing an iterator of (index, char) pairs
            //     .filter(|(i, _)| *i != position)    // Filter out the character at the given position (5)
            //     .map(|(_, c)| c)                    // Extract only the characters (discard the indices)
            //     .collect();                         // "helloworld"


            // Deletion: Remove a random character at a random position
            if input.is_empty() {
                input.to_string()
            } else {
                let position = rng.gen_range(0..input.len());
                input
                    .chars()
                    .enumerate()
                    .filter(|(i, _)| *i != position)
                    .map(|(_, c)| c)
                    .collect()
            }
        }
        2 => {

            // let input = "hello world";
            // let position = 5;
            // let mut_char: char = 'X';
            //
            // let mutated_string = input
            //     .chars()                                // "hello world" => ['h', 'e', 'l', 'l', 'o', ' ', 'w', 'o', 'r', 'l', 'd']
            //     .enumerate()                            // Enumerate the characters, producing an iterator of (index, char) pairs
            //     .map(|(i, c)| if i == position { mut_char } else { c })  // Substitute character at the given position (5) with `mut_char`
            //     .collect();                             // "helloXworld"


            // Substitution: Substitute a random character at a random position with another random character
            if input.is_empty() {
                input.to_string()
            } else {
                let position = rng.gen_range(0..input.len());
                let mut_char: char = rng.gen();
                input
                    .chars()
                    .enumerate()
                    .map(|(i, c)| if i == position { mut_char } else { c })
                    .collect()
            }
        }
        _ => input.to_string(),
    }
}

fn main() {
    let input_string = "Hello, World!";
    println!("Original String: {}", input_string);

    for _ in 0..10 {
        let mutated_string = random_mutate_string(input_string);
        println!("Mutated String:  {}", mutated_string);
    }
}