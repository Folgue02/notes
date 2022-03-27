use crate::notes::Notes;

pub fn add(notes: &mut Notes, args: &String) {
    notes.add(args.clone());
}

pub fn remove(notes: &mut Notes, args: &String) {
    let index = match args.parse::<u8>() {
        Ok(index) => index - 1,
        Err(_) => {
            println!("You have to specify a number as index for the note you want to remove.");
            std::process::exit(1);
        }
    };

    match notes.remove(index) {
        Ok(_) => (),
        Err(_) => {
            println!("Index out of range.");
            std::process::exit(1);
        }
    }
}

pub fn list(notes: &mut Notes) {
    let mut idx = 0;
    for note in &notes.get_vec() {
        idx += 1;
        println!("{}:{}", idx, note);
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

    l 
        Lists all notes in the note file.

    h
        Displays this help message.
        "
    )
}
