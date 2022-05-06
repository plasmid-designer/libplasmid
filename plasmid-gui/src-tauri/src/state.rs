use std::collections::VecDeque;

use plasmid::{traits::TryFromLetter, uni::IupacNucleotide};

use crate::shared::DisplayCodon;

pub enum CursorMovement {
    To(usize),
    By(isize),
    Start,
    End,
    CodonStart,
    CodonEnd,
}

pub enum SelectionMovement {
    Set { start: usize, end: usize },
    ExpandBy(isize),
    Reset,
    All,
}

pub struct Selection {
    pub start: usize,
    pub end: usize,
}

impl Selection {
    pub fn clamped(mut self, max_len: usize) -> Self {
        self.start = self.start.min(max_len).max(0);
        self.end = self.end.min(max_len).max(0);
        self
    }
}

#[derive(Default)]
pub struct SequenceState {
    pub sequence_dirty: bool,
    pub cursor_pos: usize,
    pub sequence: VecDeque<IupacNucleotide>,
    pub codons: Vec<DisplayCodon>,
    pub selection: Option<Selection>,
}

impl SequenceState {
    #[inline]
    fn inner_insert_nucleotide(&mut self, nucleotide: IupacNucleotide) {
        match self.cursor_pos {
            0 => self.sequence.push_front(nucleotide),
            i if i == self.sequence.len() => self.sequence.push_back(nucleotide),
            i => self.sequence.insert(i, nucleotide),
        }
        self.move_cursor(CursorMovement::By(1));
    }

    fn inner_insert_multiple_nucleotides(&mut self, nucleotides: &[IupacNucleotide]) {
        let mut vec = self.sequence.iter().cloned().collect::<Vec<_>>();
        vec.splice(
            self.cursor_pos..self.cursor_pos,
            nucleotides.iter().cloned(),
        );
        self.sequence = VecDeque::from(vec);
        self.move_cursor(CursorMovement::By(nucleotides.len() as isize));
    }

    fn inner_move_cursor(&mut self, movement: CursorMovement) {
        match movement {
            CursorMovement::To(index) => {
                if index <= self.sequence.len() {
                    self.cursor_pos = index;
                }
            }
            CursorMovement::By(distance) => {
                if distance.is_negative() {
                    self.cursor_pos = self.cursor_pos.saturating_sub(distance.abs() as usize);
                } else {
                    self.cursor_pos = self
                        .cursor_pos
                        .saturating_add(distance as usize)
                        .min(self.sequence.len());
                }
            }
            CursorMovement::Start => {
                self.cursor_pos = 0;
            }
            CursorMovement::End => {
                self.cursor_pos = self.sequence.len();
            }
            CursorMovement::CodonStart => {
                let distance = if self.cursor_pos % 3 == 0 {
                    3
                } else {
                    self.cursor_pos % 3
                };
                self.cursor_pos = self.cursor_pos.saturating_sub(distance);
            }
            CursorMovement::CodonEnd => {
                self.cursor_pos = self.cursor_pos.saturating_add(3 - self.cursor_pos % 3);
            }
        }
    }

    fn inner_reset_selection(&mut self) {
        self.selection = None;
    }

    fn inner_delete_selection_content(&mut self) {
        if let Some(selection) = &self.selection {
            self.sequence.drain(selection.start..selection.end);
            self.cursor_pos = selection.start.min(self.sequence.len()).max(0);
            self.sequence_dirty = true;
            self.inner_reset_selection();
        }
    }
}

impl SequenceState {
    pub fn insert(&mut self, letter: char) {
        self.inner_delete_selection_content();

        if let Ok(nucleotide) = IupacNucleotide::try_from_letter(letter) {
            self.inner_insert_nucleotide(nucleotide);
            self.sequence_dirty = true;
        }
    }

    pub fn insert_all(&mut self, text: String) {
        self.inner_delete_selection_content();

        if let Ok(nucleotides) = text
            .chars()
            .map(|c| IupacNucleotide::try_from_letter(c))
            .collect::<Result<Vec<_>, _>>()
        {
            self.inner_insert_multiple_nucleotides(&nucleotides);
            self.sequence_dirty = true;
        }
    }

    pub fn delete(&mut self) {
        self.inner_delete_selection_content();

        match self.cursor_pos {
            0 => (),
            1 => _ = self.sequence.pop_front(),
            i if i == self.sequence.len() => _ = self.sequence.pop_back(),
            i => _ = self.sequence.remove(i - 1),
        }
        if self.cursor_pos != 0 {
            self.sequence_dirty = true;
        }
        self.move_cursor(CursorMovement::By(-1));
    }

    pub fn delete_next(&mut self) {
        self.inner_delete_selection_content();

        match self.cursor_pos {
            0 => _ = self.sequence.pop_front(),
            i if i == self.sequence.len() - 1 => _ = self.sequence.pop_back(),
            i if i == self.sequence.len() => (),
            i => _ = self.sequence.remove(i),
        }
        self.sequence_dirty = true;
    }

    pub fn move_cursor(&mut self, movement: CursorMovement) {
        self.inner_move_cursor(movement);
        self.inner_reset_selection();
    }

    pub fn move_selection(&mut self, movement: SelectionMovement) {
        match movement {
            SelectionMovement::Reset => {
                self.selection = None;
            }
            SelectionMovement::Set { start, end } => {
                match start.cmp(&end) {
                    std::cmp::Ordering::Less => {
                        self.selection =
                            Some(Selection { start, end }.clamped(self.sequence.len()));
                    }
                    std::cmp::Ordering::Equal => {
                        self.inner_reset_selection();
                    }
                    std::cmp::Ordering::Greater => {
                        self.selection = Some(
                            Selection {
                                start: end,
                                end: start,
                            }
                            .clamped(self.sequence.len()),
                        );
                    }
                }
                self.inner_move_cursor(CursorMovement::To(end));
            }
            SelectionMovement::All => {
                self.selection = Some(
                    Selection { start: 0, end: self.sequence.len() }
                )
            }
            SelectionMovement::ExpandBy(distance) => {
                let abs_distance = distance.abs() as usize;
                match &self.selection {
                    Some(selection) => {
                        if distance.is_negative() {
                            let start = selection.start.saturating_sub(abs_distance);
                            self.selection = Some(
                                Selection {
                                    start,
                                    end: selection.end,
                                }
                                .clamped(self.sequence.len()),
                            );
                            self.inner_move_cursor(CursorMovement::To(start));
                        } else {
                            let end = selection
                                .end
                                .saturating_add(abs_distance)
                                .min(self.sequence.len());
                            self.selection = Some(
                                Selection {
                                    start: selection.start,
                                    end,
                                }
                                .clamped(self.sequence.len()),
                            );
                            self.inner_move_cursor(CursorMovement::To(end));
                        }
                    }
                    None => {
                        if distance.is_negative() {
                            let start = self
                                .cursor_pos
                                .saturating_sub(abs_distance)
                                .min(self.sequence.len())
                                .max(0);
                            self.selection = Some(
                                Selection {
                                    start,
                                    end: self.cursor_pos,
                                }
                                .clamped(self.sequence.len()),
                            );
                            self.inner_move_cursor(CursorMovement::To(start));
                        } else {
                            let end = self
                                .cursor_pos
                                .saturating_add(abs_distance)
                                .min(self.sequence.len())
                                .max(0);
                            self.selection = Some(
                                Selection {
                                    start: self.cursor_pos,
                                    end,
                                }
                                .clamped(self.sequence.len()),
                            );
                            self.inner_move_cursor(CursorMovement::To(end));
                        }
                    }
                };
            }
        }
    }

    // TODO: This could be heavily optimized by keeping track of "dirty" coding regions
    // and recoding only the changed areas into display codons.
    pub fn update(&mut self) {
        self.codons = {
            let mut display_codons = Vec::new();
            let mut chunk = Vec::with_capacity(3);
            for nucleotide in self.sequence.iter().cloned() {
                chunk.push(nucleotide);
                if chunk.len() == 3 {
                    display_codons.push(DisplayCodon::new(&chunk));
                    chunk.clear();
                }
            }
            if !chunk.is_empty() {
                display_codons.push(DisplayCodon::new(&chunk));
            }
            display_codons
        };
        self.sequence_dirty = false;
    }

    pub fn get_selected_sequence(&self) -> String {
        use plasmid::traits::ToLetter;

        match &self.selection {
            Some(selection) => self.sequence.range(selection.start..selection.end).map(|nuc| nuc.to_letter()).collect(),
            None => String::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{SelectionMovement, SequenceState};

    #[test]
    fn test_insert() {
        use plasmid::prelude::IupacNucleotide::*;

        let mut state = SequenceState::default();
        state.insert('A');
        assert_eq!(state.sequence, [A]);
    }

    #[test]
    fn test_insert_with_selection() {
        use plasmid::prelude::IupacNucleotide::*;

        let mut state = SequenceState::default();
        state.insert('A');
        state.insert('C');
        state.insert('G');
        state.move_selection(SelectionMovement::Set { start: 1, end: 2 });
        state.insert('T');
        assert_eq!(state.sequence, [A, T, G])
    }

    #[test]
    fn test_insert_all() {
        use plasmid::prelude::IupacNucleotide::*;

        let mut state = SequenceState::default();
        state.insert_all("ACGT".to_string());
        assert_eq!(state.sequence, [A, C, G, T]);
    }

    #[test]
    fn test_insert_all_with_selection() {
        use plasmid::prelude::IupacNucleotide::*;

        let mut state = SequenceState::default();
        state.insert_all("ACCCT".to_string());
        state.move_selection(SelectionMovement::Set { start: 1, end: 5 });
        state.insert_all("TG".to_string());
        assert_eq!(state.sequence, [A, T, G]);

        let mut state = SequenceState::default();
        state.insert_all("ACCCT".to_string());
        state.move_selection(SelectionMovement::Set { start: 0, end: 5 });
        state.insert_all("TG".to_string());
        assert_eq!(state.sequence, [T, G]);
    }

    #[test]
    fn test_delete() {
        use plasmid::prelude::IupacNucleotide::*;

        let mut state = SequenceState::default();
        state.insert_all("ACGT".to_string());
        state.delete();
        assert_eq!(state.sequence, [A, C, G]);
        state.delete();
        assert_eq!(state.sequence, [A, C]);
    }

    #[test]
    fn test_delete_with_selection() {
        use plasmid::prelude::IupacNucleotide::*;

        let mut state = SequenceState::default();
        state.insert_all("ACGT".to_string());
        state.move_selection(SelectionMovement::Set { start: 1, end: 3 });
        state.delete();
        assert_eq!(state.sequence, [A, T]);
    }

    #[test]
    fn test_delete_next() {
        use plasmid::prelude::IupacNucleotide::*;

        let mut state = SequenceState::default();
        state.insert_all("ACGT".to_string());
        state.delete_next();
        assert_eq!(state.sequence, [A, C, G, T]);
        state.cursor_pos = 0;
        state.delete_next();
        assert_eq!(state.sequence, [C, G, T]);
        state.delete_next();
        assert_eq!(state.sequence, [G, T]);
    }

    #[test]
    fn test_move_cursor() {
        use super::CursorMovement;

        let mut state = SequenceState::default();
        state.insert_all("ACGT".to_string());

        state.move_cursor(CursorMovement::To(4));
        assert_eq!(state.cursor_pos, 4);

        state.move_cursor(CursorMovement::To(0));
        assert_eq!(state.cursor_pos, 0);

        state.move_cursor(CursorMovement::By(1));
        assert_eq!(state.cursor_pos, 1);

        state.move_cursor(CursorMovement::By(2));
        assert_eq!(state.cursor_pos, 3);

        state.move_cursor(CursorMovement::By(-1));
        assert_eq!(state.cursor_pos, 2);

        state.move_cursor(CursorMovement::CodonStart);
        assert_eq!(state.cursor_pos, 0);

        state.move_cursor(CursorMovement::CodonEnd);
        assert_eq!(state.cursor_pos, 3);

        state.move_cursor(CursorMovement::Start);
        assert_eq!(state.cursor_pos, 0);

        state.move_cursor(CursorMovement::End);
        assert_eq!(state.cursor_pos, 4);
    }
}
