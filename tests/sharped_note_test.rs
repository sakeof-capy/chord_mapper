use chord_mapper::ChordParser;
use chord_mapper::Rule;
use pest::Parser;

#[test]
fn sharped_notes_test() -> anyhow::Result<()> {
    let sharped_notes = vec!["A#", "C#", "D#", "F#", "G#"];

    for note in sharped_notes {
        ChordParser::parse(Rule::sharped_note, note)?;
    }

    Ok(())
}

#[test]
fn non_sharpable_notes_test() {
    let non_sharpable_notes = vec!["E#", "B#"];

    for note in non_sharpable_notes {
        let result = ChordParser::parse(Rule::sharped_note, note);
        assert!(result.is_err());
    }
}
