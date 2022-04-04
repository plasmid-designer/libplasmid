use super::{RnaCodon, RnaNucleoBase};

macro_rules! rna {
    ($a:ident $b:ident $c:ident) => {
        (RnaNucleoBase::$a, RnaNucleoBase::$b, RnaNucleoBase::$c)
    };
}

pub struct RnaCodonHelper;

impl RnaCodonHelper {
    pub fn to_codon(fst: RnaNucleoBase, snd: RnaNucleoBase, trd: RnaNucleoBase) -> RnaCodon {
        match (fst, snd, trd) {
            rna!(U U U) | rna!(U U C) => RnaCodon::Phe,
            rna!(U U A) | rna!(U U G) | rna!(C U U) | rna!(C U C) | rna!(C U A) | rna!(C U G) => {
                RnaCodon::Leu
            }
            rna!(A U U) | rna!(A U C) | rna!(A U A) => RnaCodon::Ile,
            rna!(A U G) => RnaCodon::Met,
            rna!(G U U) | rna!(G U C) | rna!(G U A) | rna!(G U G) => RnaCodon::Val,
            rna!(U C U) | rna!(U C C) | rna!(U C A) | rna!(U C G) | rna!(A G U) | rna!(A G C) => {
                RnaCodon::Ser
            }
            rna!(C C U) | rna!(C C C) | rna!(C C A) | rna!(C C G) => RnaCodon::Pro,
            rna!(A C U) | rna!(A C C) | rna!(A C A) | rna!(A C G) => RnaCodon::Thr,
            rna!(G C U) | rna!(G C C) | rna!(G C A) | rna!(G C G) => RnaCodon::Ala,
            rna!(U A U) | rna!(U A C) => RnaCodon::Tyr,
            rna!(U A A) => RnaCodon::Ter, // Stop Ochre
            rna!(U A G) => RnaCodon::Ter, // Stop Amber
            rna!(C A U) | rna!(C A C) => RnaCodon::His,
            rna!(C A A) | rna!(C A G) => RnaCodon::Gln,
            rna!(A A U) | rna!(A A C) => RnaCodon::Asn,
            rna!(A A A) | rna!(A A G) => RnaCodon::Lys,
            rna!(G A U) | rna!(G A C) => RnaCodon::Asp,
            rna!(G A A) | rna!(G A G) => RnaCodon::Glu,
            rna!(U G U) | rna!(U G C) => RnaCodon::Cys,
            rna!(U G A) => RnaCodon::Ter, // Stop Opal
            rna!(U G G) => RnaCodon::Trp,
            rna!(C G U) | rna!(C G C) | rna!(C G A) | rna!(C G G) => RnaCodon::Arg,
            rna!(A G A) | rna!(A G G) => RnaCodon::Arg,
            rna!(G G U) | rna!(G G C) | rna!(G G A) | rna!(G G G) => RnaCodon::Gly,
        }
    }
}

pub struct RnaCodonBuilder(
    Option<RnaNucleoBase>,
    Option<RnaNucleoBase>,
    Option<RnaNucleoBase>,
);

impl RnaCodonBuilder {
    pub fn new(
        fst: Option<RnaNucleoBase>,
        snd: Option<RnaNucleoBase>,
        trd: Option<RnaNucleoBase>,
    ) -> Self {
        RnaCodonBuilder(fst, snd, trd)
    }

    pub fn empty() -> Self {
        RnaCodonBuilder(None, None, None)
    }

    pub fn to_codon(&self) -> Option<RnaCodon> {
        if let (Some(fst), Some(snd), Some(trd)) = (&self.0, &self.1, &self.2) {
            Some(RnaCodonHelper::to_codon(*fst, *snd, *trd))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{rna_builder, rna_codon};

    use super::{RnaCodon, RnaCodonBuilder, RnaNucleoBase};

    #[test]
    fn test_rna_codon_builder_empty() {
        let builder = RnaCodonBuilder::empty();
        assert!(builder.to_codon().is_none());
    }

    #[test]
    fn test_rna_codon_builder_partial() {
        let builder = RnaCodonBuilder::new(Some(RnaNucleoBase::A), Some(RnaNucleoBase::U), None);
        assert!(builder.to_codon().is_none());
    }

    #[test]
    fn test_rna_codon_builder_methionine() {
        let builder = RnaCodonBuilder::new(
            Some(RnaNucleoBase::A),
            Some(RnaNucleoBase::U),
            Some(RnaNucleoBase::G),
        );
        assert!(builder.to_codon().is_some());
        assert_eq!(builder.to_codon().unwrap(), RnaCodon::Met);
    }

    #[test]
    fn test_rna_codon_builder_macro_partial_1() {
        let codon = rna_builder!(A);
        assert_eq!(codon.0, Some(RnaNucleoBase::A));
        assert_eq!(codon.1, None);
        assert_eq!(codon.2, None);
    }

    #[test]
    fn test_rna_codon_builder_macro_partial_2() {
        let codon = rna_builder!(A U);
        assert_eq!(codon.0, Some(RnaNucleoBase::A));
        assert_eq!(codon.1, Some(RnaNucleoBase::U));
        assert_eq!(codon.2, None);
    }

    #[test]
    fn test_rna_codon_builder_macro() {
        let codon = rna_builder!(A U G);
        assert_eq!(codon.0, Some(RnaNucleoBase::A));
        assert_eq!(codon.1, Some(RnaNucleoBase::U));
        assert_eq!(codon.2, Some(RnaNucleoBase::G));
    }

    #[test]
    fn test_rna_codon_macro() {
        let codon = rna_codon!(A U G);
        assert!(codon.is_some());
        assert_eq!(codon.unwrap(), RnaCodon::Met);
    }
}
