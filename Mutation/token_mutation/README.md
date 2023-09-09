# Token Mutation

* _Fuzzing targets specific tokens in the input data, like keywords or identifiers, and replaces them with different values._

# Build

```bash
$ cargo run .
```

# Output

```bash
[16:46] raminfp@zenbook:token_mutation (main +*) # cargo run
   Compiling token_mutation v0.1.0 (/home/raminfp/IdeaProjects/Mutation/token_mutation)
    Finished dev [unoptimized + debuginfo] target(s) in 1.04s
     Running `target/debug/token_mutation`

        fn main() {
            when true {
                println!("It's true!");
            }
        }
    

```