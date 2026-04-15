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
    let date_check = std::fs::read_to_string("/home/brocke/OxideDex/Project_Notes.md").expect("Failed to read Project_Notes.md");
    let last_date = date_check.lines().rev().find(|line| line.starts_with("## "));

    let entry = match last_date {
        // append header + note (empty file or no headers yet)
        None => {
            format!("\n## {}\n- {}\n", today, note)
        }

        Some(line) => {
            let date = &line[3..];
            if date == today {
                // append note only
                format!("- {}\n", note)
            } else {
                // append header + note
                format!("\n## {}\n- {}\n", today, note)
            }
        }
    };

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("/home/brocke/OxideDex/Project_Notes.md")
        .expect("Failed to open Project_Notes.md");

    // Error fall back
    file.write_all(entry.as_bytes())
        .expect("Failed to write to Project_Notes.md");

    // Success!
    println!("Note added successfully:");
    println!("{}", entry);
}
