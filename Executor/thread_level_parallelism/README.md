# Thread-Level Parallelism

* _Thread-level parallelism refers to a fuzzing optimization where mutiple concurrent threads are leveraged to increase fuzzing performance_


# Build

```bash
$ cargo run .
```

# Output

```bash
[16:12] raminfp@zenbook:thread_level_parallelism (main *) # cargo run .
   Compiling thread_level_parallelism v0.1.0 (/home/raminfp/IdeaProjects/Executor/thread_level_parallelism)
    Finished dev [unoptimized + debuginfo] target(s) in 3.46s
     Running `target/debug/thread_level_parallelism .`
Thread 1: Found a vulnerability with input [86, 250, 175, 127, 247, 56, 31, 231, 28, 0, 104, 250, 152, 1, 65, 20, 203, 253, 178, 41, 110, 14, 4, 108, 167, 186, 252, 235, 49, 249, 151, 208, 31]
Thread 0: Found a vulnerability with input [127, 28, 209, 62, 231, 30, 88, 97, 54, 199, 87, 177, 191, 69, 180, 5, 248, 59, 151, 92, 3, 90, 84, 216, 78, 213, 253, 9, 58, 251, 40, 201, 69, 45, 190, 75, 64, 219]
Thread 3: Found a vulnerability with input [21, 181, 102, 199, 159, 196, 88, 202, 131, 131, 60, 117, 116, 33, 64, 34, 80, 62, 199, 222, 33, 46, 108, 117, 195, 212, 152, 122, 92, 142, 105, 127, 166]
Thread 2: Found a vulnerability with input [211, 205, 88, 56, 143, 53, 202, 205, 168, 102, 22, 169, 180, 69, 97, 40, 98, 160, 176, 197, 227, 79, 108, 47, 20, 99, 224, 111, 114, 133, 10, 23, 88, 227, 116, 249, 17, 176, 142, 147, 82, 64, 124, 250]

```