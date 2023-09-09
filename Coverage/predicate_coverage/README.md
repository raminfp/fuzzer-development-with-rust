# Predicate Coverage

* _Predicate coverage is a code coverage metric used in fuzzing that focuses on exercising the boolean predicate expressions in conditional statements and loops._

# Build

```bash
$ cargo run .
```

# Output

```bash
[13:18] raminfp@zenbook:predicate_coverage (main *%) # cargo run .
   Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/predicate_coverage .`
Starting fuzzing...
Total predicates covered: 3
Coverage: {1, 2, 3}
Achieved full predicate coverage!

``` 