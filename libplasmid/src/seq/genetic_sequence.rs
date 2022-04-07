use std::{
    borrow::Borrow,
    collections::VecDeque,
    marker::PhantomData,
    ops::{Index, IndexMut},
};

use crate::{prelude::RestrictionEnzymes, traits::*};

use super::Annotation;

pub struct GeneticSequence<B, C>
where
    B: Nucleotide,
    C: Codon<B>,
{
    sequence: Vec<B>,
    annotations: Vec<Annotation>,
    phantom: PhantomData<C>,
}

impl<B, C> GeneticSequence<B, C>
where
    B: Nucleotide + TryFromLetter + ToLetter + ToIupac + Copy,
    C: Codon<B>,
{
    pub fn new() -> Self {
        Self {
            sequence: Vec::new(),
            annotations: Vec::new(),
            phantom: PhantomData,
        }
    }

    pub fn from_str<T>(s: T) -> anyhow::Result<GeneticSequence<B, C>>
    where
        T: AsRef<str>,
    {
        let mut builder = GeneticSequence::<B, C>::new();
        builder.push_base_str(s)?;
        Ok(builder)
    }
}

impl<B, C> GeneticSequence<B, C>
where
    B: Nucleotide + TryFromLetter + ToLetter + ToIupac + Copy,
    C: Codon<B> + Sized,
{
    /// Append a nucleobase to the end of the sequence.
    ///
    /// # Examples
    /// ```rust
    /// use plasmid::{seq::DnaSequence, dna::DnaNucleotide::*};
    ///
    /// let mut seq = DnaSequence::new();
    /// seq.push_base(A);
    ///
    /// assert_eq!(seq[0], A);
    /// ```
    pub fn push_base<T>(&mut self, base: T)
    where
        T: Borrow<B>,
    {
        self.sequence.push(*base.borrow())
    }

    /// Append a nucleobase string to the end of the sequence.
    ///
    /// # Examples
    /// ```rust
    /// use plasmid::{seq::DnaSequence, dna::DnaNucleotide::*};
    ///
    /// let mut seq = DnaSequence::new();
    /// seq.push_base_str("AGT");
    ///
    /// assert_eq!(seq.to_string(), "AGT");
    /// ```
    pub fn push_base_str<T>(&mut self, s: T) -> anyhow::Result<()>
    where
        T: AsRef<str>,
    {
        for c in s.as_ref().chars() {
            let base = B::try_from_letter(c)?;
            self.push_base(base)
        }
        Ok(())
    }

    /// Append a codon to the end of the sequence.
    ///
    /// # Examples
    /// ```rust
    /// use plasmid::{seq::DnaSequence, dna::{DnaCodon, DnaNucleotide::*}};
    ///
    /// let mut seq = DnaSequence::new();
    /// let codon: DnaCodon = [A, G, T].into();
    /// seq.push_codon(&codon);
    ///
    /// assert_eq!(seq.pop_codon(), Some([A, G, T].into()));
    /// ```
    pub fn push_codon<T>(&mut self, codon: T)
    where
        T: Borrow<C>,
    {
        self.sequence.extend(codon.borrow().to_triplet_arr())
    }

    /// Remove the last nucleobase from the sequence and return it, or None if it is empty.
    ///
    /// # Examples
    /// ```rust
    /// use plasmid::{seq::DnaSequence, dna::{DnaCodon, DnaNucleotide::*}};
    ///
    /// let mut seq = DnaSequence::from_str("AGTCCT").unwrap();
    /// let base = seq.pop_base().unwrap();
    ///
    /// assert_eq!(base, T);
    /// ```
    pub fn pop_base(&mut self) -> Option<B> {
        self.sequence.pop()
    }

    /// Remove the last codon from the sequence and return it, or None if it is empty.
    /// The function will return the last proper codon in the sequence.
    ///
    /// Use `#pop_codon_unsafe` if you need a codon from the last three nucleobases.
    ///
    /// # Examples
    /// ```rust
    /// use plasmid::{seq::DnaSequence, dna::DnaNucleotide::*};
    ///
    /// let mut seq1 = DnaSequence::from_str("AGTAA").unwrap();
    /// let seq1_codon = seq1.pop_codon().unwrap(); // AGT
    ///
    /// assert_eq!(seq1_codon, [A, G, T].into());
    ///
    /// let mut seq2 = DnaSequence::from_str("AA").unwrap();
    /// let seq2_codon = seq2.pop_codon(); // None
    ///
    /// assert!(seq2_codon.is_none());
    /// ```
    pub fn pop_codon(&mut self) -> Option<C> {
        self.codons().last()
    }

    /// Remove the last codon from the sequence and return it, or None if it is empty.
    /// This function will build a codon from the last nucleotide triplet.
    ///
    /// # Examples
    /// ```rust
    /// use plasmid::{seq::DnaSequence, dna::DnaNucleotide::*};
    ///
    /// let mut seq1 = DnaSequence::from_str("AGTAA").unwrap();
    /// let seq1_codon = seq1.pop_codon_unsafe().unwrap(); // TAA
    ///
    /// assert_eq!(seq1_codon, [T, A, A].into());
    ///
    /// let mut seq2 = DnaSequence::from_str("AA").unwrap();
    /// let seq2_codon = seq2.pop_codon_unsafe(); // None
    ///
    /// assert!(seq2_codon.is_none());
    /// ```
    pub fn pop_codon_unsafe(&mut self) -> Option<C> {
        let seq: [B; 3] = self
            .sequence
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

    /// An iterator over the nucleotides of a genetic sequence.
    ///
    /// # Examples
    /// ```
    /// use plasmid::{seq::DnaSequence, dna::DnaNucleotide::*};
    ///
    /// let seq = DnaSequence::from_str("TGATCC").unwrap();
    /// let nucleotides = seq.nucleotides().map(|&x| x).collect::<Vec<_>>();
    ///
    /// assert_eq!(nucleotides, [T, G, A, T, C, C])
    /// ```
    pub fn nucleotides(&self) -> std::slice::Iter<B> {
        self.sequence.iter()
    }

    /// An iterator over the codons of a genetic sequence.
    ///
    /// # Examples
    /// ```rust
    /// use plasmid::seq::DnaSequence;
    ///
    /// let seq = DnaSequence::from_str("TGATCC").unwrap();
    /// for codon in seq.codons() {
    ///     println!("{:?}", codon);
    /// }
    /// ```
    pub fn codons(&self) -> impl Iterator<Item = C> + '_ {
        self.sequence
            .chunks_exact(3)
            .map(|chunk| C::from_triplet_arr(chunk.try_into().unwrap()))
    }

    /// An iterator over the nucleotides of a genetic sequence.
    ///
    /// # Examples
    /// ```
    /// use plasmid::{seq::DnaSequence, dna::DnaNucleotide::*};
    ///
    /// let seq = DnaSequence::from_str("TGATCC").unwrap();
    /// let nucleotides = seq.as_nucleotides();
    ///
    /// assert_eq!(nucleotides, [T, G, A, T, C, C])
    /// ```
    pub fn as_nucleotides(&self) -> &[B] {
        &self.sequence
    }

    /// Convert a genetic sequence to a Vec of its anti-nucleotides.
    ///
    /// # Examples
    /// ```
    /// use plasmid::{seq::DnaSequence, dna::DnaNucleotide::*};
    ///
    /// let seq = DnaSequence::from_str("TGATCC").unwrap();
    /// let nucleotides = seq.as_reverse_complement();
    ///
    /// assert_eq!(nucleotides, [A, C, T, A, G, G])
    /// ```
    pub fn as_reverse_complement(&self) -> Vec<B> {
        self.sequence
            .iter()
            .map(|b| b.complement())
            .collect::<Vec<_>>()
    }

    /// Convert a genetic sequence to a Vec of its codons.
    ///
    /// # Examples
    /// ```rust
    /// use plasmid::{seq::DnaSequence, dna::DnaNucleotide::*};
    ///
    /// let seq = DnaSequence::from_str("TGATCC").unwrap();
    ///
    /// assert_eq!(seq.as_codons(), [[T, G, A].into(), [T, C, C].into()])
    /// ```
    pub fn as_codons(&self) -> Vec<C> {
        self.codons().collect()
    }

    pub fn to_nucleotide_string(&self) -> String {
        self.as_nucleotides()
            .iter()
            .map(|b| b.to_letter())
            .collect()
    }

    pub fn to_reverse_complement_string(&self) -> String {
        self.as_reverse_complement()
            .iter()
            .map(|b| b.to_letter())
            .collect()
    }

    pub fn annotate_restriction_enzymes(&mut self) {
        let own_nucleotides = self.as_nucleotides();
        let mut annotations: Vec<Annotation> = Vec::new();
        for enzyme in RestrictionEnzymes.iter() {
            let mut enzyme_seq = enzyme.before.clone();
            enzyme_seq.extend(&enzyme.after);
            let mut nucleotides: VecDeque<&B> = VecDeque::new();
            for (own_index, own_nucleotide) in own_nucleotides.iter().enumerate() {
                if nucleotides.len() == enzyme_seq.len() {
                    nucleotides.pop_front();
                }
                nucleotides.push_back(own_nucleotide);
                if nucleotides.len() == enzyme_seq.len() {
                    let seq_matches = enzyme_seq
                        .iter()
                        .zip(nucleotides.iter().map(|n| n.to_iupac()))
                        .all(|(n, m)| n.matches(&m));
                    if seq_matches {
                        let needle = own_index - nucleotides.len() + enzyme.before.len();
                        let ann = Annotation::new_from_restriction_enzyme(
                            own_index - nucleotides.len(),
                            own_index,
                            Some(needle),
                            enzyme,
                        );
                        annotations.push(ann);
                    }
                }
            }
        }
        self.annotations.extend(annotations);
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

impl<B, C> ToString for GeneticSequence<B, C>
where
    B: Nucleotide + ToLetter,
    C: Codon<B>,
{
    fn to_string(&self) -> String {
        self.sequence.iter().map(|b| b.to_letter()).collect()
    }
}

impl<B, C> Index<usize> for GeneticSequence<B, C>
where
    B: Nucleotide,
    C: Codon<B>,
{
    type Output = B;

    fn index(&self, index: usize) -> &Self::Output {
        &self.sequence[index]
    }
}

impl<B, C> IndexMut<usize> for GeneticSequence<B, C>
where
    B: Nucleotide,
    C: Codon<B>,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.sequence[index]
    }
}

impl<B, C> NucleotideSequence for GeneticSequence<B, C>
where
    B: Nucleotide + ToIupac,
    C: Codon<B>,
{
    fn matches<T>(&self, seq: &[T]) -> bool
    where
        T: ToIupac,
    {
        self.sequence
            .iter()
            .zip(seq)
            .all(|(a, b)| a.to_iupac().matches(&b.to_iupac()))
    }
}

#[cfg(test)]
mod tests {
    use crate::seq::{DnaSequence, RnaSequence};

    #[test]
    fn test_rna_sequence_from_str() {
        use crate::rna::RnaNucleotide::*;

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

    #[test]
    pub fn test_annotate_restriction_enzymes() {
        let mut seq = DnaSequence::from_str("AAAACATATGAAAA").unwrap();
        seq.annotate_restriction_enzymes();
        assert_eq!(seq.annotations.len(), 1);
        let ann = seq.annotations.first().unwrap();
        assert_eq!(ann.start, 3);
        assert_eq!(ann.needle, Some(5));
        assert_eq!(ann.end, 9);
        assert_eq!(ann.text, "NdeI");
    }
}
