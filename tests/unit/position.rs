use compiler_tools::files::Filenames;
use compiler_tools::files::FileOffsets;
use compiler_tools::lines::LineOffsets;
use compiler_tools::lines::Offset;
use compiler_tools::nondistinct::Nondistinct;
use compiler_tools::position::BasicPosition;
use compiler_tools::position::FilePosition;
use compiler_tools::position::OffsetPosition;
use std::path::Path;

#[test]
fn test_file_position_file() {
    let path = Path::new("./Cargo.toml");
    let canonical_path = path.canonicalize().expect("Expected success");
    let path_str = canonical_path.to_str().expect("Expected some");
    let mut filenames = Filenames::new();
    let filename = filenames.filename(&path).expect("Expected success");
    let pos = FilePosition::File { filename: filename };

    assert_eq!(format!("in \"{}\"", path_str), format!("{}", pos));
}

#[test]
fn test_file_position_portion_point() {
    let path = Path::new("./Cargo.toml");
    let canonical_path = path.canonicalize().expect("Expected success");
    let path_str = canonical_path.to_str().expect("Expected some");
    let mut filenames = Filenames::new();
    let filename = filenames.filename(&path).expect("Expected success");
    let mut line_offsets = LineOffsets::new();

    line_offsets.push_line(2);
    line_offsets.push_line(3);

    let file_offsets = FileOffsets::new(filename, line_offsets);
    let offset_pos = OffsetPosition::Point { point: Offset::from(2) };
    let pos = FilePosition::Portion { file_offsets: &file_offsets,
                                      offset: offset_pos };

    assert_eq!(format!("at \"{}\":2.1", path_str),
               format!("{}", pos));
}

#[test]
fn test_file_position_portion_span_same_line() {
    let path = Path::new("./Cargo.toml");
    let canonical_path = path.canonicalize().expect("Expected success");
    let path_str = canonical_path.to_str().expect("Expected some");
    let mut filenames = Filenames::new();
    let filename = filenames.filename(&path).expect("Expected success");
    let mut line_offsets = LineOffsets::new();

    line_offsets.push_line(2);
    line_offsets.push_line(3);

    let file_offsets = FileOffsets::new(filename, line_offsets);
    let offset_pos = OffsetPosition::Span { start: Offset::from(3),
                                            len: Offset::from(2) };
    let pos = FilePosition::Portion { file_offsets: &file_offsets,
                                      offset: offset_pos };

    assert_eq!(format!("at \"{}\":3.1-3", path_str),
               format!("{}", pos));
}

#[test]
fn test_file_position_portion_span_cross_lines() {
    let path = Path::new("./Cargo.toml");
    let canonical_path = path.canonicalize().expect("Expected success");
    let path_str = canonical_path.to_str().expect("Expected some");
    let mut filenames = Filenames::new();
    let filename = filenames.filename(&path).expect("Expected success");
    let mut line_offsets = LineOffsets::new();

    line_offsets.push_line(2);
    line_offsets.push_line(5);

    let file_offsets = FileOffsets::new(filename, line_offsets);
    let offset_pos = OffsetPosition::Span { start: Offset::from(3),
                                            len: Offset::from(3) };
    let pos = FilePosition::Portion { file_offsets: &file_offsets,
                                      offset: offset_pos };

    assert_eq!(format!("at \"{}\":2.2-3.2", path_str),
               format!("{}", pos));
}




#[test]
fn test_basic_position_file() {
    let path = Path::new("./Cargo.toml");
    let canonical_path = path.canonicalize().expect("Expected success");
    let path_str = canonical_path.to_str().expect("Expected some");
    let mut filenames = Filenames::new();
    let filename = filenames.filename(&path).expect("Expected success");
    let file_pos = FilePosition::File { filename: filename };
    let pos = BasicPosition::File { pos: file_pos };

    assert_eq!(format!("in \"{}\"", path_str), format!("{}", pos));
}

#[test]
fn test_basic_position_file_portion_point() {
    let path = Path::new("./Cargo.toml");
    let canonical_path = path.canonicalize().expect("Expected success");
    let path_str = canonical_path.to_str().expect("Expected some");
    let mut filenames = Filenames::new();
    let filename = filenames.filename(&path).expect("Expected success");
    let mut line_offsets = LineOffsets::new();

    line_offsets.push_line(2);
    line_offsets.push_line(3);

    let file_offsets = FileOffsets::new(filename, line_offsets);
    let offset_pos = OffsetPosition::Point { point: Offset::from(2) };
    let file_pos = FilePosition::Portion { file_offsets: &file_offsets,
                                           offset: offset_pos };
    let pos = BasicPosition::File { pos: file_pos };

    assert_eq!(format!("at \"{}\":2.1", path_str),
               format!("{}", pos));
}

#[test]
fn test_basic_position_file_portion_span_same_line() {
    let path = Path::new("./Cargo.toml");
    let canonical_path = path.canonicalize().expect("Expected success");
    let path_str = canonical_path.to_str().expect("Expected some");
    let mut filenames = Filenames::new();
    let filename = filenames.filename(&path).expect("Expected success");
    let mut line_offsets = LineOffsets::new();

    line_offsets.push_line(2);
    line_offsets.push_line(3);

    let file_offsets = FileOffsets::new(filename, line_offsets);
    let offset_pos = OffsetPosition::Span { start: Offset::from(3),
                                            len: Offset::from(2) };
    let file_pos = FilePosition::Portion { file_offsets: &file_offsets,
                                           offset: offset_pos };
    let pos = BasicPosition::File { pos: file_pos };

    assert_eq!(format!("at \"{}\":3.1-3", path_str),
               format!("{}", pos));
}


#[test]
fn test_basic_position_file_portion_span_cross_lines() {
    let path = Path::new("./Cargo.toml");
    let canonical_path = path.canonicalize().expect("Expected success");
    let path_str = canonical_path.to_str().expect("Expected some");
    let mut filenames = Filenames::new();
    let filename = filenames.filename(&path).expect("Expected success");
    let mut line_offsets = LineOffsets::new();

    line_offsets.push_line(2);
    line_offsets.push_line(5);

    let file_offsets = FileOffsets::new(filename, line_offsets);
    let offset_pos = OffsetPosition::Span { start: Offset::from(3),
                                            len: Offset::from(3) };
    let file_pos = FilePosition::Portion { file_offsets: &file_offsets,
                                           offset: offset_pos };
    let pos = BasicPosition::File { pos: file_pos };

    assert_eq!(format!("at \"{}\":2.2-3.2", path_str),
               format!("{}", pos));
}

#[test]
fn test_basic_position_input_portion_point() {
    let mut line_offsets = LineOffsets::new();

    line_offsets.push_line(2);
    line_offsets.push_line(3);

    let offset_pos = OffsetPosition::Point { point: Offset::from(2) };
    let pos = BasicPosition::Input {
        line_offsets: Nondistinct::from(&line_offsets), offset: offset_pos
    };

    assert_eq!(format!("at input 2.1"), format!("{}", pos));
}

#[test]
fn test_basic_position_input_portion_span_same_line() {
    let mut line_offsets = LineOffsets::new();

    line_offsets.push_line(2);
    line_offsets.push_line(3);

    let offset_pos = OffsetPosition::Span { start: Offset::from(3),
                                            len: Offset::from(2) };
    let pos = BasicPosition::Input {
        line_offsets: Nondistinct::from(&line_offsets), offset: offset_pos
    };

    assert_eq!(format!("at input 3.1-3"), format!("{}", pos));
}


#[test]
fn test_basic_position_input_portion_span_cross_lines() {
    let mut line_offsets = LineOffsets::new();

    line_offsets.push_line(2);
    line_offsets.push_line(5);

    let offset_pos = OffsetPosition::Span { start: Offset::from(3),
                                            len: Offset::from(3) };
    let pos = BasicPosition::Input {
        line_offsets: Nondistinct::from(&line_offsets), offset: offset_pos
    };

    assert_eq!(format!("at input 2.2-3.2"), format!("{}", pos));
}
