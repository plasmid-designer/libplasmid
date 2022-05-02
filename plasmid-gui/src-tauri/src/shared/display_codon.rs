use plasmid::{
    prelude::DnaCodon,
    traits::{Nucleotide, ToLetter, TryFromStr},
    uni::IupacNucleotide,
};

#[derive(serde::Serialize)]
pub struct DisplayCodon {
    pub nucleotides: Vec<char>,
    pub anti_nucleotides: Vec<char>,
    pub peptide: Option<char>,
}

impl DisplayCodon {
    pub fn new(nucleotide_sequence: &[IupacNucleotide]) -> Self {
        // Compute list of nucleotides and their complements
        let mut nucleotides = Vec::new();
        let mut anti_nucleotides = Vec::new();
        for n in nucleotide_sequence {
            nucleotides.push(n.to_letter());
            anti_nucleotides.push(n.complement().to_letter());
        }

        // Try to interpret the codon as a peptide
        let peptide = if nucleotide_sequence.len() == 3 {
            DnaCodon::try_from_str(
                nucleotide_sequence
                    .iter()
                    .map(|n| n.to_letter())
                    .collect::<String>(),
            )
            .map(|c| c.translate().to_letter())
            .ok()
        } else {
            None
        };

        DisplayCodon {
            nucleotides,
            anti_nucleotides,
            peptide,
        }
    }
}
