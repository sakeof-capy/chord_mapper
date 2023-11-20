use chord_mapper::ChordParser;
use chord_mapper::Rule;
use pest::Parser;

#[inline]
fn check_str_matches_the_rule(note: &str, rule: Rule) -> anyhow::Result<bool> {
    ChordParser::parse(rule, note)?
        .into_iter()
        .next()
        .map(|pair| pair.as_span().as_str() == note)
        .ok_or_else(|| anyhow::anyhow!("Parsing results in too few tokens"))
}

fn test_template_rule_matching(samples: &Vec<&str>, rule: Rule) -> anyhow::Result<()> {
    for note in samples {
        assert!(
            check_str_matches_the_rule(note, rule)?,
            "Bare note {} not parsed.",
            note
        );
    }

    Ok(())
}

fn test_template_rule_not_matching(samples: &Vec<&str>, rule: Rule) -> anyhow::Result<()> {
    for note in samples {
        let check = check_str_matches_the_rule(note, rule);
        assert!(
            check.is_err() || !check.unwrap(),
            "Non-bare note {} parsed.",
            note
        );
    }

    Ok(())
}

#[test]
fn bare_notes_test() -> anyhow::Result<()> {
    let bare_notes = vec!["A", "B", "C", "D", "E", "F", "G"];
    test_template_rule_matching(&bare_notes, Rule::bare_note)
}

#[test]
fn non_bare_notes_test() -> anyhow::Result<()> {
    let non_bare_notes = vec![
        "Ab", "Bb", "Db", "Eb", "Gb", "A#", "C#", "D#", "F#", "G#", "E#", "B#", "Cb", "Fb", "H",
        "AA", "RCDSGSFD",
    ];

    test_template_rule_not_matching(&non_bare_notes, Rule::bare_note)
}

#[test]
fn flatted_notes_test() -> anyhow::Result<()> {
    let flatted_notes = vec!["Ab", "Bb", "Db", "Eb", "Gb"];
    test_template_rule_matching(&flatted_notes, Rule::flatted_note)
}

#[test]
fn non_flattable_notes_test() -> anyhow::Result<()> {
    let non_flattable_notes = vec!["Cb", "Fb", "CFb", "AA", "RCDSGSFD"];
    test_template_rule_not_matching(&non_flattable_notes, Rule::flatted_note)
}

#[test]
fn sharped_notes_test() -> anyhow::Result<()> {
    let sharped_notes = vec!["A#", "C#", "D#", "F#", "G#"];
    test_template_rule_matching(&sharped_notes, Rule::sharped_note)
}

#[test]
fn non_sharpable_notes_test() -> anyhow::Result<()> {
    let non_sharpable_notes = vec!["E#", "B#", "EB#", "AA", "RCDSGSFD"];
    test_template_rule_not_matching(&non_sharpable_notes, Rule::sharped_note)
}

#[test]
fn tonic_test() -> anyhow::Result<()> {
    let tonics = vec![
        "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "Ab", "Bb", "Db", "Eb",
        "Gb",
    ];
    test_template_rule_matching(&tonics, Rule::tonic)
}

#[test]
fn non_tonic_test() -> anyhow::Result<()> {
    let non_tonics = vec!["AA", "RCDSGSFD", "B#", "Fb"];
    test_template_rule_not_matching(&non_tonics, Rule::tonic)
}

#[test]
fn minor_chord_test() -> anyhow::Result<()> {
    let minor_chords = vec![
        "Am", "A#m", "Bm", "Cm", "C#m", "Dm", "D#m", "Em", "Fm", "F#m", "Gm", "G#m", "Abm", "Bbm",
        "Dbm", "Ebm", "Gbm",
    ];
    test_template_rule_matching(&minor_chords, Rule::minor_chord)
}

#[test]
fn non_minor_chord_test() -> anyhow::Result<()> {
    let non_minor_chords = vec![
        "A", "B", "C", "D", "E", "F", "G", "G#", "AA", "RCDSGSFD", "B#", "Fb",
    ];
    test_template_rule_not_matching(&non_minor_chords, Rule::minor_chord)
}

#[test]
fn major_chord_test() -> anyhow::Result<()> {
    let major_chords = vec![
        "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "Ab", "Bb", "Db", "Eb",
        "Gb",
    ];
    test_template_rule_matching(&major_chords, Rule::major_chord)
}

#[test]
fn non_major_chord_test() -> anyhow::Result<()> {
    let non_major_chords = vec![
        "Am", "Bm", "Cm", "Dm", "Em", "Fm", "Gm", "G#m", "AA", "RCDSGSFD", "B#", "Fb",
    ];
    test_template_rule_not_matching(&non_major_chords, Rule::major_chord)
}

#[test]
fn chord_test() -> anyhow::Result<()> {
    let chords = vec![
        "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "Ab", "Bb", "Db", "Eb",
        "Gb", "Am", "A#m", "Bm", "Cm", "C#m", "Dm", "D#m", "Em", "Fm", "F#m", "Gm", "G#m", "Abm",
        "Bbm", "Dbm", "Ebm", "Gbm",
    ];
    test_template_rule_matching(&chords, Rule::chord)
}

#[test]
fn non_chord_test() -> anyhow::Result<()> {
    let non_chords = vec!["E#", "Fbm", "AA", "RCDSGSFD", "B#", "Fb"];
    test_template_rule_not_matching(&non_chords, Rule::chord)
}

#[test]
fn delimitor_test() -> anyhow::Result<()> {
    let delimitors = vec![" ", "  ", "    ", "           "];
    test_template_rule_matching(&delimitors, Rule::delimitor)
}

#[test]
fn non_delimitor_test() -> anyhow::Result<()> {
    let non_delimitors = vec!["", ",", " , ", "_", "; "];
    test_template_rule_not_matching(&non_delimitors, Rule::delimitor)
}

#[test]
fn root_test() -> anyhow::Result<()> {
    let roots = vec![" A#", " C ", "  Dbm  ", "     C#m      "];
    test_template_rule_matching(&roots, Rule::root)
}

#[test]
fn non_root_test() -> anyhow::Result<()> {
    let non_roots = vec!["E#", " B# ", "A ;", "_ C#m"];
    test_template_rule_not_matching(&non_roots, Rule::root)
}
