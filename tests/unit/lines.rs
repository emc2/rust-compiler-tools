use compiler_tools::lines::LineOffsets;
use compiler_tools::lines::Offset;

#[test]
fn test_empty() {
    let line_offsets = LineOffsets::new();
    let (line, col) = line_offsets.lookup(Offset::from(5));

    assert_eq!(line, 0);
    assert_eq!(col, 5);
}

#[test]
fn test_zero() {
    let mut line_offsets = LineOffsets::new();

    line_offsets.push_line(2);

    let (line, col) = line_offsets.lookup(Offset::from(0));

    assert_eq!(line, 0);
    assert_eq!(col, 0);
}

#[test]
fn test_before() {
    let mut line_offsets = LineOffsets::new();

    line_offsets.push_line(2);

    let (line, col) = line_offsets.lookup(Offset::from(1));

    assert_eq!(line, 0);
    assert_eq!(col, 1);
}

#[test]
fn test_line() {
    let mut line_offsets = LineOffsets::new();

    line_offsets.push_line(2);

    let (line, col) = line_offsets.lookup(Offset::from(2));

    assert_eq!(line, 1);
    assert_eq!(col, 0);
}

#[test]
fn test_after() {
    let mut line_offsets = LineOffsets::new();

    line_offsets.push_line(2);

    let (line, col) = line_offsets.lookup(Offset::from(3));

    assert_eq!(line, 1);
    assert_eq!(col, 1);
}

#[test]
fn test_adjacent_zero() {
    let mut line_offsets = LineOffsets::new();

    line_offsets.push_line(2);
    line_offsets.push_line(3);

    let (line, col) = line_offsets.lookup(Offset::from(0));

    assert_eq!(line, 0);
    assert_eq!(col, 0);
}

#[test]
fn test_adjacent_before() {
    let mut line_offsets = LineOffsets::new();

    line_offsets.push_line(2);
    line_offsets.push_line(3);

    let (line, col) = line_offsets.lookup(Offset::from(1));

    assert_eq!(line, 0);
    assert_eq!(col, 1);
}

#[test]
fn test_adjacent_first_line() {
    let mut line_offsets = LineOffsets::new();

    line_offsets.push_line(2);
    line_offsets.push_line(3);

    let (line, col) = line_offsets.lookup(Offset::from(2));

    assert_eq!(line, 1);
    assert_eq!(col, 0);
}

#[test]
fn test_adjacent_second_line() {
    let mut line_offsets = LineOffsets::new();

    line_offsets.push_line(2);
    line_offsets.push_line(3);

    let (line, col) = line_offsets.lookup(Offset::from(3));

    assert_eq!(line, 2);
    assert_eq!(col, 0);
}

#[test]
fn test_adjacent_after() {
    let mut line_offsets = LineOffsets::new();

    line_offsets.push_line(2);
    line_offsets.push_line(3);

    let (line, col) = line_offsets.lookup(Offset::from(4));

    assert_eq!(line, 2);
    assert_eq!(col, 1);
}
