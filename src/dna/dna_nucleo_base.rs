use std::fmt::Display;

use crate::traits::*;

/// RNA Nucleobase
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DnaNucleoBase {
    /// Adenine
    A,
    /// Cytosine
    C,
    /// Guanine
    G,
    /// Thymine
    T,
}

impl Display for DnaNucleoBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use self::DnaNucleoBase::*;
        match self {
            A => write!(f, "Adenine"),
            C => write!(f, "Cytosine"),
            G => write!(f, "Guanine"),
            T => write!(f, "Thymine"),
        }
    }
}

impl TryFromLetter for DnaNucleoBase {
    fn try_from_letter(c: char) -> Option<Self> {
        use self::DnaNucleoBase::*;
        match c.to_ascii_uppercase() {
            'A' => Some(A),
            'C' => Some(C),
            'G' => Some(G),
            'T' => Some(T),
            _ => None,
        }
    }
}

impl ToLetter for DnaNucleoBase {
    fn to_letter(&self) -> char {
        use self::DnaNucleoBase::*;
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
    use super::DnaNucleoBase;

    #[test]
    fn test_dna_base_to_string() {
        assert_eq!(DnaNucleoBase::T.to_string(), format!("Thymine"))
    }
}
