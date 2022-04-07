# plasmid
> Rust Genetic Engineering Toolkit

Plasmid is a genetics / bioinformatics library.<br>

It consists of two parts:
- `libplasmid`: Library for genetic code creation, editing and analysis
- `plasmid-cli`: CLI for genetic sequence processing

This is a work-in-progress.

I should also mention that I'm not at all trained in bioinformatics, biochemistry, genetics or anything even remotely similar. I'm writing this library while I'm learning the concepts, so expect some rough edges.

## Modules

| Module   | Description |
| -------- | ----------- |
| `prelude`| Exports all essential types and traits |
| `dna`    | DNA Nucleotide Processing |
| `rna`    | RNA Nucleotide Processing |
| `eaa`    | Amino Acid Processing |
| `seq`    | Genetic Sequence Editing and Analysis |
| `uni`    | IUPAC Sequence Processing |
| `vis`    | Visualization Tools |
| `imp`    | Import Helpers |
| `exp`    | Export Helpers |
| `traits` | Helpful Traits |

## Scope

- genetic sequence modeling
  - [x] dna/rna nucleotides
  - [x] dna/rna nucleotide triplets
  - [x] dna/rna bp and anticodon handling
  - [x] dna/rna iupac sequence matching
  - [ ] sequence alignment
  - [ ] dynamic dna/rna editing (`seq` module)
    - [x] always keep nucleotide sequence
    - [x] nucleotide triplet iterator
    - [x] reverse complement iterator
    - [x] push/pop nucleotides
    - [ ] push/pop codons
    - [ ] insert/remove nucleotides
    - [ ] substitute nucleotides
    - [ ] base-pair iterator
    - [ ] polypeptide iterator (lazy translation)
  - [x] dna transcription
  - [x] rna polypeptide translation
  - [ ] amino acid reverse transcription
  - [x] genetic codes
    - [x] `ACGT` (see `DnaNucleotide`)
    - [x] `ACGU` (see `RnaNucleotide`)
    - [x] `ACGTWSMKRYBDHVN-` (see `IupacNucleotide`)
- genetic sequence analysis
  - [x] at-count, gc-count, at-ratio, gc-ratio, at/gc-ratio
  - [ ] hairpin detection
  - [ ] open reading frame detection
  - [x] cut site detection
- annotation
  - [x] general sequence annotation
  - [x] auto-annotate cut sites
- visualization
  - [x] simple text-based output
  - [x] plasmid svg generation
- import / export
  - [ ] fasta
  - [ ] fastq
  - [ ] sam
  - [ ] svg
    - [ ] linear
    - [x] circular
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