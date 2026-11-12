use chrono::{DateTime, Local};
use colored::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Note {
    pub id: u64,
    pub title: String,
    pub body: String,
    pub tags: Vec<String>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct NoteStore {
    pub notes: Vec<Note>,
    pub next_id: u64,
}

impl NoteStore {
    pub fn storage_path() -> PathBuf {
        let mut path = dirs::home_dir().expect("Cannot find home directory");
        path.push(".notecli");
        path.push("notes.json");
        path
    }

    pub fn load() -> Self {
        let path = Self::storage_path();
        if path.exists() {
            let data = fs::read_to_string(&path).unwrap_or_default();
            serde_json::from_str(&data).unwrap_or_default()
        } else {
            NoteStore::default()
        }
    }

    pub fn save(&self) {
        let path = Self::storage_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("Could not create ~/.notecli directory");
        }
        let data = serde_json::to_string_pretty(self).expect("Serialization failed");
        fs::write(&path, data).expect("Could not write notes file");
    }

    pub fn add(&mut self, title: String, body: String, tags: Vec<String>) -> u64 {
        let id = self.next_id;
        let now = Local::now();
        self.notes.push(Note {
            id,
            title,
            body,
            tags,
            created_at: now,
            updated_at: now,
        });
        self.next_id += 1;
        self.save();
        id
    }

    pub fn list(&self) -> Vec<&Note> {
        let mut notes: Vec<&Note> = self.notes.iter().collect();
        notes.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        notes
    }

    pub fn search(&self, query: &str) -> Vec<&Note> {
        let q = query.to_lowercase();
        self.notes
            .iter()
            .filter(|n| {
                n.title.to_lowercase().contains(&q)
                    || n.body.to_lowercase().contains(&q)
                    || n.tags.iter().any(|t| t.to_lowercase().contains(&q))
            })
            .collect()
    }

    pub fn get(&self, id: u64) -> Option<&Note> {
        self.notes.iter().find(|n| n.id == id)
    }

    pub fn delete(&mut self, id: u64) -> bool {
        let before = self.notes.len();
        self.notes.retain(|n| n.id != id);
        let deleted = self.notes.len() < before;
        if deleted {
            self.save();
        }
        deleted
    }

    pub fn edit(
        &mut self,
        id: u64,
        new_title: Option<String>,
        new_body: Option<String>,
        new_tags: Option<Vec<String>>,
    ) -> bool {
        if let Some(note) = self.notes.iter_mut().find(|n| n.id == id) {
            if let Some(t) = new_title { note.title = t; }
            if let Some(b) = new_body { note.body = b; }
            if let Some(tags) = new_tags { note.tags = tags; }
            note.updated_at = Local::now();
            self.save();
            true
        } else {
            false
        }
    }
}

pub fn print_note_summary(note: &Note) {
    let tags_str = if note.tags.is_empty() {
        String::new()
    } else {
        format!(" [{}]", note.tags.join(", ").cyan())
    };
    println!(
        "{} {} {}{}",
        format!("#{}", note.id).yellow().bold(),
        note.title.white().bold(),
        note.created_at.format("%Y-%m-%d %H:%M").to_string().dimmed(),
        tags_str,
    );
    let preview: String = note.body.chars().take(80).collect();
    let ellipsis = if note.body.len() > 80 { "…" } else { "" };
    println!("   {}{}", preview.dimmed(), ellipsis);
}

pub fn print_note_full(note: &Note) {
    println!("{}", "─".repeat(60).dimmed());
    println!("{} {}", format!("Note #{}", note.id).yellow().bold(), note.title.white().bold());
    println!("Created : {}", note.created_at.format("%Y-%m-%d %H:%M:%S").to_string().dimmed());
    println!("Updated : {}", note.updated_at.format("%Y-%m-%d %H:%M:%S").to_string().dimmed());
    if !note.tags.is_empty() {
        println!("Tags    : {}", note.tags.join(", ").cyan());
    }
    println!("{}", "─".repeat(60).dimmed());
    println!("{}", note.body);
    println!("{}", "─".repeat(60).dimmed());
}
