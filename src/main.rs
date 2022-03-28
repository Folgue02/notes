use std::env::{args, var};

// Modules
mod commands;
mod notes;

fn main() {
    let arguments: Vec<String> = args().collect();
    let note_file = format!(
        "{}/.notes",
        if cfg!(unix) {
            var("HOME").unwrap()
        } else if cfg!(windows) {
            var("USERPROFILE").unwrap()
        } else {
            panic!("Unknown platform.")
        }
    );
    let mut notes: notes::Notes = notes::Notes::new(note_file.clone()).unwrap_or_else(|_| {
        std::fs::File::create(note_file.clone());
        notes::Notes::new(note_file).unwrap()
    });

    if arguments.len() == 1 {
        commands::display_help();
    } else if arguments.len() == 2 {
        if arguments[1] == "l" {
            commands::list(&mut notes);
        } else {
            commands::display_help();
        }
    } else {
        if arguments[1] == "a" {
            commands::add(&mut notes, &arguments[2..].to_vec().join(" "));
        } else if arguments[1] == "r" {
            commands::remove(&mut notes, &arguments[2..].to_vec().join(" "));
        } else {
            commands::display_help();
        }
    }
    notes.save().unwrap_or_else(|_| {
        println!("Cannot save note file.");
    });
}
