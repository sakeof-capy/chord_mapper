use chord_mapper::ChordParser;
use chord_mapper::Rule;
use pest::Parser;

#[test]
fn flatted_notes_test() -> anyhow::Result<()> {
    let flatted_notes = vec!["Ab", "Bb", "Db", "Eb", "Gb"];

    for note in flatted_notes {
        ChordParser::parse(Rule::flatted_note, note)?;
    }

    Ok(())
}

#[test]
fn non_flattable_notes_test() {
    let non_flattable_notes = vec!["Cb", "Fb"];

    for note in non_flattable_notes {
        let result = ChordParser::parse(Rule::flatted_note, note);
        assert!(result.is_err());
    }
}
