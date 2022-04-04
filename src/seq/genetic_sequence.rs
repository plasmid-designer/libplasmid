use std::marker::PhantomData;

use crate::traits::*;

pub struct GeneticSequence<B, C>
where
    B: NucleoBase,
    C: Codon<B>,
{
    sequence: Vec<B>,
    phantom: PhantomData<C>,
}

impl<B, C> GeneticSequence<B, C>
where
    B: NucleoBase + TryFromLetter + Copy,
    C: Codon<B>,
{
    pub fn new() -> Self {
        Self {
            sequence: Vec::new(),
            phantom: PhantomData,
        }
    }

    pub fn from_str<T>(s: T) -> Result<GeneticSequence<B, C>, ()>
    where
        T: AsRef<str>,
    {
        let mut builder = GeneticSequence::<B, C>::new();
        builder.load_str(s).ok_or(())?;
        Ok(builder)
    }
}

impl<B, C> GeneticSequence<B, C>
where
    B: NucleoBase + TryFromLetter + Copy,
    C: Codon<B> + Sized,
{
    pub fn load_str<T>(&mut self, s: T) -> Option<()>
    where
        T: AsRef<str>,
    {
        for c in s.as_ref().chars() {
            let base = B::try_from_letter(c)?;
            self.add_base(base)
        }
        Some(())
    }

    pub fn add_base(&mut self, base: B) {
        self.sequence.push(base)
    }

    pub fn codons(&self) -> impl Iterator<Item = C> + '_ {
        self.sequence
            .chunks_exact(3)
            .map(|chunk| C::from_triplet((chunk[0], chunk[1], chunk[2])))
    }

    pub fn as_codons(&self) -> Vec<C> {
        self.codons().collect()
    }

    // TODO: Find some way to do this
    // pub fn codons(&self) -> (Vec<C>, Option<RnaPartialCodon>) {
    //     let iter = self.sequence.chunks_exact(3);
    //     let extra_bases = iter.remainder();
    //     let codons = iter
    //         .map(|chunk| RnaCodon(chunk[0], chunk[1], chunk[2]))
    //         .collect::<Vec<_>>();
    //     if extra_bases.len() == 0 {
    //         (codons, None)
    //     } else {
    //         (codons, Some(RnaPartialCodon::from_slice(extra_bases)))
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use crate::rna::RnaCodon;
    use crate::seq::RnaSequence;

    #[test]
    fn test_rna_sequence_from_str() {
        use crate::rna::RnaNucleoBase::*;

        let seq = RnaSequence::from_str("AUGUGAUGAAAGCAUAUGACUAAA");
        assert!(seq.is_ok());
        let seq = seq.unwrap();
        let codons = seq.as_codons();
        assert_eq!(
            codons,
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
    }
}
