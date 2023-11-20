//! # Chord Library
//!
//! This library provides functionality for working with musical chords.
//!
//! ## Example
//!
//! ```rust
//! use my_chord_library::{Note, Chord, parse_root};
//!
//! // Parse a chord from a string
//! let chord = parse_root("Cm")?;
//!
//! // Get the notes of the chord
//! let notes = map_chord_to_notes(chord);
//! ```
//!
//! ## Note Enum
//!
//! The `Note` enum represents musical notes:
//!
//! - A
//! - B
//! - C
//! - D
//! - E
//! - F
//! - G
//! - ASHARP
//! - CSHARP
//! - DSHARP
//! - FSHARP
//! - GSHARP
//! - DFLAT
//! - EFLAT
//! - GFLAT
//! - AFLAT
//! - BFLAT
//!
//! ## Chord Enum
//!
//! The `Chord` enum represents musical chords:
//!
//! - `MajorChord(Note)`
//! - `MinorChord(Note)`
//!
//! ## Functions
//!
//! - `to_note(string_rep: &str) -> Option<Note>`: Converts a string representation to a `Note`.
//! - `parse_root(input: &str) -> Result<Chord, Box<pest::error::Error<Rule>>>`: Parses a chord from a string.
//! - `map_chord_to_notes(chord: Chord) -> Vec<Note>`: Maps a chord to its constituent notes.
//! - `next_note(note: Note) -> Note`: Calculates the next note in the musical scale.
//! - `plus_perfect_fifth(note: Note) -> Note`: Calculates the note a perfect fifth above the given note.
//! - `plus_minor_third(note: Note) -> Note`: Calculates the note a minor third above the given note.
//! - `plus_major_third(note: Note) -> Note`: Calculates the note a major third above the given note.
//!
//! ## Example Usage
//!
//! ```rust
//! use my_chord_library::{Note, Chord};
//!
//! let note_a = Note::A;
//! let note_c_sharp = Note::CSHARP;
//!
//! let chord_a_major = Chord::MajorChord(note_a);
//! let chord_c_sharp_minor = Chord::MinorChord(note_c_sharp);
//! ```

use core::fmt;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammars/chord.pest"]
pub struct ChordParser;

#[derive(Debug, Clone, Copy)]
pub enum Note {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    ASHARP,
    CSHARP,
    DSHARP,
    FSHARP,
    GSHARP,
    DFLAT,
    EFLAT,
    GFLAT,
    AFLAT,
    BFLAT,
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Note::A => write!(f, "A"),
            Note::B => write!(f, "B"),
            Note::C => write!(f, "C"),
            Note::D => write!(f, "D"),
            Note::E => write!(f, "E"),
            Note::F => write!(f, "F"),
            Note::G => write!(f, "G"),
            Note::ASHARP => write!(f, "A#"),
            Note::CSHARP => write!(f, "C#"),
            Note::DSHARP => write!(f, "D#"),
            Note::FSHARP => write!(f, "F#"),
            Note::GSHARP => write!(f, "G#"),
            Note::DFLAT => write!(f, "Db"),
            Note::EFLAT => write!(f, "Eb"),
            Note::GFLAT => write!(f, "Gb"),
            Note::AFLAT => write!(f, "Ab"),
            Note::BFLAT => write!(f, "Bb"),
        }
    }
}

impl PartialEq for Note {
    fn eq(&self, other: &Self) -> bool {
        use Note::*;
        matches!(
            (self, other),
            (A, A)
                | (B, B)
                | (C, C)
                | (D, D)
                | (E, E)
                | (F, F)
                | (G, G)
                | (ASHARP, BFLAT)
                | (BFLAT, ASHARP)
                | (CSHARP, DFLAT)
                | (DFLAT, CSHARP)
                | (DSHARP, EFLAT)
                | (EFLAT, DSHARP)
                | (FSHARP, GFLAT)
                | (GFLAT, FSHARP)
                | (GSHARP, AFLAT)
                | (AFLAT, GSHARP)
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Chord {
    MajorChord(Note),
    MinorChord(Note),
}

pub fn to_note(string_rep: &str) -> Option<Note> {
    use Note::*;
    match string_rep {
        "A" => Some(A),
        "B" => Some(B),
        "C" => Some(C),
        "D" => Some(D),
        "E" => Some(E),
        "F" => Some(F),
        "G" => Some(G),

        "A#" => Some(ASHARP),
        "C#" => Some(CSHARP),
        "D#" => Some(DSHARP),
        "F#" => Some(FSHARP),
        "G#" => Some(GSHARP),

        "Db" => Some(DFLAT),
        "Eb" => Some(EFLAT),
        "Gb" => Some(GFLAT),
        "Ab" => Some(AFLAT),
        "Bb" => Some(BFLAT),

        &_ => None,
    }
}

fn next_note(note: Note) -> Note {
    use Note::*;
    match note {
        A => ASHARP,
        B => C,
        C => CSHARP,
        D => DSHARP,
        E => F,
        F => FSHARP,
        G => GSHARP,
        ASHARP => B,
        CSHARP => D,
        DSHARP => E,
        FSHARP => G,
        GSHARP => A,
        DFLAT => D,
        EFLAT => E,
        GFLAT => G,
        AFLAT => A,
        BFLAT => B,
    }
}

fn plus_perfect_fifth(note: Note) -> Note {
    next_note(next_note(next_note(next_note(next_note(next_note(
        next_note(note),
    ))))))
}

fn plus_minor_third(note: Note) -> Note {
    next_note(next_note(next_note(note)))
}

fn plus_major_third(note: Note) -> Note {
    next_note(plus_minor_third(note))
}

pub fn parse_root(input: &str) -> Result<Chord, Box<pest::error::Error<Rule>>> {
    let mut pairs = ChordParser::parse(Rule::root, input)?;

    for pair in pairs.clone() {
        for inner_pair in pair.into_inner() {
            if inner_pair.as_rule() == Rule::chord {
                let chord_pair = inner_pair.into_inner().next().unwrap();
                match chord_pair.as_rule() {
                    Rule::minor_chord => {
                        let tonic = chord_pair.into_inner().next().unwrap().as_str();
                        let tonic_note = to_note(tonic).unwrap();
                        return Ok(Chord::MinorChord(tonic_note));
                    }
                    Rule::major_chord => {
                        let tonic = chord_pair.into_inner().next().unwrap().as_str();
                        let tonic_note = to_note(tonic).unwrap();
                        return Ok(Chord::MajorChord(tonic_note));
                    }
                    _ => {
                        return Err(Box::new(pest::error::Error::new_from_span(
                            pest::error::ErrorVariant::CustomError {
                                message: String::from("Unknown chord rule"),
                            },
                            chord_pair.as_span(),
                        )));
                    }
                }
            }
        }
    }

    Err(Box::new(pest::error::Error::new_from_span(
        pest::error::ErrorVariant::CustomError {
            message: String::from("No chord rule found"),
        },
        pairs.next().unwrap().as_span(),
    )))
}

pub fn map_chord_to_notes(chord: Chord) -> Vec<Note> {
    match chord {
        Chord::MinorChord(tonic) => {
            let minor_third = plus_minor_third(tonic);
            let perfect_fifth = plus_perfect_fifth(tonic);
            vec![tonic, minor_third, perfect_fifth]
        }
        Chord::MajorChord(tonic) => {
            let major_third = plus_major_third(tonic);
            let perfect_fifth = plus_perfect_fifth(tonic);
            vec![tonic, major_third, perfect_fifth]
        }
    }
}
