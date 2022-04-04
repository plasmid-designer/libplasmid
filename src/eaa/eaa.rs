use crate::rna::RnaCodon;
use crate::traits::*;

/// Essential Amino Acid
#[derive(Debug, PartialEq, Eq)]
pub enum Eaa {
    /// Alanine
    Ala,
    /// Arginine
    Arg,
    /// Asparagine
    Asn,
    /// Aspartic Acid
    Asp,
    /// Cysteine
    Cys,
    /// Glutamine
    Gln,
    /// Glutamic Acid
    Glu,
    /// Glycine
    Gly,
    /// Histidine
    His,
    /// Isoleucine
    Ile,
    /// Leucine
    Leu,
    /// Lysine
    Lys,
    /// Methionine
    Met,
    /// Phenylalanine
    Phe,
    /// Proline
    Pro,
    /// Serine
    Ser,
    /// Stop Codon
    Ter,
    /// Threonine
    Thr,
    /// Tryptophan
    Trp,
    /// Tyrosine
    Tyr,
    /// Valine
    Val,
}

impl TryFromLetter for Eaa {
    fn try_from_letter(letter: char) -> Option<Self> {
        use self::Eaa::*;
        match letter.to_ascii_uppercase() {
            'A' => Some(Ala),
            'R' => Some(Arg),
            'N' => Some(Asn),
            'D' => Some(Asp),
            'C' => Some(Cys),
            'Q' => Some(Gln),
            'E' => Some(Glu),
            'G' => Some(Gly),
            'H' => Some(His),
            'I' => Some(Ile),
            'L' => Some(Leu),
            'K' => Some(Lys),
            'M' => Some(Met),
            'F' => Some(Phe),
            'P' => Some(Pro),
            'S' => Some(Ser),
            '*' => Some(Ter),
            'W' => Some(Trp),
            'Y' => Some(Tyr),
            'V' => Some(Val),
            _ => None,
        }
    }
}

impl ToLetter for Eaa {
    fn to_letter(&self) -> char {
        use self::Eaa::*;
        match self {
            Ala => 'A',
            Arg => 'R',
            Asn => 'N',
            Asp => 'D',
            Cys => 'C',
            Gln => 'Q',
            Glu => 'E',
            Gly => 'G',
            His => 'H',
            Ile => 'I',
            Leu => 'L',
            Lys => 'K',
            Met => 'M',
            Phe => 'F',
            Pro => 'P',
            Ser => 'S',
            Ter => '*',
            Thr => 'T',
            Trp => 'W',
            Tyr => 'Y',
            Val => 'V',
        }
    }
}

impl ToString for Eaa {
    fn to_string(&self) -> String {
        use self::Eaa::*;
        match self {
            Ala => "Alanine",
            Arg => "Arginine",
            Asn => "Asparagine",
            Asp => "Aspartic acid",
            Cys => "Cysteine",
            Gln => "Glutamine",
            Glu => "Glutamic acid",
            Gly => "Glycine",
            His => "Histidine",
            Ile => "Isoleucine",
            Leu => "Leucine",
            Lys => "Lysine",
            Met => "Methionine",
            Phe => "Phenylalanine",
            Pro => "Proline",
            Ser => "Serine",
            Ter => "STOP",
            Thr => "Threonine",
            Trp => "Tryptophan",
            Tyr => "Tyrosine",
            Val => "Valine",
        }
        .to_string()
    }
}

impl From<&RnaCodon> for Eaa {
    fn from(rna: &RnaCodon) -> Self {
        use crate::eaa::Eaa::*;
        use crate::rna::RnaNucleoBase::*;
        match rna.to_triplet_arr() {
            [U, U, U] | [U, U, C] => Phe,
            [U, U, A] | [U, U, G] | [C, U, U] | [C, U, C] | [C, U, A] | [C, U, G] => Leu,
            [A, U, U] | [A, U, C] | [A, U, A] => Ile,
            [A, U, G] => Met,
            [G, U, U] | [G, U, C] | [G, U, A] | [G, U, G] => Val,
            [U, C, U] | [U, C, C] | [U, C, A] | [U, C, G] | [A, G, U] | [A, G, C] => Ser,
            [C, C, U] | [C, C, C] | [C, C, A] | [C, C, G] => Pro,
            [A, C, U] | [A, C, C] | [A, C, A] | [A, C, G] => Thr,
            [G, C, U] | [G, C, C] | [G, C, A] | [G, C, G] => Ala,
            [U, A, U] | [U, A, C] => Tyr,
            [U, A, A] => Ter, // Stop Ochre
            [U, A, G] => Ter, // Stop Amber
            [C, A, U] | [C, A, C] => His,
            [C, A, A] | [C, A, G] => Gln,
            [A, A, U] | [A, A, C] => Asn,
            [A, A, A] | [A, A, G] => Lys,
            [G, A, U] | [G, A, C] => Asp,
            [G, A, A] | [G, A, G] => Glu,
            [U, G, U] | [U, G, C] => Cys,
            [U, G, A] => Ter, // Stop Opal
            [U, G, G] => Trp,
            [C, G, U] | [C, G, C] | [C, G, A] | [C, G, G] => Arg,
            [A, G, A] | [A, G, G] => Arg,
            [G, G, U] | [G, G, C] | [G, G, A] | [G, G, G] => Gly,
        }
    }
}

impl<T> TryFromStr<'_, T> for Eaa
where
    T: AsRef<str>,
{
    fn try_from_str(s: T) -> Option<Self> {
        Some(Eaa::from(&RnaCodon::try_from_str(s)?))
    }
}
