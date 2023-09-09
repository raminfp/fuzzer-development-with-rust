# target input/path

* _The target/input fuzzer is a simple but effective fuzzing strategy that mutates or tweaks the existing input files or data samples for a target application._


# Build

```bash
$ cargo run .
```

# Output

```bash
[16:11] raminfp@zenbook:target_input (main *) # cargo run . 
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/target_input .`
Input bytes: Output { status: ExitStatus(unix_wait_status(0)), stdout: "Input value: LCX3wpkS4i\n", stderr: "" }
Stdout: Input value: LCX3wpkS4i

Stderr: 
[16:11] raminfp@zenbook:target_input (main *) # 
```