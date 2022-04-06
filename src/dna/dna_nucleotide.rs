use std::fmt::Display;

use crate::traits::*;

/// RNA Nucleobase
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DnaNucleotide {
    /// Adenine
    A,
    /// Cytosine
    C,
    /// Guanine
    G,
    /// Thymine
    T,
}

impl Nucleotide for DnaNucleotide {
    fn bonding_partner(&self) -> Self {
        use self::DnaNucleotide::*;
        match self {
            A => T,
            T => A,
            C => G,
            G => C,
        }
    }
}

impl Display for DnaNucleotide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use self::DnaNucleotide::*;
        match self {
            A => write!(f, "Adenine"),
            C => write!(f, "Cytosine"),
            G => write!(f, "Guanine"),
            T => write!(f, "Thymine"),
        }
    }
}

impl TryFromLetter for DnaNucleotide {
    fn try_from_letter(c: char) -> Option<Self> {
        use self::DnaNucleotide::*;
        match c.to_ascii_uppercase() {
            'A' => Some(A),
            'C' => Some(C),
            'G' => Some(G),
            'T' => Some(T),
            _ => None,
        }
    }
}

impl ToLetter for DnaNucleotide {
    fn to_letter(&self) -> char {
        use self::DnaNucleotide::*;
        match self {
            A => 'A',
            C => 'C',
            G => 'G',
            T => 'T',
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DnaNucleotide;

    #[test]
    fn test_dna_base_to_string() {
        assert_eq!(DnaNucleotide::T.to_string(), format!("Thymine"))
    }
}
