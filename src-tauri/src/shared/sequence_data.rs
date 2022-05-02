use super::SequenceItem;

#[derive(serde::Serialize)]
pub struct SequenceData {
    pub sequence: Vec<SequenceItem>,
    pub bp_count: usize,
    pub cursor: CursorData,
}

#[derive(serde::Serialize)]
pub struct CursorData {
    pub position: usize,
    pub is_at_end: bool,
}
