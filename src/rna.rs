pub mod rna_base;
pub mod rna_codon;
pub mod rna_codon_builder;
pub mod rna_sequence_builder;

#[macro_export]
macro_rules! rna_builder {
    ($a:ident) => {
        crate::rna::RnaCodonBuilder::new(Some(RnaNucleoBase::$a), None, None)
    };
    ($a:ident $b:ident) => {
        crate::rna::RnaCodonBuilder::new(Some(RnaNucleoBase::$a), Some(RnaNucleoBase::$b), None)
    };
    ($a:ident $b:ident $c:ident) => {
        crate::rna::RnaCodonBuilder::new(
            Some(RnaNucleoBase::$a),
            Some(RnaNucleoBase::$b),
            Some(RnaNucleoBase::$c),
        )
    };
}

#[macro_export]
macro_rules! rna_codon {
    ($a:ident) => { crate::rna_builder!($a).to_codon() };
    ($a:ident $b:ident) => { crate::rna_builder!($a $b).to_codon() };
    ($a:ident $b:ident $c:ident) => { crate::rna_builder!($a $b $c).to_codon() };
}

pub use self::rna_base::RnaNucleoBase;
pub use self::rna_codon::{AnnotatedRnaCodon, RnaCodon, RnaCodon3};
pub use self::rna_codon_builder::{RnaCodonBuilder, RnaCodonHelper};
pub use self::rna_sequence_builder::RnaSequenceBuilder;
