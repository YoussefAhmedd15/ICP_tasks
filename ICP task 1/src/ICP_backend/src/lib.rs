use candid::{CandidType, Deserialize, Principal};
use std::collections::HashMap;

#[derive(CandidType, Deserialize, Clone)]
pub struct Note {
    pub id: u64,
    pub title: String,
    pub content: String,
}

// Store notes per user (Principal)
static mut USER_NOTES: Option<HashMap<Principal, HashMap<u64, Note>>> = None;
static mut NEXT_ID: u64 = 1;

fn get_user_notes() -> &'static mut HashMap<Principal, HashMap<u64, Note>> {
    unsafe {
        if USER_NOTES.is_none() {
            USER_NOTES = Some(HashMap::new());
        }
        USER_NOTES.as_mut().unwrap()
    }
}

fn get_next_id() -> u64 {
    unsafe {
        let id = NEXT_ID;
        NEXT_ID += 1;
        id
    }
}

// Helper function to get the caller's principal
fn caller() -> Principal {
    ic_cdk::api::caller()
}

#[ic_cdk::query]
fn get_all_notes() -> Vec<Note> {
    let user_principal = caller();
    let user_notes_map = get_user_notes();
    
    if let Some(user_notes) = user_notes_map.get(&user_principal) {
        user_notes.values().cloned().collect()
    } else {
        Vec::new()
    }
}

#[ic_cdk::update]
fn create_note(title: String, content: String) -> Note {
    let user_principal = caller();
    let id = get_next_id();
    let note = Note { id, title, content };
    
    let user_notes_map = get_user_notes();
    let user_notes = user_notes_map.entry(user_principal).or_insert_with(HashMap::new);
    user_notes.insert(id, note.clone());
    
    note
}

#[ic_cdk::update]
fn update_note(id: u64, title: String, content: String) -> Option<Note> {
    let user_principal = caller();
    let user_notes_map = get_user_notes();
    
    if let Some(user_notes) = user_notes_map.get_mut(&user_principal) {
        if let Some(note) = user_notes.get_mut(&id) {
            note.title = title;
            note.content = content;
            return Some(note.clone());
        }
    }
    None
}

#[ic_cdk::update]
fn delete_note(id: u64) -> bool {
    let user_principal = caller();
    let user_notes_map = get_user_notes();
    
    if let Some(user_notes) = user_notes_map.get_mut(&user_principal) {
        user_notes.remove(&id).is_some()
    } else {
        false
    }
}

// New function to get the current user's principal
#[ic_cdk::query]
fn whoami() -> Principal {
    caller()
}

candid::export_service!();
