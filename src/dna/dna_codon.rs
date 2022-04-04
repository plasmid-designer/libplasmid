use crate::traits::*;

use super::DnaNucleoBase;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DnaCodon(pub DnaNucleoBase, pub DnaNucleoBase, pub DnaNucleoBase);

impl Codon<DnaNucleoBase> for DnaCodon {
    fn from_triplet(triplet: (DnaNucleoBase, DnaNucleoBase, DnaNucleoBase)) -> DnaCodon {
        DnaCodon(triplet.0, triplet.1, triplet.2)
    }

    fn anticodon(&self) -> DnaCodon {
        todo!()
    }
}

impl<T> TryFromStr<'_, T> for DnaCodon
where
    T: AsRef<str>,
{
    fn try_from_str(value: T) -> Option<Self> {
        let chars = value.as_ref().chars().collect::<Vec<_>>();
        if chars.len() != 3 {
            return None;
        }
        let mut nucleobase_codes: Vec<DnaNucleoBase> = Vec::with_capacity(3);
        for char in chars {
            nucleobase_codes.push(DnaNucleoBase::try_from_letter(char)?)
        }
        let mut iter = nucleobase_codes.into_iter();
        Some(DnaCodon(iter.next()?, iter.next()?, iter.next()?))
    }
}

impl ToString for DnaCodon {
    fn to_string(&self) -> String {
        [self.0.to_letter(), self.1.to_letter(), self.2.to_letter()]
            .iter()
            .collect::<String>()
    }
}
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
            assert_eq!(codon, DnaCodon(A, T, G));
        }
    }

    #[test]
    fn test_dna_codon_to_string() {
        let codon = DnaCodon(A, T, G);
        assert_eq!(codon.to_string(), "ATG");
    }
}
