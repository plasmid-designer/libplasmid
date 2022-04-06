use clap::{Parser, Subcommand};
use plasmid::{
    prelude::*,
    traits::{ToLetter, TryFromLetter},
};

#[derive(Debug, Subcommand)]
#[allow(non_camel_case_types)]
enum Commands {
    /// Pretty print sequence
    pp { sequence: String },
    /// Match sequences
    r#match {
        /// The first DNA sequence (ACGT)
        sequence: String,
        /// The DNA pattern (IUPAC)
        pattern: String,
    },
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
/// Plasmid genetic code toolkit
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

fn main() -> Result<(), ()> {
    let args = Args::parse();
    match args.command {
        // Pretty Print
        Commands::pp { sequence } => {
            let seq = DnaSequence::from_str(sequence)?;
            let nucleotides = seq.to_nucleotide_string();
            let antinucleotides = seq.to_reverse_complement_string();
            let codonstr = seq
                .as_codons()
                .iter()
                .map(|c| format!(" {} ", c.translate().to_letter()))
                .collect::<String>();
            println!(
                "5' -> {} -> 3'\n3' <- {} <- 5'\n      {}",
                nucleotides, antinucleotides, codonstr
            );
        }
        // Match
        Commands::r#match { sequence, pattern } => {
            let seq = DnaSequence::from_str(sequence)?;
            let iupac_seq = pattern
                .chars()
                .map(|c| IupacNucleotide::try_from_letter(c).unwrap())
                .collect::<Vec<_>>();
            println!(
                "5' {} 3'\n   {}\n-> Matches: {}",
                seq.to_nucleotide_string(),
                pattern,
                seq.matches(&iupac_seq)
            );
        }
    }
    Ok(())
}
