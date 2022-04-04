# plasmid
> Rust Genetic Engineering Toolkit

Scope:
- dna sequence modeling
- rna sequence modeling
- dna transcription
- rna translation
- annotation
- visualization
- import / export

## Modules

| Module   | Description |
| -------- | ----------- |
| `dna`    | DNA Processing |
| `rna`    | RNA Processing |
| `eaa`    | Essential Amino Acid Processing |
| `seq`    | Genetic Sequence Editing and Analysis |
| `vis`    | Visualization Tools |
| `traits` | Helpful Traits |

### Examples

Load DNA from string:
```rs
let dna = "ATGGTTCGGCAATTT";
if let Some(seq) = DnaSequence.from_str(dna) {
    // Print sequence
    println!("{}", seq);
}
```

Load RNA from string:
```rs
let rna = "AUGGUUCGGCAAUUU";
if let Some(seq) = RnaSequence.from_str(rna) {
    // Print sequence
    println!("{}", seq);
}
```