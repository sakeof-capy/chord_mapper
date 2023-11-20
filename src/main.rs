use std::env;

use chord_mapper::{map_chord_to_notes, parse_root};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Undefined command. Use 'chord_mapper help' for help information.");
        return;
    }

    let command = &args[1];
    match command.as_str() {
        "parse" => {
            if args.len() != 3 {
                println!("Usage: chord_mapper parse <chord_notation>");
            } else {
                let chord_notation = &args[2];
                match parse_root(chord_notation) {
                    Ok(chord) => {
                        let notes = map_chord_to_notes(chord);
                        for note in notes {
                            print!("{} ", note);
                        }
                    }
                    Err(err) => {
                        println!("Error: {}", err);
                    }
                }
            }
        }
        "help" => {
            println!("Chord Mapper - Help");
            println!("Usage:");
            println!("  chord_mapper parse <chord_notation> - Parse a chord notation");
            println!("  chord_mapper credits - Display credits information.");
            println!("  chord_mapper help - See help information.");
            println!("\nAdditional Information:");
            println!(" - Chord notation should be provided in uppercase (e.g., Am, G, C#)");
            println!(" - Supported notes: A, B, C, D, E, F, G");
            println!(" - Accidentals: # (sharp), b (flat)");
        }
        "credits" => {
            println!("Chord Mapper - Credits");
            println!("Developed by Ruslan Zymovets");
            println!("Version 1.0.0");
            println!("© 2023 Chord Mapper Project");
        }
        _ => {
            println!("Undefined command. Use 'chord_mapper help' for help information.");
        }
    }
}
