use regex::Regex;

fn fuzz_keyword(input: &str) -> String {
    // Define a regex pattern to match the keyword "if"
    let pattern = Regex::new(r"\bif\b").unwrap();

    // Define the replacement value (in this case, "when")
    let replacement = "when";

    // Use the `replace_all` method to find and replace the matched tokens
    let fuzzed_input = pattern.replace_all(input, replacement);

    fuzzed_input.to_string()
}

fn main() {
    let input_code = r#"
        fn main() {
            if true {
                println!("It's true!");
            }
        }
    "#;

    let fuzzed_code = fuzz_keyword(input_code);
    println!("{}", fuzzed_code);
}
