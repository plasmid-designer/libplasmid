use super::GeneticSequence;
use crate::dna::{DnaCodon, DnaNucleoBase};

/// Dynamic DNA sequence
pub type DnaSequence = GeneticSequence<DnaNucleoBase, DnaCodon>;
