# plasmid
> Rust Genetic Engineering Toolkit

## Modules

| Module   | Description |
| -------- | ----------- |
| `dna`    | DNA Processing |
| `rna`    | RNA Processing |
| `eaa`    | Essential Amino Acid Processing |
| `seq`    | Genetic Sequence Editing and Analysis |
| `vis`    | Visualization Tools |
| `traits` | Helpful Traits |

## Scope

- genetic sequence modeling
  - [x] dna/rna nucleobases
  - [x] dna/rna nucleotide triplets
  - [x] dna/rna bp and anticodon handling
  - [ ] dynamic dna/rna editing (`seq` module)
    - [x] always keep nucleobase sequence
    - [x] nucleotide triplet iterator
    - [x] push/pop nucleobases
    - [ ] insert/remove nucleobases
    - [ ] substitute nucleobases
    - [ ] base-pair iterator
    - [ ] anticodon iterator
    - [ ] polypeptide iterator (lazy translation)
  - [ ] dna transcription
  - [ ] rna reverse transcription
  - [x] rna polypeptide translation
  - [ ] genetic codes
    - [x] `ACGT` (see `DnaNucleoBase`)
    - [x] `ACGU` (see `RnaNucleoBase`)
    - [ ] `ACGTUN` (not sure if needed?)
    - [ ] `ACGTUWSMKRYBDHVN*-` (IUPAC)
- genetic sequence analysis
  - [ ] gc<sup>n</sup> content
  - [ ] at/gc content
  - [ ] hairpin detection
  - [ ] open reading frame detection
  - [ ] cut site detection
- annotation
  - [ ] general sequence annotation
  - [ ] automatic annotation
- visualization
  - [ ] simple text-based output
  - [ ] plasmid svg generation
- import / export
  - [ ] fasta
  - [ ] fastq
  - [ ] sam
  - [ ] custom?

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