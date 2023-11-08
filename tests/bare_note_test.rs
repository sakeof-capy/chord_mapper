use chord_mapper::ChordParser;
use chord_mapper::Rule;
use pest::Parser;

#[test]
fn bare_notes_test() -> anyhow::Result<()> {
    let bare_notes = vec!["A", "B", "C", "D", "E", "F", "G", "H"];

    for note in bare_notes {
        ChordParser::parse(Rule::bare_note, note)?;
    }

    Ok(())
}

#[test]
fn non_bare_notes_test() {
    let non_bare_notes = vec![
        "Ab", "Bb", "Db", "Eb", "Gb", "A#", "C#", "D#", "F#", "G#", "E#", "B#", "Cb", "Fb",
    ];

    for note in non_bare_notes {
        let result = ChordParser::parse(Rule::bare_note, note);
        let parsed_note = result.clone().into_iter().next().unwrap().as_str();
        assert!(parsed_note != note || result.is_err());
    }
}
