#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use parking_lot::RwLock;

mod state;
use state::{CursorMovement, SelectionMovement, SequenceState};

mod shared;
use shared::{CursorData, SequenceData, SequenceItem};

fn main() {
    tauri::Builder::default()
        .manage(RwLock::new(SequenceState::default()))
        .invoke_handler(tauri::generate_handler![
            calculate_sequence_data,
            sequence_insert,
            sequence_insert_all,
            sequence_delete,
            sequence_delete_next,
            move_cursor,
            move_cursor_left,
            move_cursor_right,
            move_cursor_to_start,
            move_cursor_to_end,
            move_cursor_to_codon_start,
            move_cursor_to_codon_end,
            set_selection,
            set_selection_all,
            reset_selection,
            expand_selection_left,
            expand_selection_right,
            get_selected_sequence,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn sequence_insert(state: tauri::State<RwLock<SequenceState>>, letter: char) {
    state.write().insert(letter)
}

#[tauri::command]
fn sequence_insert_all(state: tauri::State<RwLock<SequenceState>>, text: String) {
    let text_without_whitespace = text.chars().filter(|c| !c.is_whitespace()).collect();
    state.write().insert_all(text_without_whitespace)
}

#[tauri::command]
fn sequence_delete(state: tauri::State<RwLock<SequenceState>>) {
    state.write().delete()
}

#[tauri::command]
fn sequence_delete_next(state: tauri::State<RwLock<SequenceState>>) {
    state.write().delete_next()
}

#[tauri::command]
fn move_cursor(state: tauri::State<RwLock<SequenceState>>, index: usize) {
    state.write().move_cursor(CursorMovement::To(index))
}

#[tauri::command]
fn move_cursor_left(state: tauri::State<RwLock<SequenceState>>) {
    state.write().move_cursor(CursorMovement::By(-1))
}

#[tauri::command]
fn move_cursor_right(state: tauri::State<RwLock<SequenceState>>) {
    state.write().move_cursor(CursorMovement::By(1))
}

#[tauri::command]
fn move_cursor_to_start(state: tauri::State<RwLock<SequenceState>>) {
    state.write().move_cursor(CursorMovement::Start)
}

#[tauri::command]
fn move_cursor_to_end(state: tauri::State<RwLock<SequenceState>>) {
    state.write().move_cursor(CursorMovement::End)
}

#[tauri::command]
fn move_cursor_to_codon_start(state: tauri::State<RwLock<SequenceState>>) {
    state.write().move_cursor(CursorMovement::CodonStart)
}

#[tauri::command]
fn move_cursor_to_codon_end(state: tauri::State<RwLock<SequenceState>>) {
    state.write().move_cursor(CursorMovement::CodonEnd)
}

#[tauri::command]
fn set_selection(state: tauri::State<RwLock<SequenceState>>, start: usize, end: usize) {
    state
        .write()
        .move_selection(SelectionMovement::Set { start, end })
}

#[tauri::command]
fn set_selection_all(state: tauri::State<RwLock<SequenceState>>) {
    state.write().move_selection(SelectionMovement::All)
}

#[tauri::command]
fn reset_selection(state: tauri::State<RwLock<SequenceState>>) {
    state.write().move_selection(SelectionMovement::Reset)
}

#[tauri::command]
fn expand_selection_left(state: tauri::State<RwLock<SequenceState>>) {
    state
        .write()
        .move_selection(SelectionMovement::ExpandBy(-1))
}

#[tauri::command]
fn expand_selection_right(state: tauri::State<RwLock<SequenceState>>) {
    state.write().move_selection(SelectionMovement::ExpandBy(1))
}

#[tauri::command]
fn get_selected_sequence(state: tauri::State<RwLock<SequenceState>>) -> String {
    state.read().get_selected_sequence()
}

#[tauri::command]
fn calculate_sequence_data(
    state: tauri::State<RwLock<SequenceState>>,
    force: bool,
) -> SequenceData {
    let data = {
        if force || state.read().sequence_dirty {
            let mut data: Vec<SequenceItem> = Vec::with_capacity(state.read().codons.len());
            state.write().update();
            for (index, codon) in state.read().codons.iter().enumerate() {
                data.push(SequenceItem {
                    codon: codon.nucleotides.clone(),
                    anticodon: codon.anti_nucleotides.clone(),
                    peptide: codon.peptide,
                    start_index: index * 3,
                })
            }
            Some(data)
        } else {
            None
        }
    };
    let state = state.read();
    SequenceData {
        sequence: data,
        bp_count: state.sequence.len(),
        cursor: CursorData {
            position: state.cursor_pos,
            is_at_end: state.cursor_pos == state.sequence.len(),
        },
        selection: state.selection.as_ref().map(|selection| selection.into()),
    }
}