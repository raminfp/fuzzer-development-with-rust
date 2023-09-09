# Loop Coverage

* _Loop coverage is a code coverage metric used in fuzzing that focuses on executing the loops in the code under test._

# Build

```bash
$ cargo run .
```

# Output

```bash
[13:17] raminfp@zenbook:loop_coverage (main *%) # cargo run .
   Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/loop_coverage .`
Starting fuzzing...
Total loops covered: 3
Coverage: {3, 2, 1}
Achieved full loop coverage!

``` 