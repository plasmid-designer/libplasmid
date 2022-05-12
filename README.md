![Plasmid Logo](./logo_header.svg)

Plasmid is a genetics / bioinformatics framework.<br>

It consists of two parts:
- `libplasmid`: Library for genetic code creation, editing and analysis
- `plasmid-cli`: CLI for genetic sequence processing

This is a work-in-progress.

I should also mention that I'm not at all trained in bioinformatics, biochemistry, genetics or anything even remotely similar. I'm writing this library while I'm learning the concepts, so expect some rough edges.

## GUI

The desktop application has moved to [Plasmid Designer](https://github.com/plasmid-designer/plasmid-designer).

## CLI

### Pretty print DNA sequence

```sh
# Command
plasmid-cli pp "ATGTACCCGTATCTG"

# Output
# 5' ATGTACCCGTATCTG 3'
# 3' TACATGGGCATAGAC 5'
#     M  Y  P  Y  L
```

### Match DNA sequence with IUPAC pattern

```sh
# Command
plasmid-cli match "ATGTACCCGTATCTG" "ATGNNNSSSW"

# Output
# 5' ATGTACCCGTATCTG 3'
#    ATGNNNSSSW   
# -> Matches: true
```

### Export circular DNA as SVG

```sh
plasmid-cli export "ATGTACCCGTATCTG" --format svg > out.svg
```

## libPlasmid

### Modules

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

### Scope

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

#### Examples

```rs
fn main() -> Result<(), Box<dyn std::error::Error>> {
  let dna = "ATGGTTCGGCAATTT";

  // Load DNA from string
  let mut seq = DnaSequence.from_str(dna)?;

  // Annotate restriction enzyme cut sites
  seq.annotate_restriction_enzymes();

  // Check if NdeI cut site exists
  if let Some(ann) = seq.annotation_iter().find(|ann| ann.text == "NdeI") {
    println!("Found NdeI cut site (start: {}; end: {})", ann.start, ann.end);
  }

  // Generate SVG of circular DNA
  let conf = SvgExportConfig::circular();
  let svg = SvgExport::new(conf, seq.as_nucleotides());
  println!("{}", svg.export()); // print svg code
}
```
