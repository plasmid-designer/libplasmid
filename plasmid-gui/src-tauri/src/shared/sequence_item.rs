#[derive(serde::Serialize)]
pub struct SequenceItem {
    pub codon: Vec<char>,
    pub anticodon: Vec<char>,
    pub peptide: Option<char>,
    pub start_index: usize,
    pub end_index: usize,
}
