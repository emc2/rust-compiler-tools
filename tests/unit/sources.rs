use compiler_tools::files::Filenames;
use compiler_tools::lines::Offset;
use compiler_tools::position::OffsetPosition;
use compiler_tools::sources::Sources;
use compiler_tools::sources::SourceContext;
use std::path::Path;

#[test]
fn test_sources_point_begin() {
    let path = Path::new("./Cargo.toml");
    let mut filenames = Filenames::new();
    let filename = filenames.filename(&path).expect("Expected success");
    let mut srcs = Sources::new();
    let src = srcs.add_src(filename).expect("Expected some");

    src.push_line(0, String::from("a"));
    src.push_line(2, String::from(""));
    src.push_line(3, String::from("bcde"));
    src.push_line(8, String::from("fghi"));
    src.push_line(13, String::from("jk"));

    let offset_pos = OffsetPosition::Point { point: Offset::from(0) };
    let ctx = srcs.get_ctx(filename, &offset_pos).expect("Expected some");

    assert_eq!(SourceContext::Single { prefix: "", selected: "a",
                                       suffix: "" },
               ctx)
}

#[test]
fn test_sources_point_endline() {
    let path = Path::new("./Cargo.toml");
    let mut filenames = Filenames::new();
    let filename = filenames.filename(&path).expect("Expected success");
    let mut srcs = Sources::new();
    let src = srcs.add_src(filename).expect("Expected some");

    src.push_line(0, String::from("a"));
    src.push_line(2, String::from(""));
    src.push_line(3, String::from("bcde"));
    src.push_line(8, String::from("fghi"));
    src.push_line(13, String::from("jk"));

    let offset_pos = OffsetPosition::Point { point: Offset::from(1) };
    let ctx = srcs.get_ctx(filename, &offset_pos).expect("Expected some");

    assert_eq!(SourceContext::Single { prefix: "a", selected: "",
                                       suffix: "" },
               ctx)
}

#[test]
fn test_sources_point_endline_last() {
    let path = Path::new("./Cargo.toml");
    let mut filenames = Filenames::new();
    let filename = filenames.filename(&path).expect("Expected success");
    let mut srcs = Sources::new();
    let src = srcs.add_src(filename).expect("Expected some");

    src.push_line(0, String::from("a"));
    src.push_line(2, String::from(""));
    src.push_line(3, String::from("bcde"));
    src.push_line(8, String::from("fghi"));
    src.push_line(13, String::from("jk"));

    let offset_pos = OffsetPosition::Point { point: Offset::from(15) };
    let ctx = srcs.get_ctx(filename, &offset_pos).expect("Expected some");

    assert_eq!(SourceContext::Single { prefix: "jk", selected: "",
                                       suffix: "" },
               ctx)
}

#[test]
fn test_sources_point_empty_line() {
    let path = Path::new("./Cargo.toml");
    let mut filenames = Filenames::new();
    let filename = filenames.filename(&path).expect("Expected success");
    let mut srcs = Sources::new();
    let src = srcs.add_src(filename).expect("Expected some");

    src.push_line(0, String::from("a"));
    src.push_line(2, String::from(""));
    src.push_line(3, String::from("bcde"));
    src.push_line(8, String::from("fghi"));
    src.push_line(13, String::from("jk"));

    let offset_pos = OffsetPosition::Point { point: Offset::from(2) };
    let ctx = srcs.get_ctx(filename, &offset_pos).expect("Expected some");

    assert_eq!(SourceContext::Single { prefix: "", selected: "",
                                       suffix: "" },
               ctx)
}


#[test]
fn test_sources_point_middle() {
    let path = Path::new("./Cargo.toml");
    let mut filenames = Filenames::new();
    let filename = filenames.filename(&path).expect("Expected success");
    let mut srcs = Sources::new();
    let src = srcs.add_src(filename).expect("Expected some");

    src.push_line(0, String::from("a"));
    src.push_line(2, String::from(""));
    src.push_line(3, String::from("bcde"));
    src.push_line(8, String::from("fghi"));
    src.push_line(13, String::from("jk"));

    let offset_pos = OffsetPosition::Point { point: Offset::from(4) };
    let ctx = srcs.get_ctx(filename, &offset_pos).expect("Expected some");

    assert_eq!(SourceContext::Single { prefix: "b", selected: "c",
                                       suffix: "de" },
               ctx)
}

#[test]
fn test_sources_span_begin_len_1() {
    let path = Path::new("./Cargo.toml");
    let mut filenames = Filenames::new();
    let filename = filenames.filename(&path).expect("Expected success");
    let mut srcs = Sources::new();
    let src = srcs.add_src(filename).expect("Expected some");

    src.push_line(0, String::from("a"));
    src.push_line(2, String::from(""));
    src.push_line(3, String::from("bcde"));
    src.push_line(8, String::from("fghi"));
    src.push_line(13, String::from("jk"));

    let offset_pos = OffsetPosition::Span { start: Offset::from(0),
                                            len: Offset::from(1) };
    let ctx = srcs.get_ctx(filename, &offset_pos).expect("Expected some");

    assert_eq!(SourceContext::Single { prefix: "", selected: "a",
                                       suffix: "" },
               ctx)
}

#[test]
fn test_sources_span_endline_len_1() {
    let path = Path::new("./Cargo.toml");
    let mut filenames = Filenames::new();
    let filename = filenames.filename(&path).expect("Expected success");
    let mut srcs = Sources::new();
    let src = srcs.add_src(filename).expect("Expected some");

    src.push_line(0, String::from("a"));
    src.push_line(2, String::from(""));
    src.push_line(3, String::from("bcde"));
    src.push_line(8, String::from("fghi"));
    src.push_line(13, String::from("jk"));

    let offset_pos = OffsetPosition::Span { start: Offset::from(1),
                                            len: Offset::from(1) };
    let ctx = srcs.get_ctx(filename, &offset_pos).expect("Expected some");

    assert_eq!(SourceContext::Single { prefix: "a", selected: "",
                                       suffix: "" },
               ctx)
}

#[test]
fn test_sources_span_empty_line_len_1() {
    let path = Path::new("./Cargo.toml");
    let mut filenames = Filenames::new();
    let filename = filenames.filename(&path).expect("Expected success");
    let mut srcs = Sources::new();
    let src = srcs.add_src(filename).expect("Expected some");

    src.push_line(0, String::from("a"));
    src.push_line(2, String::from(""));
    src.push_line(3, String::from("bcde"));
    src.push_line(8, String::from("fghi"));
    src.push_line(13, String::from("jk"));

    let offset_pos = OffsetPosition::Span { start: Offset::from(2),
                                            len: Offset::from(1) };
      let ctx = srcs.get_ctx(filename, &offset_pos).expect("Expected some");

    assert_eq!(SourceContext::Single { prefix: "", selected: "",
                                       suffix: "" },
               ctx)
}

#[test]
fn test_sources_span_middle_len_1() {
    let path = Path::new("./Cargo.toml");
    let mut filenames = Filenames::new();
    let filename = filenames.filename(&path).expect("Expected success");
    let mut srcs = Sources::new();
    let src = srcs.add_src(filename).expect("Expected some");

    src.push_line(0, String::from("a"));
    src.push_line(2, String::from(""));
    src.push_line(3, String::from("bcde"));
    src.push_line(8, String::from("fghi"));
    src.push_line(13, String::from("jk"));

    let offset_pos = OffsetPosition::Span { start: Offset::from(4),
                                            len: Offset::from(1) };
    let ctx = srcs.get_ctx(filename, &offset_pos).expect("Expected some");

    assert_eq!(SourceContext::Single { prefix: "b", selected: "c",
                                       suffix: "de" },
               ctx)
}

#[test]
fn test_sources_span_middle() {
    let path = Path::new("./Cargo.toml");
    let mut filenames = Filenames::new();
    let filename = filenames.filename(&path).expect("Expected success");
    let mut srcs = Sources::new();
    let src = srcs.add_src(filename).expect("Expected some");

    src.push_line(0, String::from("a"));
    src.push_line(2, String::from(""));
    src.push_line(3, String::from("bcde"));
    src.push_line(8, String::from("fghi"));
    src.push_line(13, String::from("jk"));

    let offset_pos = OffsetPosition::Span { start: Offset::from(4),
                                            len: Offset::from(2) };
    let ctx = srcs.get_ctx(filename, &offset_pos).expect("Expected some");

    assert_eq!(SourceContext::Single { prefix: "b", selected: "cd",
                                       suffix: "e" },
               ctx)
}

#[test]
fn test_sources_span_start_line() {
    let path = Path::new("./Cargo.toml");
    let mut filenames = Filenames::new();
    let filename = filenames.filename(&path).expect("Expected success");
    let mut srcs = Sources::new();
    let src = srcs.add_src(filename).expect("Expected some");

    src.push_line(0, String::from("a"));
    src.push_line(2, String::from(""));
    src.push_line(3, String::from("bcde"));
    src.push_line(8, String::from("fghi"));
    src.push_line(13, String::from("jk"));

    let offset_pos = OffsetPosition::Span { start: Offset::from(3),
                                            len: Offset::from(3) };
    let ctx = srcs.get_ctx(filename, &offset_pos).expect("Expected some");

    assert_eq!(SourceContext::Single { prefix: "", selected: "bcd",
                                       suffix: "e" },
               ctx)
}

#[test]
fn test_sources_span_end_visible_line() {
    let path = Path::new("./Cargo.toml");
    let mut filenames = Filenames::new();
    let filename = filenames.filename(&path).expect("Expected success");
    let mut srcs = Sources::new();
    let src = srcs.add_src(filename).expect("Expected some");

    src.push_line(0, String::from("a"));
    src.push_line(2, String::from(""));
    src.push_line(3, String::from("bcde"));
    src.push_line(8, String::from("fghi"));
    src.push_line(13, String::from("jk"));

    let offset_pos = OffsetPosition::Span { start: Offset::from(4),
                                            len: Offset::from(3) };
    let ctx = srcs.get_ctx(filename, &offset_pos).expect("Expected some");

    assert_eq!(SourceContext::Single { prefix: "b", selected: "cde",
                                       suffix: "" },
               ctx)
}

#[test]
fn test_sources_span_end_whole_line() {
    let path = Path::new("./Cargo.toml");
    let mut filenames = Filenames::new();
    let filename = filenames.filename(&path).expect("Expected success");
    let mut srcs = Sources::new();
    let src = srcs.add_src(filename).expect("Expected some");

    src.push_line(0, String::from("a"));
    src.push_line(2, String::from(""));
    src.push_line(3, String::from("bcde"));
    src.push_line(8, String::from("fghi"));
    src.push_line(13, String::from("jk"));

    let offset_pos = OffsetPosition::Span { start: Offset::from(4),
                                            len: Offset::from(4) };
    let ctx = srcs.get_ctx(filename, &offset_pos).expect("Expected some");

    assert_eq!(SourceContext::Single { prefix: "b", selected: "cde",
                                       suffix: "" },
               ctx)
}

#[test]
fn test_sources_span_visible_line() {
    let path = Path::new("./Cargo.toml");
    let mut filenames = Filenames::new();
    let filename = filenames.filename(&path).expect("Expected success");
    let mut srcs = Sources::new();
    let src = srcs.add_src(filename).expect("Expected some");

    src.push_line(0, String::from("a"));
    src.push_line(2, String::from(""));
    src.push_line(3, String::from("bcde"));
    src.push_line(8, String::from("fghi"));
    src.push_line(13, String::from("jk"));

    let offset_pos = OffsetPosition::Span { start: Offset::from(3),
                                            len: Offset::from(4) };
    let ctx = srcs.get_ctx(filename, &offset_pos).expect("Expected some");

    assert_eq!(SourceContext::Single { prefix: "", selected: "bcde",
                                       suffix: "" },
               ctx)
}

#[test]
fn test_sources_span_whole_line() {
    let path = Path::new("./Cargo.toml");
    let mut filenames = Filenames::new();
    let filename = filenames.filename(&path).expect("Expected success");
    let mut srcs = Sources::new();
    let src = srcs.add_src(filename).expect("Expected some");

    src.push_line(0, String::from("a"));
    src.push_line(2, String::from(""));
    src.push_line(3, String::from("bcde"));
    src.push_line(8, String::from("fghi"));
    src.push_line(13, String::from("jk"));

    let offset_pos = OffsetPosition::Span { start: Offset::from(3),
                                            len: Offset::from(5) };
    let ctx = srcs.get_ctx(filename, &offset_pos).expect("Expected some");

    assert_eq!(SourceContext::Single { prefix: "", selected: "bcde",
                                       suffix: "" },
               ctx)
}

#[test]
fn test_sources_span_two_lines_middle() {
    let path = Path::new("./Cargo.toml");
    let mut filenames = Filenames::new();
    let filename = filenames.filename(&path).expect("Expected success");
    let mut srcs = Sources::new();
    let src = srcs.add_src(filename).expect("Expected some");

    src.push_line(0, String::from("a"));
    src.push_line(2, String::from(""));
    src.push_line(3, String::from("bcde"));
    src.push_line(8, String::from("fghi"));
    src.push_line(13, String::from("jk"));

    let offset_pos = OffsetPosition::Span { start: Offset::from(4),
                                            len: Offset::from(6) };
    let ctx = srcs.get_ctx(filename, &offset_pos).expect("Expected some");

    assert_eq!(SourceContext::Multiple { prefix: "b", first: "cde",
                                         middle: &vec![], last: "fg",
                                         suffix: "hi" },
               ctx)
}

#[test]
fn test_sources_span_two_lines_start_line() {
    let path = Path::new("./Cargo.toml");
    let mut filenames = Filenames::new();
    let filename = filenames.filename(&path).expect("Expected success");
    let mut srcs = Sources::new();
    let src = srcs.add_src(filename).expect("Expected some");

    src.push_line(0, String::from("a"));
    src.push_line(2, String::from(""));
    src.push_line(3, String::from("bcde"));
    src.push_line(8, String::from("fghi"));
    src.push_line(13, String::from("jk"));

    let offset_pos = OffsetPosition::Span { start: Offset::from(3),
                                            len: Offset::from(7) };
    let ctx = srcs.get_ctx(filename, &offset_pos).expect("Expected some");

    assert_eq!(SourceContext::Multiple { prefix: "", first: "bcde",
                                         middle: &vec![], last: "fg",
                                         suffix: "hi" },
               ctx)
}

#[test]
fn test_sources_span_two_lines_end_visible_line() {
    let path = Path::new("./Cargo.toml");
    let mut filenames = Filenames::new();
    let filename = filenames.filename(&path).expect("Expected success");
    let mut srcs = Sources::new();
    let src = srcs.add_src(filename).expect("Expected some");

    src.push_line(0, String::from("a"));
    src.push_line(2, String::from(""));
    src.push_line(3, String::from("bcde"));
    src.push_line(8, String::from("fghi"));
    src.push_line(13, String::from("jk"));

    let offset_pos = OffsetPosition::Span { start: Offset::from(4),
                                            len: Offset::from(8) };
    let ctx = srcs.get_ctx(filename, &offset_pos).expect("Expected some");

    assert_eq!(SourceContext::Multiple { prefix: "b", first: "cde",
                                         middle: &vec![], last: "fghi",
                                         suffix: "" },
               ctx)
}

#[test]
fn test_sources_span_two_lines_end_whole_line() {
    let path = Path::new("./Cargo.toml");
    let mut filenames = Filenames::new();
    let filename = filenames.filename(&path).expect("Expected success");
    let mut srcs = Sources::new();
    let src = srcs.add_src(filename).expect("Expected some");

    src.push_line(0, String::from("a"));
    src.push_line(2, String::from(""));
    src.push_line(3, String::from("bcde"));
    src.push_line(8, String::from("fghi"));
    src.push_line(13, String::from("jk"));

    let offset_pos = OffsetPosition::Span { start: Offset::from(4),
                                            len: Offset::from(9) };
    let ctx = srcs.get_ctx(filename, &offset_pos).expect("Expected some");

    assert_eq!(SourceContext::Multiple { prefix: "b", first: "cde",
                                         middle: &vec![], last: "fghi",
                                         suffix: "" },
               ctx)
}

#[test]
fn test_sources_span_two_lines_visible_line() {
    let path = Path::new("./Cargo.toml");
    let mut filenames = Filenames::new();
    let filename = filenames.filename(&path).expect("Expected success");
    let mut srcs = Sources::new();
    let src = srcs.add_src(filename).expect("Expected some");

    src.push_line(0, String::from("a"));
    src.push_line(2, String::from(""));
    src.push_line(3, String::from("bcde"));
    src.push_line(8, String::from("fghi"));
    src.push_line(13, String::from("jk"));

    let offset_pos = OffsetPosition::Span { start: Offset::from(3),
                                            len: Offset::from(9) };
    let ctx = srcs.get_ctx(filename, &offset_pos).expect("Expected some");

    assert_eq!(SourceContext::Multiple { prefix: "", first: "bcde",
                                         middle: &vec![], last: "fghi",
                                         suffix: "" },
               ctx)
}

#[test]
fn test_sources_span_two_lines_whole_line() {
    let path = Path::new("./Cargo.toml");
    let mut filenames = Filenames::new();
    let filename = filenames.filename(&path).expect("Expected success");
    let mut srcs = Sources::new();
    let src = srcs.add_src(filename).expect("Expected some");

    src.push_line(0, String::from("a"));
    src.push_line(2, String::from(""));
    src.push_line(3, String::from("bcde"));
    src.push_line(8, String::from("fghi"));
    src.push_line(13, String::from("jk"));

    let offset_pos = OffsetPosition::Span { start: Offset::from(3),
                                            len: Offset::from(10) };
    let ctx = srcs.get_ctx(filename, &offset_pos).expect("Expected some");

    assert_eq!(SourceContext::Multiple { prefix: "", first: "bcde",
                                         middle: &vec![], last: "fghi",
                                         suffix: "" },
               ctx)
}

#[test]
fn test_sources_span_three_lines_middle() {
    let path = Path::new("./Cargo.toml");
    let mut filenames = Filenames::new();
    let filename = filenames.filename(&path).expect("Expected success");
    let mut srcs = Sources::new();
    let src = srcs.add_src(filename).expect("Expected some");

    src.push_line(0, String::from("a"));
    src.push_line(2, String::from(""));
    src.push_line(3, String::from("bcde"));
    src.push_line(8, String::from("fghi"));
    src.push_line(13, String::from("jk"));

    let offset_pos = OffsetPosition::Span { start: Offset::from(4),
                                            len: Offset::from(10) };
    let ctx = srcs.get_ctx(filename, &offset_pos).expect("Expected some");

    assert_eq!(SourceContext::Multiple { prefix: "b", first: "cde",
                                         middle: &vec![String::from("fghi")],
                                         last: "j", suffix: "k" },
               ctx)
}

#[test]
fn test_sources_span_three_lines_start_line() {
    let path = Path::new("./Cargo.toml");
    let mut filenames = Filenames::new();
    let filename = filenames.filename(&path).expect("Expected success");
    let mut srcs = Sources::new();
    let src = srcs.add_src(filename).expect("Expected some");

    src.push_line(0, String::from("a"));
    src.push_line(2, String::from(""));
    src.push_line(3, String::from("bcde"));
    src.push_line(8, String::from("fghi"));
    src.push_line(13, String::from("jk"));

    let offset_pos = OffsetPosition::Span { start: Offset::from(3),
                                            len: Offset::from(11) };
    let ctx = srcs.get_ctx(filename, &offset_pos).expect("Expected some");

    assert_eq!(SourceContext::Multiple { prefix: "", first: "bcde",
                                         middle: &vec![String::from("fghi")],
                                         last: "j", suffix: "k" },
               ctx)
}

#[test]
fn test_sources_span_three_lines_end_visible_line() {
    let path = Path::new("./Cargo.toml");
    let mut filenames = Filenames::new();
    let filename = filenames.filename(&path).expect("Expected success");
    let mut srcs = Sources::new();
    let src = srcs.add_src(filename).expect("Expected some");

    src.push_line(0, String::from("a"));
    src.push_line(2, String::from(""));
    src.push_line(3, String::from("bcde"));
    src.push_line(8, String::from("fghi"));
    src.push_line(13, String::from("jk"));

    let offset_pos = OffsetPosition::Span { start: Offset::from(4),
                                            len: Offset::from(11) };
    let ctx = srcs.get_ctx(filename, &offset_pos).expect("Expected some");

    assert_eq!(SourceContext::Multiple { prefix: "b", first: "cde",
                                         middle: &vec![String::from("fghi")],
                                         last: "jk", suffix: "" },
               ctx)
}

#[test]
fn test_sources_span_three_lines_end_whole_line() {
    let path = Path::new("./Cargo.toml");
    let mut filenames = Filenames::new();
    let filename = filenames.filename(&path).expect("Expected success");
    let mut srcs = Sources::new();
    let src = srcs.add_src(filename).expect("Expected some");

    src.push_line(0, String::from("a"));
    src.push_line(2, String::from(""));
    src.push_line(3, String::from("bcde"));
    src.push_line(8, String::from("fghi"));
    src.push_line(13, String::from("jk"));
    src.push_line(16, String::from("lm"));

    let offset_pos = OffsetPosition::Span { start: Offset::from(4),
                                            len: Offset::from(12) };
    let ctx = srcs.get_ctx(filename, &offset_pos).expect("Expected some");

    assert_eq!(SourceContext::Multiple { prefix: "b", first: "cde",
                                         middle: &vec![String::from("fghi")],
                                         last: "jk", suffix: "" },
               ctx)
}

#[test]
fn test_sources_span_three_lines_visible_line() {
    let path = Path::new("./Cargo.toml");
    let mut filenames = Filenames::new();
    let filename = filenames.filename(&path).expect("Expected success");
    let mut srcs = Sources::new();
    let src = srcs.add_src(filename).expect("Expected some");

    src.push_line(0, String::from("a"));
    src.push_line(2, String::from(""));
    src.push_line(3, String::from("bcde"));
    src.push_line(8, String::from("fghi"));
    src.push_line(13, String::from("jk"));

    let offset_pos = OffsetPosition::Span { start: Offset::from(3),
                                            len: Offset::from(12) };
    let ctx = srcs.get_ctx(filename, &offset_pos).expect("Expected some");

    assert_eq!(SourceContext::Multiple { prefix: "", first: "bcde",
                                         middle: &vec![String::from("fghi")],
                                         last: "jk", suffix: "" },
               ctx)
}

#[test]
fn test_sources_span_three_lines_whole_line() {
    let path = Path::new("./Cargo.toml");
    let mut filenames = Filenames::new();
    let filename = filenames.filename(&path).expect("Expected success");
    let mut srcs = Sources::new();
    let src = srcs.add_src(filename).expect("Expected some");

    src.push_line(0, String::from("a"));
    src.push_line(2, String::from(""));
    src.push_line(3, String::from("bcde"));
    src.push_line(8, String::from("fghi"));
    src.push_line(13, String::from("jk"));
    src.push_line(16, String::from("lm"));

    let offset_pos = OffsetPosition::Span { start: Offset::from(3),
                                            len: Offset::from(13) };
    let ctx = srcs.get_ctx(filename, &offset_pos).expect("Expected some");

    assert_eq!(SourceContext::Multiple { prefix: "", first: "bcde",
                                         middle: &vec![String::from("fghi")],
                                         last: "jk", suffix: "" },
               ctx)
}

#[test]
fn test_sources_span_three_lines_start() {
    let path = Path::new("./Cargo.toml");
    let mut filenames = Filenames::new();
    let filename = filenames.filename(&path).expect("Expected success");
    let mut srcs = Sources::new();
    let src = srcs.add_src(filename).expect("Expected some");

    src.push_line(0, String::from("a"));
    src.push_line(2, String::from(""));
    src.push_line(3, String::from("bcde"));
    src.push_line(8, String::from("fghi"));
    src.push_line(13, String::from("jk"));

    let offset_pos = OffsetPosition::Span { start: Offset::from(0),
                                            len: Offset::from(5) };
    let ctx = srcs.get_ctx(filename, &offset_pos).expect("Expected some");

    assert_eq!(SourceContext::Multiple { prefix: "", first: "a",
                                         middle: &vec![String::from("")],
                                         last: "bc", suffix: "de" },
               ctx)
}

#[test]
fn test_sources_span_three_lines_first_empty() {
    let path = Path::new("./Cargo.toml");
    let mut filenames = Filenames::new();
    let filename = filenames.filename(&path).expect("Expected success");
    let mut srcs = Sources::new();
    let src = srcs.add_src(filename).expect("Expected some");

    src.push_line(0, String::from("a"));
    src.push_line(2, String::from(""));
    src.push_line(3, String::from("bcde"));
    src.push_line(8, String::from("fghi"));
    src.push_line(13, String::from("jk"));

    let offset_pos = OffsetPosition::Span { start: Offset::from(2),
                                            len: Offset::from(8) };
    let ctx = srcs.get_ctx(filename, &offset_pos).expect("Expected some");

    assert_eq!(SourceContext::Multiple { prefix: "", first: "",
                                         middle: &vec![String::from("bcde")],
                                         last: "fg", suffix: "hi" },
               ctx)
}

#[test]
fn test_sources_span_three_lines_last_empty() {
    let path = Path::new("./Cargo.toml");
    let mut filenames = Filenames::new();
    let filename = filenames.filename(&path).expect("Expected success");
    let mut srcs = Sources::new();
    let src = srcs.add_src(filename).expect("Expected some");

    src.push_line(0, String::from("a"));
    src.push_line(2, String::from(""));
    src.push_line(3, String::from("bcde"));
    src.push_line(8, String::from("fghi"));
    src.push_line(13, String::from(""));
    src.push_line(14, String::from("jk"));

    let offset_pos = OffsetPosition::Span { start: Offset::from(5),
                                            len: Offset::from(9) };
    let ctx = srcs.get_ctx(filename, &offset_pos).expect("Expected some");

    assert_eq!(SourceContext::Multiple { prefix: "bc", first: "de",
                                         middle: &vec![String::from("fghi")],
                                         last: "", suffix: "" },
               ctx)
}

#[test]
fn test_sources_span_three_lines_first_last_empty() {
    let path = Path::new("./Cargo.toml");
    let mut filenames = Filenames::new();
    let filename = filenames.filename(&path).expect("Expected success");
    let mut srcs = Sources::new();
    let src = srcs.add_src(filename).expect("Expected some");

    src.push_line(0, String::from("a"));
    src.push_line(2, String::from(""));
    src.push_line(3, String::from("bcde"));
    src.push_line(8, String::from(""));
    src.push_line(9, String::from("fghi"));
    src.push_line(14, String::from("jk"));

    let offset_pos = OffsetPosition::Span { start: Offset::from(2),
                                            len: Offset::from(7) };
    let ctx = srcs.get_ctx(filename, &offset_pos).expect("Expected some");

    assert_eq!(SourceContext::Multiple { prefix: "", first: "",
                                         middle: &vec![String::from("bcde")],
                                         last: "", suffix: "" },
               ctx)
}

#[test]
fn test_sources_span_three_lines_all_empty() {
    let path = Path::new("./Cargo.toml");
    let mut filenames = Filenames::new();
    let filename = filenames.filename(&path).expect("Expected success");
    let mut srcs = Sources::new();
    let src = srcs.add_src(filename).expect("Expected some");

    src.push_line(0, String::from("a"));
    src.push_line(2, String::from(""));
    src.push_line(3, String::from(""));
    src.push_line(4, String::from(""));
    src.push_line(5, String::from("jk"));

    let offset_pos = OffsetPosition::Span { start: Offset::from(2),
                                            len: Offset::from(3) };
    let ctx = srcs.get_ctx(filename, &offset_pos).expect("Expected some");

    assert_eq!(SourceContext::Multiple { prefix: "", first: "",
                                         middle: &vec![String::from("")],
                                         last: "", suffix: "" },
               ctx)
}
