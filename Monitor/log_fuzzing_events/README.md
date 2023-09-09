# Fuzzing Events
    
* _Record when fuzzing starts/ends, when crashes occur, when interesting test cases are found, etc._

# Build

```bash
$ cargo run .
```

# Output

```bash
[16:35] raminfp@zenbook:log_fuzzing_events (main *) # cargo run .
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/log_fuzzing_events .`
thread 'main' panicked at 'Crash!', src/main.rs:11:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
13:05:10 [ERROR] Fuzzer crashed!
thread 'main' panicked at 'Crash!', src/main.rs:11:9
13:05:10 [ERROR] Fuzzer crashed!
thread 'main' panicked at 'Crash!', src/main.rs:11:9
13:05:10 [ERROR] Fuzzer crashed!
thread 'main' panicked at 'Crash!', src/main.rs:11:9
13:05:10 [ERROR] Fuzzer crashed!
thread 'main' panicked at 'Crash!', src/main.rs:11:9
13:05:10 [ERROR] Fuzzer crashed!
thread 'main' panicked at 'Crash!', src/main.rs:11:9
13:05:10 [ERROR] Fuzzer crashed!
thread 'main' panicked at 'Crash!', src/main.rs:11:9
13:05:10 [ERROR] Fuzzer crashed!
thread 'main' panicked at 'Crash!', src/main.rs:11:9
13:05:10 [ERROR] Fuzzer crashed!
thread 'main' panicked at 'Crash!', src/main.rs:11:9
13:05:10 [ERROR] Fuzzer crashed!
thread 'main' panicked at 'Crash!', src/main.rs:11:9
13:05:10 [ERROR] Fuzzer crashed!
thread 'main' panicked at 'Crash!', src/main.rs:11:9
13:05:10 [ERROR] Fuzzer crashed!
thread 'main' panicked at 'Crash!', src/main.rs:11:9
13:05:10 [ERROR] Fuzzer crashed!
thread 'main' panicked at 'Crash!', src/main.rs:11:9
13:05:10 [ERROR] Fuzzer crashed!
thread 'main' panicked at 'Crash!', src/main.rs:11:9
13:05:10 [ERROR] Fuzzer crashed!
thread 'main' panicked at 'Crash!', src/main.rs:11:9
13:05:10 [ERROR] Fuzzer crashed!
thread 'main' panicked at 'Crash!', src/main.rs:11:9
13:05:10 [ERROR] Fuzzer crashed!
thread 'main' panicked at 'Crash!', src/main.rs:11:9
13:05:10 [ERROR] Fuzzer crashed!
thread 'main' panicked at 'Crash!', src/main.rs:11:9
13:05:10 [ERROR] Fuzzer crashed!
thread 'main' panicked at 'Crash!', src/main.rs:11:9

```