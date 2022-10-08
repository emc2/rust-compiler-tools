use crate::files::Filename;
use crate::lines::LineOffsets;
use crate::position::OffsetPosition;
use std::collections::HashMap;

struct Source {
    content: Vec<String>,
    line_offsets: LineOffsets
}

pub struct Sources<'a> {
    files: HashMap<Filename<'a>, Source>
}

/// Source context, retrieved from a [`FilePosition`].
pub enum SourceContext<'a> {
    /// Source context consisting of single lines.
    Single {
        /// Part of the line before the selected region.
        prefix: &'a str,
        /// The selected region.
        selected: &'a str,
        /// Part of the line after the selected region.
        suffix: &'a str
    },
    /// Source context consisting of multiple lines.
    Multiple {
        /// Part of the line before the selected region.
        prefix: &'a str,
        /// The part of the selected region on the first line.
        first: &'a str,
        /// The middle lines of the selected region.
        middle: &'a [String],
        /// The part of the selected region on the last line.
        last: &'a str,
        /// Part of the line after the selected region.
        suffix: &'a str
    }
}

impl<'a> Sources<'a> {
    pub fn get_ctx(&'a self, file: Filename<'a>, pos: &'a OffsetPosition) ->
        Option<SourceContext<'a>> {
        match self.files.get(&file) {
            Some(src) => match pos {
                OffsetPosition::Span { start, len } => {
                    let end = *start + *len;
                    let (start_line, start_col) =
                        src.line_offsets.lookup(*start);
                    let (end_line, end_col) =
                        src.line_offsets.lookup(end);

                    if start_line == end_line {
                        let content = &src.content[start_line];
                        let (prefix, rest) = content.split_at(start_col);
                        let (selected, suffix) = rest.split_at(len.into());
                        let out = SourceContext::Single {
                            prefix: prefix, selected: selected, suffix: suffix
                        };

                        Some(out)
                    } else {
                        let content = &src.content[start_line];
                        let (prefix, first) = content.split_at(start_col);
                        let middle = &src.content[start_line + 1 ..
                                                  end_line - 1];
                        let content = &src.content[start_line];
                        let (last, suffix) = content.split_at(end_col);
                        let out = SourceContext::Multiple {
                            prefix: prefix, first: first, middle: middle,
                            last: last, suffix: suffix
                        };

                        Some(out)
                    }
                },
                OffsetPosition::Point { point } => {
                    let (line, col) = src.line_offsets.lookup(*point);
                    let content = &src.content[line];
                    let (prefix, rest) = content.split_at(col);
                    let (selected, suffix) = rest.split_at(1);
                    let out = SourceContext::Single {
                        prefix: prefix, selected: selected, suffix: suffix
                    };

                    Some(out)
                }
            },
            None => None
        }
    }
}
