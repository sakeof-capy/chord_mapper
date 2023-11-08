# Chord Mapper

## Brief Description

Chord Mapper is a Rust project that provides a command-line utility for parsing chord notations and mapping them to the individual musical notes they consist of. 

- parsing process: the parsing itself is delegated to the 'pest' crate. So the details about parsing process can be found here: https://docs.rs/pest/2.7.5/pest/

- what exactly is being parsed: actually only strings representing chords in international chord notations, like "C#", "C#m" are being parsed.

- how the results of the parsing will be used: the parsed string is converted to a program's chord representation which is then converted to the set of notes that make the chord up. Example: "Am" -> "A-C-E" (because A, C and E notes make up chord Am).

## Grammar Rules

Chord Mapper uses a custom grammar to parse chord notations. The grammar rules are defined as follows:

```rust
root = { SOI ~ delimitor* ~ chord ~ delimitor* ~ EOI }
delimitor = { " "+ }

chord = { minor_chord | major_chord }
major_chord = { tonic }
minor_chord = { tonic ~ "m" }

tonic = { sharped_note | flatted_note | bare_note }
bare_note = { "A" | "B" | "C" | "D" | "E" | "F" | "G" | "H" }
sharped_note = { ("A" | "C" | "D" | "F" | "G") ~ "#" }
flatted_note = { ("D" | "E" | "G" | "A" | "B") ~ "b" }
```

- `root`: Represents the root rule for parsing a chord notation.
- `delimitor`: Matches any number of spaces used as delimiters.
- `chord`: Defines the structure of a chord, which can be either a `major_chord` or a `minor_chord`.
- `major_chord`: Represents a major chord consisting of a `tonic`.
- `minor_chord`: Represents a minor chord consisting of a `tonic` followed by 'm'.
- `tonic`: Specifies the core note of a chord, which can be a `sharped_note`, `flatted_note`, or `bare_note`.
- `bare_note`: Matches bare note names, such as 'A', 'B', 'C', 'D', 'E', 'F', 'G', or 'H'.
- `sharped_note`: Matches sharped note names, like 'A#', 'C#', 'D#', 'F#', or 'G#'.
- `flatted_note`: Matches flatted note names, including 'Db', 'Eb', 'Gb', 'Ab', or 'Bb'.

## Usage

The Chord Mapper project includes a command-line interface (CLI) that allows you to interact with the tool. Here are some available commands:

- `parse`: Use this command to parse a chord notation and display its constituent notes.

    Example:
    ```shell
    chord_mapper parse Cm
    ```

- `help`: Show information about how to use the Chord Mapper CLI.

    Example:
    ```shell
    chord_mapper help
    ```

- `credits`: Show information about the project and its contributors.

    Example:
    ```shell
    chord_mapper credits
    ```
---

Chord Mapper is an essential tool for musicians and music enthusiasts, helping you understand chord notations and their underlying musical components.
