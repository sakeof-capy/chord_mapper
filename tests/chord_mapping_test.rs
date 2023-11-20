use chord_mapper::map_chord_to_notes;
use chord_mapper::Chord;
use chord_mapper::Note;

fn assert_equal(vec1: Vec<Note>, vec2: Vec<Note>) -> bool {
    for i in 0..vec1.len() {
        if vec1[i] != vec2[i] {
            return false;
        }
    }

    true
}

#[test]
fn minor_chord_mapping_test() {
    use Chord::*;
    use Note::*;
    assert_equal(map_chord_to_notes(MinorChord(A)), vec![A, C, E]);
    assert_equal(
        map_chord_to_notes(MinorChord(ASHARP)),
        vec![BFLAT, DFLAT, F],
    );
    assert_equal(map_chord_to_notes(MinorChord(BFLAT)), vec![BFLAT, DFLAT, F]);
    assert_equal(map_chord_to_notes(MinorChord(C)), vec![C, EFLAT, G]);
    assert_equal(map_chord_to_notes(MinorChord(DFLAT)), vec![DFLAT, E, AFLAT]);
    assert_equal(
        map_chord_to_notes(MinorChord(CSHARP)),
        vec![DFLAT, E, AFLAT],
    );
    assert_equal(map_chord_to_notes(MinorChord(D)), vec![D, F, A]);
    assert_equal(
        map_chord_to_notes(MinorChord(EFLAT)),
        vec![EFLAT, GFLAT, BFLAT],
    );
    assert_equal(
        map_chord_to_notes(MinorChord(DSHARP)),
        vec![EFLAT, GFLAT, BFLAT],
    );
    assert_equal(map_chord_to_notes(MinorChord(E)), vec![E, G, B]);
    assert_equal(map_chord_to_notes(MinorChord(F)), vec![F, AFLAT, C]);
    assert_equal(map_chord_to_notes(MinorChord(GFLAT)), vec![GFLAT, A, DFLAT]);
    assert_equal(
        map_chord_to_notes(MinorChord(FSHARP)),
        vec![GFLAT, A, DFLAT],
    );
    assert_equal(map_chord_to_notes(MinorChord(G)), vec![G, BFLAT, D]);
    assert_equal(map_chord_to_notes(MinorChord(AFLAT)), vec![AFLAT, B, EFLAT]);
}

#[test]
fn major_chord_mapping_test() {
    use Chord::*;
    use Note::*;
    assert_equal(map_chord_to_notes(MajorChord(A)), vec![A, CSHARP, E]);
    assert_equal(map_chord_to_notes(MajorChord(ASHARP)), vec![BFLAT, D, F]);
    assert_equal(map_chord_to_notes(MajorChord(BFLAT)), vec![BFLAT, D, F]);
    assert_equal(map_chord_to_notes(MajorChord(C)), vec![C, E, G]);
    assert_equal(map_chord_to_notes(MajorChord(DFLAT)), vec![DFLAT, F, AFLAT]);
    assert_equal(
        map_chord_to_notes(MajorChord(CSHARP)),
        vec![DFLAT, F, AFLAT],
    );
    assert_equal(map_chord_to_notes(MajorChord(D)), vec![D, FSHARP, A]);
    assert_equal(map_chord_to_notes(MajorChord(EFLAT)), vec![EFLAT, G, BFLAT]);
    assert_equal(
        map_chord_to_notes(MajorChord(DSHARP)),
        vec![EFLAT, G, BFLAT],
    );
    assert_equal(map_chord_to_notes(MajorChord(E)), vec![E, GSHARP, B]);
    assert_equal(map_chord_to_notes(MajorChord(F)), vec![F, A, C]);
    assert_equal(
        map_chord_to_notes(MajorChord(GFLAT)),
        vec![GFLAT, ASHARP, DFLAT],
    );
    assert_equal(
        map_chord_to_notes(MajorChord(FSHARP)),
        vec![GFLAT, ASHARP, DFLAT],
    );
    assert_equal(map_chord_to_notes(MajorChord(G)), vec![G, B, D]);
    assert_equal(map_chord_to_notes(MajorChord(AFLAT)), vec![AFLAT, C, EFLAT]);
}
