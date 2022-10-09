use crate::files::FileOffsets;
use crate::files::Filename;
use crate::lines::LineOffsets;
use crate::lines::Offset;
use crate::nondistinct::Nondistinct;
use std::convert::TryFrom;
use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;

/// Trait for things that have a position of type `P`.
pub trait Positioned<P> {
    /// Get the position.
    fn position(&self) -> &P;
}

/// Component of a position describing a portion of a file.
///
/// These are described with absolute offsets, not line/column
/// offsets.  These are intended to then be interpreted using
/// a [`LineOffsets`] structure.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialOrd, PartialEq)]
pub enum OffsetPosition {
    /// A span of some length.
    Span {
        /// Starting offset.
        start: Offset,
        /// Span length.
        len: Offset
    },
    /// A single point.
    Point {
        /// Offset of the point.
        point: Offset
    }
}

/// A position within a single file.
///
/// This can describe the entire file, or a portion of it.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialOrd, PartialEq)]
pub enum FilePosition<'a> {
    /// A specific portion of an input file.
    Portion {
        /// The name of the file.
        file_offsets: &'a FileOffsets<'a>,
        /// The position of the portion of the file.
        offset: OffsetPosition
    },
    /// A position representing an entire file.
    File {
        /// The name of the file.
        filename: Filename<'a>
    }
}

/// Basic position structure, intended to cover the cases seen by the
/// front-end portion of a compiler.
///
/// Note that for most AST and IR structures, it is a good idea to
/// wrap a `BasicPosition` with a [`Nondistinct`].
#[derive(Clone, Debug, Eq, Hash, Ord, PartialOrd, PartialEq)]
pub enum BasicPosition<'a> {
    /// A position within a source file.
    File {
        /// The file position data.
        pos: FilePosition<'a>
    },
    /// A specific portion of the input stream.
    ///
    /// This is intended primarily for interpreters, or for compilers
    /// that can accept code from an input stream.
    Input {
        /// Position of the portion of input.
        offset: OffsetPosition,
        /// [`LineOffsets`] structure for input.
        line_offsets: Nondistinct<&'a LineOffsets>
    },
    /// A synthetic position, generated internally by a compiler.
    ///
    /// This is primarily for debugging, testing, or for
    /// internally-generated structures; these should typically not be
    /// seen by ordinary users under most circumstances.
    Synthetic {
        /// Description of the origin of this position.
        desc: String
    },
    /// A position representing the command line.
    CmdLine {
        args: Vec<usize>
    }
}
/*
#[derive(Clone, Debug, Eq, Hash, Ord, PartialOrd, PartialEq)]
pub enum DWARFPosition<'a, DefID, TyDefID> {
    /// Position within a regular definition.
    Def {
        /// Id of the definition.
        id: DefID,
        /// Position within the definition.
        pos: OffsetPosition
    },
    /// Position within a type definition.
    TypeDef {
        /// Id of the definition.
        id: TyDefID,
        /// Position within the definition.
        pos: OffsetPosition
    },
    /// A position within a block.
    Block {
        /// Position of the whole enclosing block.
        pos: Box<DWARFPosition<'a, DefID, TyDefID>>,
        /// Position within the block.
        offset: OffsetPosition
    },
    /// A position within a file.
    File {
        /// The file position data.
        pos: FilePosition<'a>
    },
    /// A specific portion of the input stream.
    Input {
        /// Position of the portion of input
        offset: OffsetPosition
    },
    /// A synthetic position, generated internally by a compiler.
    Synthetic {
        /// Description of the origin of this position.
        desc: String
    },
    /// A position representing the command line.
    CmdLine
}
*/
impl Display for FilePosition<'_> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            FilePosition::Portion { file_offsets, offset } => match offset {
                OffsetPosition::Span { start, len } => {
                    let end = *start + *len;
                    let (start_line, start_col) = file_offsets
                        .line_offsets().lookup(*start);
                    let (end_line, end_col) = file_offsets
                        .line_offsets().lookup(end);

                    if start_line == end_line {
                        write!(f, "at {}:{}.{}-{}", file_offsets.filename(),
                               start_line, start_col, end_col)
                    } else {
                        write!(f, "at {}:{}.{}-{}.{}", file_offsets.filename(),
                               start_line, start_col, end_line, end_col)
                    }
                },
                OffsetPosition::Point { point } => {
                    let (line, col) = file_offsets
                        .line_offsets().lookup(*point);

                    write!(f, "at {}:{}.{}", file_offsets.filename(), line, col)
                }
            },
            FilePosition::File { filename } => {
                write!(f, "in {}", filename)
            }
        }
    }
}

impl Display for BasicPosition<'_> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            BasicPosition::File { pos } => pos.fmt(f),
            BasicPosition::Input { offset, line_offsets } => match offset {
                OffsetPosition::Span { start, len } => {
                    let end = *start + *len;
                    let (start_line, start_col) =
                        line_offsets.val.lookup(*start);
                    let (end_line, end_col) =
                        line_offsets.val.lookup(end);

                    if start_line == end_line {
                        write!(f, "at input {}.{}-{}",start_line,
                               start_col, end_col)
                    } else {
                        write!(f, "at input {}.{}-{}.{}", start_line,
                               start_col, end_line, end_col)
                    }
                },
                OffsetPosition::Point { point } => {
                    let (line, col) = line_offsets.val.lookup(*point);

                    write!(f, "at input {}.{}", line, col)
                }
            },
            BasicPosition::Synthetic { desc } => desc.fmt(f),
            BasicPosition::CmdLine { .. } => {
                write!(f, "from command line")
            }
        }
    }
}

impl<'a> TryFrom<BasicPosition<'a>> for FilePosition<'a> {
    type Error = ();

    fn try_from(val: BasicPosition<'a>) -> Result<Self, Self::Error> {
        match val {
            BasicPosition::File { pos } => Ok(pos),
            _ => Err(())
        }
    }
}

impl<'a> TryFrom<&'a BasicPosition<'a>> for &'a FilePosition<'a> {
    type Error = ();

    fn try_from(val: &'a BasicPosition<'a>) -> Result<Self, Self::Error> {
        match val {
            BasicPosition::File { pos } => Ok(pos),
            _ => Err(())
        }
    }
}
