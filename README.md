# NOTES
Basic and simple application for taking notes written in Rust.

## Usage

### Arguments
Notes' arguments explained:

- `notes a {{note}}`:
    - Appends `{{note}}` to the notes file.

- `notes l`:
    - Lists all the notes in the notes file.

- `notes r {{index}}`:
    - Removes the note associated with `{{index}}`.

- `notes m {{index1}} {{index2}}`:
    - Swaps the notes associated with the indexes `{{index1}}` and `{{index2}}`.

- `notes c`:
    - Enters in the NCLI (*Notes Command Line Interface*), that can be used with all the arguments previously specified. To quit from the NCLI use `q`.

- `notes h`:
    -  Displays a help message.

### Notes file location
The notes application can use three different paths as notes file:

- **`./.notes`**, if it exists in the current folder you are located.
- **`$NOTES_FILE`**, (*an environmental variable*), if its defined.
- **`$HOME/.notes`**, if the previous ones didn't exist.


## Author
- [Folgue02](https://github.com/Folgue02/notes)
