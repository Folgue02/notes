use crate::notes;
use std::io;
use std::io::{ErrorKind, Write};

pub fn run(mut nts: &mut notes::Notes, args: Vec<String>) -> Result<(), &'static str> {
    if args.get(0).is_none() {
        display_help();
    } else {
        // No arguments commands
        if args.len() == 1 {
            match args[0].as_str() {
                "l" => list(&mut nts)?,
                "h" => display_help(),
                "c" => cli(&mut nts)?,
                _ => display_help(),
            }
        } else {
            let cmd_arguments: &Vec<String> = &args[1..].to_vec();
            // Multiple arguments
            match args[0].as_str() {
                "a" => {
                    println!("Adding note to the note file.");
                    add(&mut nts, &cmd_arguments.join(" "))?
                }
                "r" => {
                    remove(&mut nts, &cmd_arguments[0])?;
                    println!("Note removed.");
                }
                "m" => {
                    move_note(&mut nts, &cmd_arguments[0], &cmd_arguments[1])?;
                    println!("Notes moved.");
                }
                _ => display_help(),
            }
        }
    }
    if let Err(error) = nts.save() {
        match error.kind() {
            ErrorKind::PermissionDenied => Err("No rights to write in the notes file."),
            ErrorKind::NotFound => Err("Notes file doesn't exist."),
            _ => Err("Unknown error while trying to save the file."),
        }
    } else {
        Ok(())
    }
}

pub fn add(notes: &mut notes::Notes, args: &String) -> Result<(), &'static str> {
    notes.add(args.clone());
    Ok(())
}

pub fn remove(notes: &mut notes::Notes, args: &String) -> Result<(), &'static str> {
    let index = match args.parse::<u8>() {
        Ok(index) => index - 1,
        Err(_) => {
            return Err("The index you've specified doesn't seem to be a number.");
        }
    };
    match notes.remove(index) {
        Ok(_) => (),
        Err(_) => return Err("The index you've specified its greater than the number of notes."),
    };
    Ok(())
}

pub fn list(notes: &mut notes::Notes) -> Result<(), &'static str> {
    let space = notes.len().to_string().len();
    println!("{}:{}", "-".repeat(space), "-".repeat(15));
    for (idx, note) in notes.get_vec().into_iter().enumerate() {
        println!(
            "{}{}: {}",
            idx + 1,
            " ".repeat(space - (idx + 1).to_string().len()),
            note
        );
    }
    Ok(())
}

pub fn move_note(
    notes: &mut notes::Notes,
    original_index: &String,
    destination_index: &String,
) -> Result<(), &'static str> {
    // Not numbers
    if !is_numeric(&original_index) || !is_numeric(&destination_index) {
        return Err(
            "One or both of the indexes of the notes to move are not specified as a number.",
        );
    }
    let mut destination_index: usize = destination_index.parse::<usize>().unwrap() - 1;
    let original_index: usize = original_index.parse::<usize>().unwrap() - 1;

    if original_index > notes.get_vec().len() {
        return Err(
            "The index of the note to move is greater than the actual number of notes stored.",
        );
    }
    if destination_index > notes.get_vec().len() {
        destination_index = notes.get_vec().len() - 1;
    } // BUG: If value specified under 0, the program will crash
    let original_note = notes[original_index].clone();
    notes[original_index] = notes[destination_index].clone();
    notes[destination_index] = original_note;
    Ok(())
}

fn cli(nts: &mut notes::Notes) -> Result<(), &'static str> {
    println!("Notes Command Line Interface (type 'q' to leave)");
    let mut rl = rustyline::Editor::<()>::new();
    loop {
        let buffer = rl.readline("NCLI: ").unwrap();

        // Quit command
        if buffer == "q" {
            break;
        }
        match run(
            nts,
            buffer
                .split(" ")
                .filter(|slice| slice.trim() != "" || slice.trim() == "\n")
                .map(|slice| String::from(slice.trim()))
                .collect::<Vec<String>>(),
        ) {
            Ok(()) => (),
            Err(error) => {
                eprintln!("An error has occurred: {}", error);
            }
        };
    }
    Ok(())
}

pub fn display_help() {
    println!(
        "
Note taking app:
    a {{note}}
        Appends {{note}} to the note file.

    r {{index}}
        Removes a a note with the index {{index}}.

    m {{original_note}} {{destination_note}}
        Moves/Swaps the places of the notes with the index {{original_note}} and {{destination_note}}

    l 
        Lists all notes in the note file.

    c
        Enters in a command-like interface.

    h
        Displays this help message.
        "
    )
}

fn is_numeric(target: &String) -> bool {
    for character in target.chars() {
        if !character.is_numeric() {
            return false;
        }
    }
    true
}
