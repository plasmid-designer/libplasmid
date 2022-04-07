use clap::{ArgEnum, Parser, Subcommand};
use pad::PadStr;
use plasmid::{
    prelude::*,
    traits::{ToLetter, TryFromLetter},
};

#[derive(ArgEnum, Debug, Clone)]
enum Strand {
    Watson,
    Crick,
}

#[derive(ArgEnum, Debug, Clone)]
enum ExportFormat {
    Svg,
}

impl Strand {
    pub fn start_str(&self) -> String {
        match self {
            Self::Watson => "5' ",
            Self::Crick => "3' ",
        }
        .to_string()
    }

    pub fn end_str(&self) -> String {
        match self {
            Self::Watson => " 3'",
            Self::Crick => " 5'",
        }
        .to_string()
    }

    pub fn format_str<T>(&self, str: T) -> String
    where
        T: AsRef<str>,
    {
        let start = self.start_str();
        let end = self.end_str();
        format!("{}{}{}", start, str.as_ref(), end)
    }

    pub fn pad<T>(&self, str: T) -> String
    where
        T: AsRef<str>,
    {
        let start = "".pad_to_width(self.start_str().len());
        let end = "".pad_to_width(self.end_str().len());
        format!("{}{}{}", start, str.as_ref(), end)
    }

    pub fn complement(&self) -> Strand {
        match self {
            Self::Watson => Strand::Crick,
            Self::Crick => Strand::Watson,
        }
    }
}

#[derive(Debug, Subcommand)]
#[allow(non_camel_case_types)]
enum Commands {
    /// Pretty print sequence
    pp {
        sequence: String,
        #[clap(long("strand"), arg_enum)]
        strand: Option<Strand>,
    },
    /// Match sequences
    r#match {
        /// The first DNA sequence (ACGT)
        sequence: String,
        /// The DNA pattern (IUPAC)
        pattern: String,
        #[clap(long("strand"), arg_enum)]
        strand: Option<Strand>,
    },
    /// Export
    export {
        sequence: String,
        #[clap(long("format"), arg_enum)]
        format: ExportFormat,
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    match args.command {
        // Pretty Print
        Commands::pp { sequence, strand } => {
            let strand = strand.unwrap_or_else(|| Strand::Watson);
            let seq = DnaSequence::from_str(sequence)?;
            let nucleotides = seq.to_nucleotide_string();
            let antinucleotides = seq.to_reverse_complement_string();
            let codonstr = seq
                .as_codons()
                .iter()
                .map(|c| format!(" {} ", c.translate().to_letter()))
                .collect::<String>();
            println!(
                "{}\n{}\n{}",
                strand.format_str(nucleotides),
                strand.complement().format_str(antinucleotides),
                strand.pad(codonstr)
            );
        }
        // Match
        Commands::r#match {
            sequence,
            pattern,
            strand,
        } => {
            let strand = strand.unwrap_or_else(|| Strand::Watson);
            let seq = DnaSequence::from_str(sequence)?;
            let iupac_seq = pattern
                .chars()
                .map(|c| IupacNucleotide::try_from_letter(c).unwrap())
                .collect::<Vec<_>>();
            println!(
                "{}\n{}\n-> Matches: {}",
                strand.format_str(seq.to_nucleotide_string()),
                strand.pad(pattern),
                seq.matches(&iupac_seq)
            );
        }
        // Export
        Commands::export { sequence, format } => match format {
            ExportFormat::Svg => {
                let seq = DnaSequence::from_str(sequence)?;
                let conf = SvgExportConfig::circular();
                let exp = SvgExport::new(conf, seq.as_nucleotides());
                println!("{}", exp.export());
            }
        },
    }
    Ok(())
}
