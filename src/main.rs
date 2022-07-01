use std::env;
use std::fs;
// Modules
mod commands;
mod notes;


fn main() {
    let arguments: Vec<String> = env::args().collect();
    let note_file = notes::Notes::get_note_file_path();
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
