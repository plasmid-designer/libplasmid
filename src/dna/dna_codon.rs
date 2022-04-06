use crate::traits::*;

use super::DnaNucleotide;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct DnaCodon {
    triplet: [DnaNucleotide; 3],
}

impl_codon_traits!(DnaNucleotide => DnaCodon);

#[cfg(test)]
mod tests {
    use super::DnaCodon;
    use super::DnaNucleotide::*;
    use crate::traits::*;

    #[test]
    fn test_dna_codon_from_string() {
        let result = DnaCodon::try_from_str("ATG");
        assert!(result.is_some());
        if let Some(codon) = result {
            assert_eq!(codon, [A, T, G].into());
        }
    }

    #[test]
    fn test_dna_codon_to_string() {
        let codon: DnaCodon = [A, T, G].into();
        assert_eq!(codon.to_string(), "ATG");
    }

    #[test]
    fn test_dna_codon_anticodon() {
        let codon: DnaCodon = [A, T, G].into();
        let anticodon = codon.anticodon();
        assert_eq!(anticodon, [T, A, C].into());
    }
}
