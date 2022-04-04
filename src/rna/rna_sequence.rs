use super::{RnaCodon, RnaNucleoBase, RnaPartialCodon};
use crate::traits::*;

pub struct RnaSequence {
    sequence: Vec<RnaNucleoBase>,
}

impl RnaSequence {
    pub fn new() -> Self {
        Self {
            sequence: Vec::new(),
        }
    }

    pub fn from_str<T>(s: T) -> Result<RnaSequence, ()>
    where
        T: AsRef<str>,
    {
        let mut builder = RnaSequence::new();
        builder.load_str(s).ok_or(())?;
        Ok(builder)
    }
}

impl RnaSequence {
    pub fn load_str<T>(&mut self, s: T) -> Option<()>
    where
        T: AsRef<str>,
    {
        for c in s.as_ref().chars() {
            let base = RnaNucleoBase::try_from_letter(c)?;
            self.add_base(base)
        }
        Some(())
    }

    pub fn add_base(&mut self, base: RnaNucleoBase) {
        self.sequence.push(base)
    }

    pub fn codons(&self) -> (Vec<RnaCodon>, Option<RnaPartialCodon>) {
        let iter = self.sequence.chunks_exact(3);
        let extra_bases = iter.remainder();
        let codons = iter
            .map(|chunk| RnaCodon(chunk[0], chunk[1], chunk[2]))
            .collect::<Vec<_>>();
        if extra_bases.len() == 0 {
            (codons, None)
        } else {
            (codons, Some(RnaPartialCodon::from_slice(extra_bases)))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::rna::RnaCodon;

    use super::RnaSequence;

    #[test]
    fn test_rna_sequence_from_str() {
        use super::RnaNucleoBase::*;

        let seq = RnaSequence::from_str("AUGUGAUGAAAGCAUAUGACUAAA");
        assert!(seq.is_ok());
        let seq = seq.unwrap();
        let codons = seq.codons();
        assert_eq!(
            codons.0,
            &[
                RnaCodon(A, U, G), // Met
                RnaCodon(U, G, A), // Ter
                RnaCodon(U, G, A), // Ter
                RnaCodon(A, A, G), // Lys
                RnaCodon(C, A, U), // His
                RnaCodon(A, U, G), // Met
                RnaCodon(A, C, U), // Thr
                RnaCodon(A, A, A), // Lys
            ]
        );
        assert_eq!(codons.1, None)
    }
}
