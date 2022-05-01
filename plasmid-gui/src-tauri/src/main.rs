#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            calculate_sequence_data,
            sequence_to_triplet_chunks,
            codon_to_peptide,
            codon_to_complement
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(serde::Serialize)]
struct SequenceData {
    codon: String,
    anticodon: String,
    peptide: String,
    start_index: usize,
    end_index: usize,
}

#[tauri::command]
fn calculate_sequence_data(sequence: Vec<String>) -> Vec<SequenceData> {
    let mut data: Vec<SequenceData> = Vec::with_capacity(sequence.len() / 3);
    for (index, triplet) in sequence_to_triplet_chunks(sequence).iter().enumerate() {
        data.push(SequenceData {
            codon: triplet.clone(),
            anticodon: codon_to_complement(triplet.clone()),
            peptide: codon_to_peptide(triplet.clone()),
            start_index: index * 3,
            end_index: index * 3 + triplet.len(),
        })
    }
    data
}

#[tauri::command]
fn sequence_to_triplet_chunks(sequence: Vec<String>) -> Vec<String> {
    let mut chunks = Vec::new();
    for chunk in sequence.chunks(3) {
        chunks.push(chunk.join(""));
    }
    chunks
}

#[tauri::command]
fn codon_to_peptide(codon: String) -> String {
    use plasmid::prelude::*;
    use plasmid::traits::{ToLetter, TryFromStr};
    if let Ok(codon) = DnaCodon::try_from_str(&codon) {
        codon.translate().to_letter().to_string()
    } else {
        String::new()
    }
}

#[tauri::command]
fn codon_to_complement(codon: String) -> String {
    use plasmid::prelude::*;
    use plasmid::traits::{ToLetter, TryFromLetter};
    let mut buf = String::with_capacity(codon.len());
    for nuc in codon.chars() {
        if let Ok(nuc) = DnaNucleotide::try_from_letter(nuc) {
            buf.push(nuc.complement().to_letter());
        }
    }
    buf
}
