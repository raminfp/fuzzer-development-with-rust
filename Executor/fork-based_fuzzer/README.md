# Fork Based Fuzzer

* _A fork-based fuzzer is a type of coverage-guided fuzzer that uses fork() system calls to test each input in a separate process._


# Build

```bash
$ cargo run .
```

# Output

```bash
[16:07] raminfp@zenbook:fork-based_fuzzer (main *) # cargo run .
   Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/fork-based_fuzzer .`
Starting fuzzer...
Test case #1
Crashed! 6
Test case #2
Crashed! 6
Test case #3
Crashed! 6
Test case #4
Crashed! 6
Test case #5
Crashed! 6
Test case #6
Crashed! 6
Test case #7
Crashed! 6
Test case #8
Crashed! 6
Test case #9
Crashed! 6
Test case #10
Crashed! 6
Test case #11
Crashed! 6
Test case #12
Crashed! 6
Test case #13
Crashed! 6
Test case #14
Crashed! 6
Test case #15
Crashed! 6
Test case #16
Crashed! 6
Test case #17
^CCrashed! 6
Test case #18
Crashed! 6
Test case #19
Crashed! 6
Test case #20
Crashed! 6
Test case #21
Crashed! 6
Test case #22
Crashed! 6
Test case #23

```