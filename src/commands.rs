use crate::notes;
use std::io;
use std::io::Write;

/// The `most cancerous` function ever written in a programming language, **in the world**
pub fn run(mut nts: &mut notes::Notes, args: Vec<String>) -> Result<(), &'static str> {
    // WELCOME TO THE WORLD'S WORST FUNCTION EVER WRITTEN IN A PROGRAMMING LANGUAGE! :D (TODO: `Fix` this)
    // No arguments supplied
    if args.len() == 0 {
        display_help();
    } else {
        // One argument
        if args.len() >= 1 {
            if args[0] == "l" {
                list(&mut nts)?;
                return Ok(());
            } else if args[0] == "h" {
                display_help();
                return Ok(());
            } else if args[0] == "c" {
                cli(&mut nts);
                return Ok(());
            }
        }

        // At least two arguments
        if args.len() >= 2 {
            if args[0] == "a" {
                add(&mut nts, &args[1..].to_vec().join(" "))?;
            } else if args[0] == "r" {
                remove(&mut nts, &args[1])?;
            } else if args.len() >= 3 {
                // At least three arguments
                if args[0] == "m" {
                    move_note(&mut nts, &args[1], &args[2])?;
                } else {
                    display_help();
                }
            } else {
                display_help();
            }
        } else {
            display_help();
        }
    }
    nts.save();
    Ok(())
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
    for (idx, note) in notes.get_vec().into_iter().enumerate() {
        println!("{}:{}", idx + 1, note);
    }
    Ok(())
}

pub fn move_note(notes: &mut notes::Notes, original_index: &String, destination_index: &String, ) -> Result<(), &'static str> {
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

fn cli(mut nts: &mut notes::Notes) {
    println!("Notes Command Line Interface (type 'q' to leave)");
    loop {
        let mut buffer = String::new();
        print!("CLI: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut buffer);

        // Quit command
        if buffer == "q\n" {
            return ();
        }

        match run(
            nts,
            buffer
                .split(" ")
                .filter(|slice| {
                    if slice.trim() != "" || slice.trim() == "\n" {
                        true
                    } else {
                        false
                    }
                })
                .map(|slice| String::from(slice.trim()))
                .collect::<Vec<String>>(),
        ) {
            Ok(()) => (),
            Err(error) => {
                eprintln!("An error has occurred: {}", error);
            }
        };
    }
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
