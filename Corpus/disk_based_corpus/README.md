# Disk-Based-Corpus

* _Disk-based corpus refers to a corpus (collection of texts or speech data) that is too large to fit into the main memory of a computer and needs to be stored on disk_


# Build
```bash
$ cargo run
```
# Output
```bash
# cargo run 
   Compiling disk_based_corpus v0.1.0 (/home/raminfp/IdeaProjects/Corpus/disk_based_corpus)
    Finished dev [unoptimized + debuginfo] target(s) in 0.34s
     Running `target/debug/disk_based_corpus`
Fuzzed input: [72, 101, 108, 186, 111]
Fuzzed input: [72, 101, 108, 186, 39]
Fuzzed input: [72, 101, 108, 186, 77]
Fuzzed input: [72, 101, 211, 186, 77]
Fuzzed input: [72, 101, 211, 186, 180]
Fuzzed input: [72, 101, 75, 186, 180]
Fuzzed input: [72, 183, 75, 186, 180]
Fuzzed input: [72, 183, 75, 44, 180]
Fuzzed input: [18, 183, 75, 44, 180]
Fuzzed input: [18, 239, 75, 44, 180]
Fuzzed input: [18, 239, 75, 111, 180]
Fuzzed input: [26, 239, 75, 111, 180]
Fuzzed input: [26, 239, 24, 111, 180]
Fuzzed input: [26, 239, 24, 92, 180]
Fuzzed input: [26, 239, 24, 92, 185]
Fuzzed input: [26, 239, 24, 92, 207]
Fuzzed input: [26, 239, 24, 71, 207]
Fuzzed input: [26, 239, 24, 113, 207]
Fuzzed input: [26, 183, 24, 113, 207]
Fuzzed input: [26, 183, 24, 63, 207]
Fuzzed input: [26, 183, 24, 211, 207]
Fuzzed input: [26, 183, 11, 211, 207]
Fuzzed input: [26, 183, 45, 211, 207]
Fuzzed input: [26, 183, 45, 211, 221]

```