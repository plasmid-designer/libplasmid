use super::SequenceItem;

#[derive(serde::Serialize)]
pub struct SequenceData {
    pub sequence: Option<Vec<SequenceItem>>,
    pub bp_count: usize,
    pub cursor: CursorData,
    pub selection: Option<SelectionData>,
}

#[derive(serde::Serialize)]
pub struct CursorData {
    pub position: usize,
    pub is_at_end: bool,
}

#[derive(serde::Serialize)]
pub struct SelectionData {
    pub start: usize,
    pub end: usize,
}

impl From<&crate::state::Selection> for SelectionData {
    fn from(selection: &crate::state::Selection) -> Self {
        SelectionData {
            start: selection.start,
            end: selection.end,
        }
    }
}
