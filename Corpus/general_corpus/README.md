# General Corpus

* _A general corpus contains a wide variety of valid and invalid sample inputs for the target application. This allows the fuzzer to generate a diverse set of test cases._

# Build
```bash
$ cargo run .
```
# Output
```bash
# cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/general_corpus`
Generated corpus files in corpus
[13:00] raminfp@zenbook:general_corpus (main *%) # exa corpus/
1  5  9   13  17  21  25  29  33  37  41  45  49  53  57  61  65  69  73  77  81  85  89  93  97
2  6  10  14  18  22  26  30  34  38  42  46  50  54  58  62  66  70  74  78  82  86  90  94  98
3  7  11  15  19  23  27  31  35  39  43  47  51  55  59  63  67  71  75  79  83  87  91  95  99
4  8  12  16  20  24  28  32  36  40  44  48  52  56  60  64  68  72  76  80  84  88  92  96  

```