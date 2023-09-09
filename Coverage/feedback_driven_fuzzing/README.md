# Feedback Driven Fuzzing

* _Feedback-driven fuzzing is a software testing technique that uses feedback from the target program to guide and optimize the fuzzing process for maximum effectiveness._

# Build

```bash
$ cargo run .
```

# Output

```bash
[13:16] raminfp@zenbook:feedback_driven_fuzzing (main *%) # cargo run . | more
  Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/feedback_driven_fuzzing .`
Starting fuzzing...
Fuzzer cycles: 100
Corpus size: 99
Unique coverages: 99
Fuzzer cycles: 200
Corpus size: 199
Unique coverages: 199
Fuzzer cycles: 300
Corpus size: 299
Unique coverages: 299
Fuzzer cycles: 400
Corpus size: 399
Unique coverages: 399
Fuzzer cycles: 500
Corpus size: 499
Unique coverages: 499
Fuzzer cycles: 600
Corpus size: 599
Unique coverages: 599
Fuzzer cycles: 700
Corpus size: 699
Unique coverages: 699
Fuzzer cycles: 800
Corpus size: 799
Unique coverages: 799
Fuzzer cycles: 900
Corpus size: 899
Unique coverages: 899
Fuzzer cycles: 1000

``` 