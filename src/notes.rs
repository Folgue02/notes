use std::fs::{read_to_string, File};
use std::io::Write;

const DEFAULT_LOCAL_NOTES: &str = "./.notes";
pub struct Notes {
    notes: Vec<String>,
    file_path: String,
}

impl Notes {
    pub fn new(file_path: String) -> std::io::Result<Self> {
        let mut result = Vec::new();
        let notes = read_to_string(&file_path.clone())?;
        for line in notes.split("\n") {
            if line.trim() == "" {
                continue;
            }
            result.push(line.to_owned());
        }
        Ok(Self {
            notes: result,
            file_path: file_path,
        })
    }

    /// Pushes a note at the end of the vector
    pub fn add(&mut self, new_note: String) {
        self.notes.push(new_note)
    }

    /// Removes a note associated with the index
    pub fn remove(&mut self, idx: u8) -> Result<(), ()> {
        if idx >= self.notes.len() as u8 {
            return Err(());
        }
        self.notes.remove(idx as usize);
        Ok(())
    }

    /// Saves the content in the notes struct into `self.file_path`
    pub fn save(&mut self) -> std::io::Result<()> {
        let mut target_file = File::create(&self.file_path)?;
        write!(target_file, "{}", self.notes.clone().join("\n"))
    }

    /// Returns a clone of `self.notes`
    pub fn get_vec(&mut self) -> Vec<String> {
        self.notes.clone()
    }

    pub fn len(&self) -> usize {
        self.notes.len()
    }
    
    /// Returns the location of the notes file
    /// This file can be in:
    /// - $(pwd)/.notes
    /// - ~/.notes
    pub fn get_note_file_path() -> String {
        return if std::path::Path::new(DEFAULT_LOCAL_NOTES).exists() {
            DEFAULT_LOCAL_NOTES.to_owned()
        } else {
            match std::env::var("NOTES_FILE") {
                Ok(path) => path,
                Err(_) => {
                    format!(
                        "{}/.notes",
                        if cfg!(windows) {
                            std::env::var("USERPROFILE").unwrap()
                        } else if cfg!(unix) {
                            std::env::var("HOME").unwrap()
                        } else {
                            eprintln!("Unknown platform.");
                            std::process::exit(1);
                        }
                    )
                }
            }
        };
    } 
}

// Traits
impl std::ops::Index<usize> for Notes {
    type Output = String;
    fn index(&self, idx: usize) -> &Self::Output {
        if idx > self.notes.len() {
            panic!(
                "{}",
                format!(
                    "Index ({}) out of bounds, the len of notes is {}",
                    idx,
                    self.notes.len()
                )
            );
        }
        &self.notes[idx]
    }
}

impl std::ops::IndexMut<usize> for Notes {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        if idx > self.notes.len() {
            panic!(
                "{}",
                format!(
                    "Index ({}) out of bounds, the len of notes is {}",
                    idx,
                    self.notes.len()
                )
            );
        }
        &mut self.notes[idx]
    }
}
