use super::GeneticSequence;
use crate::rna::{RnaCodon, RnaNucleoBase};

/// Dynamic RNA sequence
pub type RnaSequence = GeneticSequence<RnaNucleoBase, RnaCodon>;
