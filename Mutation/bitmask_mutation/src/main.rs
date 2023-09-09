use rand::Rng;

fn bitmask_mutate(input: &mut Vec<u8>) {
    let mut rng = rand::thread_rng();
    // Generate random bitmask
    let mask: u8 = rng.gen();
    // Choose random byte to mutate
    let index = rng.gen_range(0..input.len());
    // Apply bitmask mutation
    input[index] ^= mask;
}

fn main() {
    let mut input = vec![0b01010101, 0b10101010];
    println!("Original input: {:?}", &input);
    bitmask_mutate(&mut input);
    println!("Mutated input: {:?}", &input);
}
