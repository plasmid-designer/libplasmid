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
            self.push_base(base)
        }
        Some(())
    }

    pub fn push_base(&mut self, base: B) {
        self.sequence.push(base)
    }

    /// Remove the last nucleobase from the sequence and return it, or None if it is empty.
    pub fn pop_base(&mut self) -> Option<B> {
        self.sequence.pop()
    }

    pub fn push_codon(&mut self, codon: C) {
        self.sequence.extend(codon.to_triplet_arr())
    }

    /// Remove the last codon from the sequence and return it, or None if it is empty.
    /// The function will return the last proper codon in the sequence.
    /// 
    /// Use `#pop_codon_unsafe` if you need a codon from the last three nucleobases.
    /// 
    /// Example:
    /// ```rust
    /// use plasmid::seq::DnaSequence;
    /// use plasmid::dna::DnaNucleoBase::*;
    /// 
    /// let mut seq1 = DnaSequence::from_str("AGTAA").unwrap();
    /// let seq1_codon = seq1.pop_codon().unwrap(); // AGT
    /// assert_eq!(seq1_codon, [A, G, T].into());
    /// 
    /// let mut seq2 = DnaSequence::from_str("AA").unwrap();
    /// let seq2_codon = seq2.pop_codon(); // None
    /// assert!(seq2_codon.is_none());
    /// ```
    pub fn pop_codon(&mut self) -> Option<C> {
        self.codons().last()
    }

    /// Remove the last codon from the sequence and return it, or None if it is empty.
    /// This function will build a codon from the last nucleotide triplet.
    /// 
    /// ```rust
    /// use plasmid::seq::DnaSequence;
    /// use plasmid::dna::DnaNucleoBase::*;
    /// 
    /// let mut seq1 = DnaSequence::from_str("AGTAA").unwrap();
    /// let seq1_codon = seq1.pop_codon_unsafe().unwrap(); // TAA
    /// assert_eq!(seq1_codon, [T, A, A].into());
    /// 
    /// let mut seq2 = DnaSequence::from_str("AA").unwrap();
    /// let seq2_codon = seq2.pop_codon_unsafe(); // None
    /// assert!(seq2_codon.is_none());
    /// ```
    pub fn pop_codon_unsafe(&mut self) -> Option<C> {
        let seq: [B; 3] = self.sequence
            .iter()
            .rev()
            .take(3)
            .rev()
            .cloned()
            .collect::<Vec<B>>()
            .try_into()
            .ok()?;
        Some(C::from_triplet_arr(seq))
    }

    pub fn codons(&self) -> impl Iterator<Item = C> + '_ {
        self.sequence
            .chunks_exact(3)
            .map(|chunk| C::from_triplet_arr(chunk.try_into().unwrap()))
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
                [A, U, G].into(), // Met
                [U, G, A].into(), // Ter
                [U, G, A].into(), // Ter
                [A, A, G].into(), // Lys
                [C, A, U].into(), // His
                [A, U, G].into(), // Met
                [A, C, U].into(), // Thr
                [A, A, A].into(), // Lys
            ]
        );
    }
}
