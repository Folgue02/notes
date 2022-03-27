use std::fs::{read_to_string, File};
use std::io::Write;

pub struct Notes {
    // add, remove, save, get_vec
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

    pub fn add(&mut self, new_note: String) {
        self.notes.push(new_note)
    }

    pub fn remove(&mut self, idx: u8) -> Result<(), ()> {
        if idx > self.notes.len() as u8 + 1 {
            return Err(());
        }
        self.notes.remove(idx as usize);
        Ok(())
    }

    pub fn save(&mut self) -> std::io::Result<()> {
        let mut target_file = File::create(&self.file_path)?;
        write!(target_file, "{}", self.notes.clone().join("\n"))
    }

    pub fn get_vec(&mut self) -> Vec<String> {
        self.notes.clone()
    }
}
