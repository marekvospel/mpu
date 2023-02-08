use masm_location::{get_location_from_source, Position, SourceLocation};

#[test]
fn should_get_text_from_source() {
    let source = "abcde\nf";

    let location = SourceLocation {
        start: Position::default(),
        end: Position {
            line: 1,
            column: 5,
            offset: 4,
        },
    };

    assert_eq!(get_location_from_source(source, location), "abcde");

    let location = SourceLocation {
        start: Position {
            line: 1,
            column: 4,
            offset: 3,
        },
        end: Position {
            line: 1,
            column: 6,
            offset: 5,
        },
    };

    assert_eq!(get_location_from_source(source, location), "de\n");

    let location = SourceLocation {
        start: Position {
            line: 1,
            column: 4,
            offset: 3,
        },
        end: Position {
            line: 1,
            column: 8,
            offset: 7,
        },
    };

    assert_eq!(get_location_from_source(source, location), "de\nf");
}
