# Random Mutation

* _This simple approach involves randomly modifying bytes or bits in the input data to create mutated test cases._

# Build

```bash
$ cargo run .
```

# Output

```bash
[16:45] raminfp@zenbook:gen_rand_mutation (main +*) # cargo run
   Compiling gen_rand_mutation v0.1.0 (/home/raminfp/IdeaProjects/Mutation/gen_rand_mutation)
    Finished dev [unoptimized + debuginfo] target(s) in 0.40s
     Running `target/debug/gen_rand_mutation`
Original String: Hello, World!
Mutated String:  Hello,򏒰World!
Mutated String:  Hello񧞍 World!
Mutated String:  Hell𚽂, World!
Mutated String:  Hello, Wrld!
Mutated String:  Hell󔆍o, World!
Mutated String:  He󥒗llo, World!
Mutated String:  Helo, World!
Mutated String:  Hello, Wold!
Mutated String:  Hello, W񜡅orld!
Mutated String:  􎬦Hello, World!

```