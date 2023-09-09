# Crash

* _A crash in fuzzing refers to the target application crashing or abruptly terminating due to the fuzzed or mutated input that was provided to it_

# Build

```bash
$ cargo run .
```

# Output

```bash
Crash detected on iteration 198924
Iterations: 199000, Crashes: 778
thread 'main' panicked at 'Crash!', src/main.rs:9:9
Crash detected on iteration 199415
thread 'main' panicked at 'Crash!', src/main.rs:9:9
Crash detected on iteration 199729
thread 'main' panicked at 'Crash!', src/main.rs:9:9
Crash detected on iteration 199911
Iterations: 200000, Crashes: 781
thread 'main' panicked at 'Crash!', src/main.rs:9:9
Crash detected on iteration 200492
Iterations: 201000, Crashes: 782
thread 'main' panicked at 'Crash!', src/main.rs:9:9
Crash detected on iteration 201807
thread 'main' panicked at 'Crash!', src/main.rs:9:9
Crash detected on iteration 201815
thread 'main' panicked at 'Crash!', src/main.rs:9:9
Crash detected on iteration 201817
Iterations: 202000, Crashes: 785
thread 'main' panicked at 'Crash!', src/main.rs:9:9
Crash detected on iteration 202150
thread 'main' panicked at 'Crash!', src/main.rs:9:9
Crash detected on iteration 202343
thread 'main' panicked at 'Crash!', src/main.rs:9:9
Crash detected on iteration 202378

```