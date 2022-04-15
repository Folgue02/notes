use std::env;
use std::fs;
use std::path;
use std::process;
// Modules
mod commands;
mod notes;

const DEFAULT_LOCAL_NOTES: &str = "./.notes";

// TODO: Delegate functionality from main.rs::main() to another function (run())
fn main() {
    let arguments: Vec<String> = env::args().collect();
    // Path to the notes file (default or $NOTES_FILE)
    let note_file = if path::Path::new(DEFAULT_LOCAL_NOTES).exists() {
        DEFAULT_LOCAL_NOTES.to_owned()
    } else {
        match env::var("NOTES_FILE") {
            Ok(path) => path,
            Err(_) => {
                format!(
                    "{}/.notes",
                    if cfg!(windows) {
                        env::var("USERPROFILE").unwrap()
                    } else if cfg!(unix) {
                        env::var("HOME").unwrap()
                    } else {
                        eprintln!("Unknown platform.");
                        process::exit(1);
                    }
                )
            }
        }
    };
    println!("Using {} as notes file.", &note_file);

    let mut nts: notes::Notes = notes::Notes::new(note_file.clone()).unwrap_or_else(|_| {
        fs::File::create(note_file.clone()).unwrap();
        notes::Notes::new(note_file).unwrap()
    });

    match commands::run(&mut nts, arguments[1..].to_vec()) {
        Ok(_) => (),
        Err(error) => {
            eprintln!("{}", format!("An error occurred: {}", error));
            std::process::exit(1);
        }
    };
}
