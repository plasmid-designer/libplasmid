use crate::traits::*;

use super::DnaNucleoBase;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DnaCodon {
    triplet: [DnaNucleoBase; 3],
}

impl_codon_traits!(DnaNucleoBase => DnaCodon);

#[cfg(test)]
mod tests {
    use super::DnaCodon;
    use super::DnaNucleoBase::*;
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
