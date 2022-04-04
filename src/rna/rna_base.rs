use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum RnaNucleoBaseDecodingError {
    InvalidChar(char),
}

impl Display for RnaNucleoBaseDecodingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidChar(c) => write!(f, "Invalid character: {}", c),
        }
    }
}

impl Error for RnaNucleoBaseDecodingError {}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RnaNucleoBase {
    A,
    C,
    G,
    U,
}

impl Display for RnaNucleoBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::A => write!(f, "Adenine"),
            Self::C => write!(f, "Cytosine"),
            Self::G => write!(f, "Guanine"),
            Self::U => write!(f, "Uracil"),
        }
    }
}

impl TryFrom<char> for RnaNucleoBase {
    type Error = RnaNucleoBaseDecodingError;

    fn try_from(c: char) -> Result<RnaNucleoBase, RnaNucleoBaseDecodingError> {
        match c.to_ascii_uppercase() {
            'A' => Ok(Self::A),
            'C' => Ok(Self::C),
            'G' => Ok(Self::G),
            'U' => Ok(Self::U),
            'Ψ' => Ok(Self::U), // Pseudouridine
            _ => Err(RnaNucleoBaseDecodingError::InvalidChar(c)),
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
