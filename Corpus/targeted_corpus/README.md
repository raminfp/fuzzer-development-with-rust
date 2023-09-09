# Targeted corpus?

* **A corpus can be tailored to focus on a specific feature or component of the target application. This allows more targeted fuzzing.**

# Build

```bash
$ cargo run .
```

# Output
```bash
[13:05] raminfp@zenbook:targeted_corpus (main +*%) # cargo run .
   Compiling targeted_corpus v0.1.0 (/home/raminfp/IdeaProjects/Corpus/targeted_corpus)
    Finished dev [unoptimized + debuginfo] target(s) in 0.26s
     Running `target/debug/targeted_corpus .`
Created image corpus in image_corpus
[13:05] raminfp@zenbook:targeted_corpus (main +*%) # exa image_corpus/
invalid.png  valid.png
```
