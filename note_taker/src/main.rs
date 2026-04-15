use std::env;
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    // Converts input string into a vector
    let args: Vec<String> = env::args().collect();

    // Checks for 2 arguments
    if args.len() < 2 {
        eprintln!("Error: No note provided.");
        eprintln!("Usage: note_taker \"your note here\"");
        std::process::exit(1);
    }

    // Note buffer
    let note = &args[1];

    // Max length
    if note.len() > 200 {
        eprintln!("Error: Note exceeds 200 character limit ({} chars).", note.len());
        std::process::exit(1);
    }

    // Grab today's date
    let today = chrono::Local::now().format("%-m/%-d/%y").to_string();

    // Create entry with date header
    let entry = format!("## {}\n- {}\n\n", today, note);
    
    // Create OpenOptions object with write enable
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("/home/brocke/OxideDex/Project_Notes.md")
        .expect("Failed to open Project_Notes.md");

    // Error fall back
    file.write_all(entry.as_bytes())
        .expect("Failed to write to Project_Notes.md");

    // Success!
    println!("Note added successfully.");
}
