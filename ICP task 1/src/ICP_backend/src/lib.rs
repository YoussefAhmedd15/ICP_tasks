use candid::{CandidType, Deserialize};
use std::collections::HashMap;

#[derive(CandidType, Deserialize, Clone)]
pub struct Note {
    pub id: u64,
    pub title: String,
    pub content: String,
}


static mut NOTES: Option<HashMap<u64, Note>> = None;
static mut NEXT_ID: u64 = 1;

fn get_notes() -> &'static mut HashMap<u64, Note> {
    unsafe {
        if NOTES.is_none() {
            NOTES = Some(HashMap::new());
        }
        NOTES.as_mut().unwrap()
    }
}

fn get_next_id() -> u64 {
    unsafe {
        let id = NEXT_ID;
        NEXT_ID += 1;
        id
    }
}

#[ic_cdk::query]
fn get_all_notes() -> Vec<Note> {
    let notes = get_notes();
    notes.values().cloned().collect()
}

#[ic_cdk::update]
fn create_note(title: String, content: String) -> Note {
    let id = get_next_id();
    let note = Note { id, title, content };
    let notes = get_notes();
    notes.insert(id, note.clone());
    note
}

#[ic_cdk::update]
fn update_note(id: u64, title: String, content: String) -> Option<Note> {
    let notes = get_notes();
    if let Some(note) = notes.get_mut(&id) {
        note.title = title;
        note.content = content;
        Some(note.clone())
    } else {
        None
    }
}

#[ic_cdk::update]
fn delete_note(id: u64) -> bool {
    let notes = get_notes();
    notes.remove(&id).is_some()
}

candid::export_service!();
