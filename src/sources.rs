use crate::files::Filename;
use crate::lines::LineOffsets;
use crate::position::OffsetPosition;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub struct Source {
    content: Vec<String>,
    line_offsets: LineOffsets
}

pub struct Sources<'a> {
    /// The table of all sources.
    files: HashMap<Filename<'a>, Source>
}

/// Source context, retrieved from a [`FilePosition`].
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
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

impl Source {
    /// Create a new `Source`.
    #[inline]
    fn new() -> Self {
        Source { content: Vec::new(), line_offsets: LineOffsets::new() }
    }

    /// Create a new `Source` with a size hint.
    #[inline]
    fn with_capacity(nlines: usize) -> Self {
        Source { line_offsets: LineOffsets::with_capacity(nlines),
                 content: Vec::with_capacity(nlines) }
    }

    /// Shrink the structures of this `Source` to fit its current size.
    #[inline]
    pub fn shrink_to_fit(&mut self) {
        self.line_offsets.shrink_to_fit();
        self.content.shrink_to_fit();
    }

    #[inline]
    pub fn push_line(&mut self, start: usize, line: String) {
        self.content.push(line);
        self.line_offsets.push_line(start);
    }
}

impl<'a> Sources<'a> {
    /// Create a new `Sources`.
    #[inline]
    pub fn new() -> Self {
        Sources { files: HashMap::new() }
    }

    /// Create a new `Sources` with a size hint for the number of
    /// files it will contain.
    #[inline]
    pub fn with_capacity(nfiles: usize) -> Self {
        Sources { files: HashMap::with_capacity(nfiles) }
    }

    /// Shrink the structures of this `Sources` to fit its current size.
    #[inline]
    pub fn shrink_to_fit(&mut self) {
        self.files.shrink_to_fit()
    }

    /// Get the [`SourceContext`] for a given [`OffsetPosition`] in `file`.
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
                    let end_line = if end_col != 0 {
                        end_line
                    } else {
                        end_line - 1
                    };

                    if start_line == end_line {
                        let content = &src.content[start_line - 1];
                        let end_col = if end_col != 0 {
                            end_col
                        } else {
                            content.len()
                        };
                        let (prefix, rest) = content.split_at(start_col);

                        let out = if end_col < content.len() {
                            let (selected, suffix) = rest.split_at(len.into());

                            SourceContext::Single {
                                prefix: prefix, selected: selected,
                                suffix: suffix
                            }
                        } else {
                            SourceContext::Single {
                                prefix: prefix, selected: rest, suffix: ""
                            }
                        };

                        Some(out)
                    } else {
                        let content = &src.content[start_line - 1];
                        let (prefix, first) = content.split_at(start_col);
                        let middle = &src.content[start_line .. end_line - 1];
                        let content = &src.content[end_line - 1];
                        let end_col = if end_col != 0 {
                            end_col
                        } else {
                            content.len()
                        };
                        let out = if end_col < content.len() {
                            let (last, suffix) = content.split_at(end_col);

                            SourceContext::Multiple {
                                prefix: prefix, first: first, middle: middle,
                                last: last, suffix: suffix
                            }
                        } else {
                            SourceContext::Multiple {
                                prefix: prefix, first: first, middle: middle,
                                last: content, suffix: ""
                            }
                        };

                        Some(out)
                    }
                },
                OffsetPosition::Point { point } => {
                    let (line, col) = src.line_offsets.lookup(*point);
                    let content = &src.content[line - 1];
                    let (prefix, rest) = content.split_at(col);

                    let out = if col < content.len() {
                        let (selected, suffix) = rest.split_at(1);

                        SourceContext::Single {
                            prefix: prefix, selected: selected, suffix: suffix
                        }
                    } else {
                        SourceContext::Single {
                            prefix: prefix, selected: rest, suffix: ""
                        }
                    };

                    Some(out)
                }
            },
            None => None
        }
    }

    /// Add a [`Source`] for `filename` if it doesn't already exist,
    /// and return a mutable reference to it.
    #[inline]
    pub fn add_src(&mut self, filename: Filename<'a>) -> Option<&mut Source> {
        match self.files.entry(filename) {
            Entry::Vacant(ent) => Some(ent.insert(Source::new())),
            Entry::Occupied(_) => None
        }
    }

    /// Add a [`Source`] for `filename` if it doesn't already exist,
    /// with a size hint, and return a mutable reference to it.
    #[inline]
    pub fn add_src_with_capacity(&mut self, filename: Filename<'a>,
                                 nlines: usize) -> Option<&mut Source> {
        match self.files.entry(filename) {
            Entry::Vacant(ent) =>
                Some(ent.insert(Source::with_capacity(nlines))),
            Entry::Occupied(_) => None
        }
    }
}
