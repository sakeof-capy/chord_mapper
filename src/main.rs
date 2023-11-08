use std::env;

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
                println!(
                    "The chord consists of the following notes: {}",
                    chord_notation
                );
            }
        }
        "help" => {
            println!("Chord Mapper - Help");
            println!("Usage:");
            println!("  chord_mapper parse <chord_notation> - Parse a chord notation");
            println!("  chord_mapper help - See help information.");
        }
        _ => {
            println!("Undefined command. Use 'chord_mapper help' for help information.");
        }
    }
}
