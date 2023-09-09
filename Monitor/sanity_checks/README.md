# Sanity Checks

* _A hang in fuzzing refers to the target application freezing or becoming unresponsive due to some input generated during fuzzing_

# Build

```bash
$ cargo run .
```

# Output

```bash
    Finished dev [unoptimized + debuginfo] target(s) in 12.91s
     Running `target/debug/sanity_checks`
Fuzz timeout!
Iterations: 1, Crashes detected: 0, Elapsed time: 20.01s, CPU usage: 13.779259

```