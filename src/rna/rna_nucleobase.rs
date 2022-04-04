use std::fmt::Display;

use crate::traits::*;

/// RNA Nucleobase
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RnaNucleoBase {
    /// Adenine
    A,
    /// Cytosine
    C,
    /// Guanine
    G,
    /// Uracil
    U,
}

impl NucleoBase for RnaNucleoBase {
    fn bonding_partner(&self) -> RnaNucleoBase {
        use self::RnaNucleoBase::*;
        match self {
            A => U,
            U => A,
            C => G,
            G => C,
        }
    }
}

impl Display for RnaNucleoBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use self::RnaNucleoBase::*;
        match self {
            A => write!(f, "Adenine"),
            C => write!(f, "Cytosine"),
            G => write!(f, "Guanine"),
            U => write!(f, "Uracil"),
        }
    }
}

impl TryFromLetter for RnaNucleoBase {
    fn try_from_letter(c: char) -> Option<Self> {
        use self::RnaNucleoBase::*;
        match c.to_ascii_uppercase() {
            'A' => Some(A),
            'C' => Some(C),
            'G' => Some(G),
            'U' => Some(U),
            'Ψ' => Some(U), // Pseudouridine
            _ => None,
        }
    }
}

impl ToLetter for RnaNucleoBase {
    fn to_letter(&self) -> char {
        use self::RnaNucleoBase::*;
        match self {
            A => 'A',
            C => 'C',
            G => 'G',
            U => 'U',
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RnaNucleoBase;

    #[test]
    fn test_rna_base_to_string() {
        assert_eq!(RnaNucleoBase::U.to_string(), format!("Uracil"))
    }
}
