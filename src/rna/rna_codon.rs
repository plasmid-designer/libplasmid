use crate::traits::*;

use super::RnaNucleoBase;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RnaCodon {
    triplet: [RnaNucleoBase; 3],
}

impl_codon_traits!(RnaNucleoBase => RnaCodon);

#[cfg(test)]
mod tests {
    use super::RnaCodon;
    use super::RnaNucleoBase::*;
    use crate::traits::*;

    #[test]
    fn test_rna_codon_from_string() {
        let result = RnaCodon::try_from_str("AUG");
        assert!(result.is_some());
        if let Some(codon) = result {
            assert_eq!(codon, RnaCodon::from_triplet_arr([A, U, G]));
        }
    }

    #[test]
    fn test_rna_codon_from_string_psi() {
        let result = RnaCodon::try_from_str("AΨG");
        assert!(result.is_some());
        if let Some(codon) = result {
            assert_eq!(codon, [A, U, G].into());
        }
    }

    #[test]
    fn test_rna_codon_to_string() {
        let codon: RnaCodon = [A, U, G].into();
        assert_eq!(codon.to_string(), "AUG");
    }

    #[test]
    fn test_dna_codon_anticodon() {
        let codon: RnaCodon = [A, U, G].into();
        let anticodon = codon.anticodon();
        assert_eq!(anticodon, [U, A, C].into());
    }
}
