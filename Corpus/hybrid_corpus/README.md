# Hybrid corpus

* _A corpus may contain a mix of valid and invalid samples. This provides inputs the application should handle and inputs it should reject._


# Build

```bash
$ cargo run .
```

# Output
```bash
[13:01] raminfp@zenbook:hybrid_corpus (main *%) # cargo run .
   Compiling hybrid_corpus v0.1.0 (/home/raminfp/IdeaProjects/Corpus/hybrid_corpus)
    Finished dev [unoptimized + debuginfo] target(s) in 0.26s
     Running `target/debug/hybrid_corpus .`
Created hybrid corpus in hybrid_corpus
[13:01] raminfp@zenbook:hybrid_corpus (main *%) # exa hybrid_corpus
invalid.png  truncated.jpg  valid.jpg  valid.png

```