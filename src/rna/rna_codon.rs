use std::error::Error;
use std::fmt::Display;

use super::{RnaCodonBuilder, RnaNucleoBase};

#[derive(Debug)]
pub enum RnaCodonDecodingError {
    InvalidLength { expected: usize, actual: usize },
    InvalidInput,
}

impl Display for RnaCodonDecodingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RnaCodonDecodingError::InvalidLength { expected, actual } => {
                write!(f, "Invalid length: {} (should be {})", actual, expected)
            }
            RnaCodonDecodingError::InvalidInput => write!(f, "Invalid input"),
        }
    }
}

impl Error for RnaCodonDecodingError {}

#[derive(Debug, PartialEq, Eq)]
pub enum RnaCodon {
    Ala,
    Arg,
    Asn,
    Asp,
    Cys,
    Gln,
    Glu,
    Gly,
    His,
    Ile,
    Leu,
    Lys,
    Met,
    Phe,
    Pro,
    Ser,
    Ter,
    Thr,
    Trp,
    Tyr,
    Val,
}

pub struct RnaCodon3(RnaNucleoBase, RnaNucleoBase, RnaNucleoBase);

pub struct AnnotatedRnaCodon {
    codon: RnaCodon,
    bases: RnaCodon3,
}

impl RnaCodon {
    pub fn to_letter(&self) -> char {
        match self {
            Self::Ala => 'A',
            Self::Arg => 'R',
            Self::Asn => 'N',
            Self::Asp => 'D',
            Self::Cys => 'C',
            Self::Gln => 'Q',
            Self::Glu => 'E',
            Self::Gly => 'G',
            Self::His => 'H',
            Self::Ile => 'I',
            Self::Leu => 'L',
            Self::Lys => 'K',
            Self::Met => 'M',
            Self::Phe => 'F',
            Self::Pro => 'P',
            Self::Ser => 'S',
            Self::Ter => '*',
            Self::Thr => 'T',
            Self::Trp => 'W',
            Self::Tyr => 'Y',
            Self::Val => 'V',
        }
    }
}

impl ToString for RnaCodon {
    fn to_string(&self) -> String {
        match self {
            Self::Ala => "Alanine",
            Self::Arg => "Arginine",
            Self::Asn => "Asparagine",
            Self::Asp => "Aspartic acid",
            Self::Cys => "Cysteine",
            Self::Gln => "Glutamine",
            Self::Glu => "Glutamic acid",
            Self::Gly => "Glycine",
            Self::His => "Histidine",
            Self::Ile => "Isoleucine",
            Self::Leu => "Leucine",
            Self::Lys => "Lysine",
            Self::Met => "Methionine",
            Self::Phe => "Phenylalanine",
            Self::Pro => "Proline",
            Self::Ser => "Serine",
            Self::Ter => "STOP",
            Self::Thr => "Threonine",
            Self::Trp => "Tryptophan",
            Self::Tyr => "Tyrosine",
            Self::Val => "Valine",
        }
        .to_string()
    }
}

impl<'a> TryFrom<&'a str> for RnaCodon {
    type Error = RnaCodonDecodingError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let chars = value.chars().collect::<Vec<_>>();
        if chars.len() != 3 {
            return Err(RnaCodonDecodingError::InvalidLength {
                expected: 3,
                actual: value.len(),
            });
        }
        let nucleobase_results = chars
            .iter()
            .map(|c| RnaNucleoBase::try_from(*c))
            .collect::<Vec<_>>();
        let invalid_code_count = nucleobase_results.iter().filter(|r| r.is_err()).count();
        if invalid_code_count > 0 {
            return Err(RnaCodonDecodingError::InvalidInput);
        }
        let nucleobase_codes: [RnaNucleoBase; 3] = nucleobase_results
            .into_iter()
            .map(|r| r.ok().unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let codon = RnaCodonBuilder::new(
            Some(nucleobase_codes[0]),
            Some(nucleobase_codes[1]),
            Some(nucleobase_codes[2]),
        )
        .to_codon()
        .unwrap();
        Ok(codon)
    }
}

#[cfg(test)]
mod tests {
    use super::RnaCodon;

    #[test]
    fn test_rna_codon_from_string() {
        let result = RnaCodon::try_from("AUG");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), RnaCodon::Met);
    }

    #[test]
    fn test_rna_codon_from_string_psi() {
        let result = RnaCodon::try_from("AΨG");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), RnaCodon::Met);
    }
}
