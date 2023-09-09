# Statement Coverage

* _Statement coverage is a code coverage metric used in fuzzing that tracks whether each executable statement in the code has been executed by at least one test input._

# Build

```bash
$ cargo run .
```

# Output

```bash
[13:20] raminfp@zenbook:statement_coverage (main +*%) # cargo run .
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/statement_coverage .`
Coverage: {0, 2, 1}

``` 