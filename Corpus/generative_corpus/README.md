# Generative corpus

* _A generative corpus uses grammars, models or generators to automatically create a diverse corpus for the fuzzer._

# Build

```bash
$ cargo run .
```

# Output
```bash
[13:07] raminfp@zenbook:generative_corpus (main) # cargo run .
   Compiling generative_corpus v0.1.0 (/home/raminfp/IdeaProjects/Corpus/generative_corpus)
   Finished dev [unoptimized + debuginfo] target(s) in 0.36s
     Running `target/debug/generative_corpus .`
Stored 100 samples in fuzz_corpus
[13:08] raminfp@zenbook:generative_corpus (main %) # exa fuzz_corpus/
sample_000  sample_012  sample_024  sample_036  sample_048  sample_060  sample_072  sample_084  sample_096
sample_001  sample_013  sample_025  sample_037  sample_049  sample_061  sample_073  sample_085  sample_097
sample_002  sample_014  sample_026  sample_038  sample_050  sample_062  sample_074  sample_086  sample_098
sample_003  sample_015  sample_027  sample_039  sample_051  sample_063  sample_075  sample_087  sample_099
sample_004  sample_016  sample_028  sample_040  sample_052  sample_064  sample_076  sample_088  
sample_005  sample_017  sample_029  sample_041  sample_053  sample_065  sample_077  sample_089  
sample_006  sample_018  sample_030  sample_042  sample_054  sample_066  sample_078  sample_090  
sample_007  sample_019  sample_031  sample_043  sample_055  sample_067  sample_079  sample_091  
sample_008  sample_020  sample_032  sample_044  sample_056  sample_068  sample_080  sample_092  
sample_009  sample_021  sample_033  sample_045  sample_057  sample_069  sample_081  sample_093  
sample_010  sample_022  sample_034  sample_046  sample_058  sample_070  sample_082  sample_094  
sample_011  sample_023  sample_035  sample_047  sample_059  sample_071  sample_083  sample_095  

```