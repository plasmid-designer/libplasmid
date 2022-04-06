# plasmid
> Rust Genetic Engineering Toolkit

Plasmid is a genetics / bioinformatics library.<br>

It consists of two parts:
- `libplasmid`:<br>
  Library for genetic code creation, editing and analysis
- `plasmid-cli`:<br>
  CLI for genetic sequence processing

This is a work-in-progress. I should also mention that I'm not at all trained in bioinformatics, biochemistry, genetics or anything even remotely similar. I'm writing this library while I'm learning the concepts, so expect some rough edges.

## Modules

| Module   | Description |
| -------- | ----------- |
| `dna`    | DNA Processing |
| `rna`    | RNA Processing |
| `eaa`    | Essential Amino Acid Processing |
| `seq`    | Genetic Sequence Editing and Analysis |
| `uni`    | IUPAC Sequence Processing |
| `vis`    | Visualization Tools |
| `traits` | Helpful Traits |

## Scope

- genetic sequence modeling
  - [x] dna/rna nucleotides
  - [x] dna/rna nucleotide triplets
  - [x] dna/rna bp and anticodon handling
  - [x] dna/rna iupac sequence matching
  - [ ] dynamic dna/rna editing (`seq` module)
    - [x] always keep nucleotide sequence
    - [x] nucleotide triplet iterator
    - [x] reverse complement iterator
    - [x] push/pop nucleotides
    - [x] push/pop triplets
    - [ ] insert/remove nucleotides
    - [ ] substitute nucleotides
    - [ ] base-pair iterator
    - [ ] polypeptide iterator (lazy translation)
  - [x] dna transcription
  - [ ] rna reverse transcription
  - [x] rna polypeptide translation
  - [x] genetic codes
    - [x] `ACGT` (see `DnaNucleotide`)
    - [x] `ACGU` (see `RnaNucleotide`)
    - [x] `ACGTWSMKRYBDHVN-` (IUPAC)
- genetic sequence analysis
  - [ ] gc<sup>n</sup> content
  - [ ] at/gc content
  - [ ] hairpin detection
  - [ ] open reading frame detection
  - [x] cut site detection
- annotation
  - [ ] general sequence annotation
  - [x] auto-annotate restriction sites
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