# Path Coverage

* _Path coverage is an advanced code coverage metric used in fuzzing that tracks exercised program execution paths through the code._

# Build

```bash
$ cargo run .
```

# Output

```bash
[13:18] raminfp@zenbook:path_coverage (main *%) # cargo run .
   Compiling path_coverage v0.1.0 (/home/raminfp/IdeaProjects/Coverage/path_coverage)
    Finished dev [unoptimized + debuginfo] target(s) in 0.41s
     Running `target/debug/path_coverage .`
Fuzzing: input = short, path = 2
Fuzzing: input = short, path = 2
Fuzzing: input = short, path = 2
Fuzzing: input = , path = 0
Fuzzing: input = short, path = 2
Fuzzing: input = 􏕟𚑋緙𹫆󓮴򊙞񳎑򈐸𕦦󷸦򃱆񰂾񬗲򥥮󁚾䡿􃄛񆝟򩇆򜻩, path = 1
Coverage: {1, 2, 0}

``` 