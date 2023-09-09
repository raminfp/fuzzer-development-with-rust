use rand::Rng;

fn arithmetic_mutate(input: &mut [u8]) {
    let mut rng = rand::thread_rng();
    // Pick a random byte to mutate
    let index = rng.gen_range(0..input.len());
    let op = rng.gen_range(0..4);
    match op {
        0 => input[index] += 1,
        1 => input[index] -= 1,
        2 => input[index] *=2,
        3 => input[index] /= 2,
        _ => {}
    };
}

fn main() {
    let mut input = vec![1,2,3,4];
    println!("input: {:?}", &input);
    arithmetic_mutate(&mut input);
    println!("input: {:?}", &input);


}
