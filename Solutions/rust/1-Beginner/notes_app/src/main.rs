use std::io;
use std::fs;
use std::collections::HashMap;

use chrono;
use serde_derive::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
struct Note {
    title: String,
    date: String,
    body: String,
}

impl Note {
    fn display(&self) {
        println!("{} - {}", self.title, self.date);
        println!("\t{}", self.body);
    }

    fn default() -> Note {
        Note {
            title: String::from("Untitled"),
            date: String::from(""),
            body: String::from(""),
        }
    }
}

#[derive(PartialEq, Debug)]
enum Action {
    Create,
    Display,
    Update,
    Delete,
    Quit,
}

fn get_action() -> Action {
    loop {
        let mut action = String::new();
        
        println!("What do you want to do?");
        println!("1. Create a note");
        println!("2. Display the notes");
        println!("3. Update a note");
        println!("4. Delete a note");
        println!("5. Quit");

        // get user input and store it in action (String)
        io::stdin()
            .read_line(&mut action)
            .expect("Failed to read line");

        // convert the input to an integer, displaying appropriate error message if it fails
        let action: i32 = match action.trim().parse() {
            Ok(num) => {
                if num > 0 && num < 6 {
                    num
                } else {
                    println!("Invalid input, number should be between 1 and 5");
                    continue
                }
            },
            Err(_) => {
                println!("Enter a number");
                continue
            },
        };
        
        // match the integer to the appropriate Action enum variant
        return match action {
            1 => Action::Create,
            2 => Action::Display,
            3 => Action::Update,
            4 => Action::Delete,
            5 => Action::Quit,
            _ => Action::Quit, // should never happen, needed for compiler
        }
    }
}

fn execute(action: Action, notes: &HashMap<String, Note>) -> Option<(Action, Note)> {
    match action {
        Action::Create => {
            let mut title = String::new();
            let mut body = String::new();

            println!("Enter a title for your note:");
            io::stdin()
                .read_line(&mut title)
                .expect("Failed to read line");

            println!("Enter the body of your note:");
            io::stdin()
                .read_line(&mut body)
                .expect("Failed to read line");

            return Some((action, Note {
                title: title.trim().to_string().to_uppercase(),
                date: chrono::Local::now().format("%d %b %Y").to_string(),
                body: body.trim().to_string(),
            }));
        },
        Action::Display => {
            println!("Notes:");
            if notes.len() != 0 {
                for (_, note) in notes {
                    note.display();
                }
                println!("");
            } else {
                println!("No notes found");
                println!("");
            }
            return None;
        },
        Action::Update => {
            println!("Enter the title of the note you want to update:");
            let mut title = String::new();
            io::stdin()
                .read_line(&mut title)
                .expect("Failed to read line");

            title = title.trim().to_string().to_uppercase();

            if notes.contains_key(&title) {
                println!("Enter the new body of the note:");
                let mut body = String::new();
                io::stdin()
                    .read_line(&mut body)
                    .expect("Failed to read line");

                return Some((action, Note {
                    title,
                    date: chrono::Local::now().to_string(),
                    body: body.trim().to_string(),
                }));
            } else {
                println!("Note not found");
                return None;
            }
        },
        Action::Delete => {
            println!("Enter the title of the note you want to delete:");

            let mut title = String::new();
            io::stdin()
                .read_line(&mut title)
                .expect("Failed to read line");

            title = title.trim().to_string().to_uppercase();

            return Some((action, Note {
                title,
                date: chrono::Local::now().to_string(),
                body: String::new(),
            }));
        },
        Action::Quit => {
            println!("Quit");
            return Some((action, Note::default()));
        },
    }
}

fn read_json() -> Option<HashMap<String, Note>> {
    let mut notes: HashMap<String, Note> = HashMap::new();

    let data = String::from_utf8(
        fs::read("notes/notes.json").expect("Failed to read file")
    ).expect("Failed to convert to string");

    if data.len() == 0 {
        println!("No notes found");
        return None;
    }

    let json: serde_json::Value = match serde_json::from_str(&data) {
        Ok(json) => json,
        Err(_) => {
            println!("Failed to parse JSON");
            return None;
        },
    };

    let notes_json = json["notes"].as_array().expect("Failed to parse JSON");
    
    for note in notes_json {
        let title = note["title"].as_str().expect("Failed to parse title json").to_string();
        let body = note["body"].as_str().expect("Failed to parse body json").to_string();
        let date = note["date"].as_str().expect("Failed to parse date json").to_string();

        notes.insert(title.clone(), Note {
            title,
            date,
            body,
        });
    }

    return Some(notes);
} 

fn write_json(notes: HashMap<String, Note>) {
    let mut notes_json: Vec<serde_json::Value> = Vec::new();

    for (title, note) in notes {
        notes_json.push(serde_json::json!({
            "title": title,
            "body": note.body,
            "date": note.date,
        }));
    }

    let json = serde_json::json!({
        "notes": notes_json,
    });

    let data = json.to_string();

    fs::write("notes/notes.json", data.as_bytes()).expect("Failed to write file");
}

fn main() {

    let mut notes: HashMap<String, Note> = match read_json() {
        Some(notes) => notes,
        None => HashMap::new(),
    };

    loop {
        let action = get_action();

        match execute(action, &notes) {
            Some((action, note)) => {
                match action {
                    Action::Create => {
                        if !notes.contains_key(&note.title) {
                            notes.insert(note.title.clone(), note);
                        } else {
                            println!("Note already exists");
                            notes.insert(note.title.clone(), note);
                        }
                    },
                    Action::Update => {
                        notes.remove(&note.title);
                        notes.insert(note.title.clone(), note);
                    },
                    Action::Delete => {
                        println!("Deleting note '{}'", note.title);
                        notes.remove(&note.title);
                    },
                    Action::Quit => {
                        println!("Saving notes...");
                        write_json(notes);
                        break;
                    },
                    _ => continue,
                }
            },
            None => continue,
        };
    }
}
