# Seed Corpus

_* _A seed corpus contains a small set of valid and invalid samples to bootstrap the fuzzing process. The fuzzer mutates these initial seeds to expand code coverage.__


# Build

```bash
$ cargo run .
```

# Output
```bash
[13:02] raminfp@zenbook:seed_corpus (main *%) # cargo run .
    Updating crates.io index
    Finished dev [unoptimized + debuginfo] target(s) in 3.12s
     Running `target/debug/seed_corpus .`
Created seed corpus in seed_corpus
[13:03] raminfp@zenbook:seed_corpus (main *%) # exa seed_corpus
seed1.txt  seed2.txt  seed3.txt
```

