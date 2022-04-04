use super::{rna_base::RnaNucleoBaseDecodingError, RnaCodon, RnaCodonHelper, RnaNucleoBase};

pub struct RnaSequenceBuilder {
    sequence: Vec<RnaNucleoBase>,
}

impl RnaSequenceBuilder {
    pub fn new() -> Self {
        Self {
            sequence: Vec::new(),
        }
    }

    pub fn load_str<T>(&mut self, s: T) -> Result<(), RnaNucleoBaseDecodingError>
    where
        T: AsRef<str>,
    {
        for c in s.as_ref().chars() {
            let base = RnaNucleoBase::try_from(c);
            match base {
                Ok(base) => self.add_base(base),
                Err(err) => return Err(err),
            }
        }
        Ok(())
    }

    pub fn from_str<T>(s: T) -> Result<RnaSequenceBuilder, RnaNucleoBaseDecodingError>
    where
        T: AsRef<str>,
    {
        let mut builder = RnaSequenceBuilder::new();
        match builder.load_str(s) {
            Ok(_) => Ok(builder),
            Err(err) => Err(err),
        }
    }

    pub fn add_base(&mut self, base: RnaNucleoBase) {
        self.sequence.push(base)
    }

    pub fn codons(&self) -> Vec<RnaCodon> {
        self.sequence
            .chunks_exact(3)
            .map(|chunk| RnaCodonHelper::to_codon(chunk[0], chunk[1], chunk[2]))
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use crate::rna::RnaCodon;

    use super::RnaSequenceBuilder;

    #[test]
    fn test_rna_sequence_builder() {
        let builder = RnaSequenceBuilder::from_str("AUGUGAUGAAAGCAUAUGACUAAA");
        assert!(builder.is_ok());
        let builder = builder.unwrap();
        let codons = builder.codons();
        assert_eq!(
            codons,
            &[
                RnaCodon::Met,
                RnaCodon::Ter,
                RnaCodon::Ter,
                RnaCodon::Lys,
                RnaCodon::His,
                RnaCodon::Met,
                RnaCodon::Thr,
                RnaCodon::Lys
            ]
        );
    }
}
