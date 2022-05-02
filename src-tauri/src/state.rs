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

pub struct SequenceState {
    pub cursor_pos: usize,
    pub sequence: VecDeque<IupacNucleotide>,
    pub codons: Vec<DisplayCodon>,
}

impl SequenceState {
    fn inner_insert_nucleotide(&mut self, nucleotide: IupacNucleotide) {
        match self.cursor_pos {
            0 => self.sequence.push_front(nucleotide),
            i if i == self.sequence.len() => self.sequence.push_back(nucleotide),
            i => self.sequence.insert(i, nucleotide),
        }
        self.move_cursor(CursorMovement::By(1));
    }

    pub fn insert(&mut self, letter: char) {
        if let Ok(nucleotide) = IupacNucleotide::try_from_letter(letter) {
            self.inner_insert_nucleotide(nucleotide);
        }
    }

    pub fn insert_all(&mut self, text: String) {
        if let Ok(nucleotides) = text.chars().map(|c| IupacNucleotide::try_from_letter(c)).collect::<Result<Vec<_>, _>>() {
            for nucleotide in nucleotides {
                self.inner_insert_nucleotide(nucleotide);
            }
        }
    }

    pub fn delete(&mut self) {
        match self.cursor_pos {
            0 => (),
            i if i == self.sequence.len() => {
                _ = self.sequence.pop_back()
            }
            i => {
                _ = self.sequence.remove(i)
            }
        }
        self.move_cursor(CursorMovement::By(-1))
    }

    pub fn move_cursor(&mut self, movement: CursorMovement) {
        match movement {
            CursorMovement::To(index) => {
                if index <= self.sequence.len() {
                    self.cursor_pos = index;
                }
            },
            CursorMovement::By(distance) => {
                if distance.is_negative() {
                    self.cursor_pos = self.cursor_pos.saturating_sub(distance.abs() as usize);
                } else {
                    self.cursor_pos = self.cursor_pos.saturating_add(distance as usize);
                }
            },
            CursorMovement::Start => {
                self.cursor_pos = 0;
            },
            CursorMovement::End => {
                self.cursor_pos = self.sequence.len();
            },
            CursorMovement::CodonStart => {
                let distance = if self.cursor_pos % 3 == 0 { 3 } else { self.cursor_pos % 3 };
                self.cursor_pos = self.cursor_pos.saturating_sub(distance);
            },
            CursorMovement::CodonEnd => {
                self.cursor_pos = self.cursor_pos.saturating_add(3 - self.cursor_pos % 3);
            },
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
            display_codons.push(DisplayCodon::new(&chunk));
            display_codons
        }
    }
}

impl Default for SequenceState {
    fn default() -> Self {
        Self {
            cursor_pos: 0,
            sequence: VecDeque::new(),
            codons: Vec::new(),
        }
    }
}
