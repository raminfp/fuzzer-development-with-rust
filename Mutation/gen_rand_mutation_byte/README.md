# Byte Flip

* _In this technique, random bytes in the input data are flipped (i.e., a 0 is changed to 1 or vice versa)._

# Build

```bash
$ cargo run .
```

# Output

```bash
[16:45] raminfp@zenbook:gen_rand_mutation_byte (main +*) # cargo run
   Compiling gen_rand_mutation_byte v0.1.0 (/home/raminfp/IdeaProjects/Mutation/gen_rand_mutation_byte)
    Finished dev [unoptimized + debuginfo] target(s) in 0.38s
     Running `target/debug/gen_rand_mutation_byte`
Original Data: [1, 35, 69, 103, 137]
Mutated Data:  [1, 35, 69, 103, 153]
Mutated Data:  [1, 35, 69, 103, 200, 137]
Mutated Data:  [1, 69, 103, 137]
Mutated Data:  [1, 69, 103, 137]
Mutated Data:  [1, 35, 69, 233, 103, 137]
Mutated Data:  [35, 69, 103, 137]
Mutated Data:  [1, 35, 69, 103, 137, 97]
Mutated Data:  [1, 35, 69, 137]
Mutated Data:  [1, 3, 69, 103, 137]
Mutated Data:  [1, 35, 69, 137]

```