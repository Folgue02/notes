use std::env::{args, var};
use std::fs;

// Modules
mod commands;
mod notes;
// TODO: Delegate functionality from main.rs::main() to another function (run())
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

    let mut nts: notes::Notes = notes::Notes::new(note_file.clone()).unwrap_or_else(|_| {
        fs::File::create(note_file.clone());
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
