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

### dna
> DNA Tools

### rna
> RNA Tools

Load RNA from String:
```rs
let seq = "AUGGUUCGGCAAUUU";
if let Some(seq) = RnaSequence.from_str(seq) {
    // Print sequence
    println!("{}", seq);
}
```

### eaa
> Essential Amino Acid Tools

### vis
> Visualization Tools