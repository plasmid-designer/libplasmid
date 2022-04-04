use crate::traits::*;

use super::RnaNucleoBase;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RnaCodon(pub RnaNucleoBase, pub RnaNucleoBase, pub RnaNucleoBase);

impl Codon<RnaNucleoBase> for RnaCodon {
    fn from_triplet(triplet: (RnaNucleoBase, RnaNucleoBase, RnaNucleoBase)) -> RnaCodon {
        RnaCodon(triplet.0, triplet.1, triplet.2)
    }

    fn anticodon(&self) -> RnaCodon {
        todo!()
    }
}

impl<T> TryFromStr<'_, T> for RnaCodon
where
    T: AsRef<str>,
{
    fn try_from_str(value: T) -> Option<Self> {
        let chars = value.as_ref().chars().collect::<Vec<_>>();
        if chars.len() != 3 {
            return None;
        }
        let mut nucleobase_codes: Vec<RnaNucleoBase> = Vec::with_capacity(3);
        for char in chars {
            nucleobase_codes.push(RnaNucleoBase::try_from_letter(char)?)
        }
        let mut iter = nucleobase_codes.into_iter();
        Some(RnaCodon(iter.next()?, iter.next()?, iter.next()?))
    }
}

impl ToString for RnaCodon {
    fn to_string(&self) -> String {
        [self.0.to_letter(), self.1.to_letter(), self.2.to_letter()]
            .iter()
            .collect::<String>()
    }
}
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
            assert_eq!(codon, RnaCodon(A, U, G));
        }
    }

    #[test]
    fn test_rna_codon_from_string_containing_psi() {
        let result = RnaCodon::try_from_str("AΨG");
        assert!(result.is_some());
        if let Some(codon) = result {
            assert_eq!(codon, RnaCodon(A, U, G));
        }
    }

    #[test]
    fn test_rna_codon_to_string() {
        let codon = RnaCodon(A, U, G);
        assert_eq!(codon.to_string(), "AUG");
    }
}
