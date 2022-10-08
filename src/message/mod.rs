use crate::position::FilePosition;
use crate::sources::Sources;
use crate::sources::SourceContext;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::fmt::Display;
use std::fmt::Formatter;
use std::io::Error;
use termcolor::Color;
use termcolor::ColorSpec;
use termcolor::WriteColor;

/// Trait for compiler messages.
pub trait Message {
    /// Get the [`Severity`] of the message.
    fn severity(&self) -> Severity;

    /// Get a brief human-readable description of the error.
    fn brief(&self) -> &str;

    /// Get a detailed human-readable description of the error.
    fn detail(&self) -> &str;

    /// Get the highlighting style for the message.
    ///
    /// This will determine how the relevant text is highlighted.
    #[inline]
    fn highlighting(&self) -> Highlighting {
        Highlighting::Foreground
    }
}

/// Trait for compiler messages with positions.
///
/// This will apply to almost all compiler messages.
pub trait MessagePositions<P>: Message  {
    /// Get a set of positions, with a severity and optional message
    /// to display.
    ///
    /// The default behavior returns an empty slice.
    #[inline]
    fn positions(&self) -> &[(Option<&str>, P, Severity)] {
        &[]
    }
}

/// Trait for modes of writing out compiler messages.
pub trait MessageWriter {
    /// Write a message out to the `stream`.
    fn write_msg<'a, M, P, W>(&self, msg: &'a M, out: &mut W) ->
        Result<(), Error>
    where &'a FilePosition<'a>: TryFrom<&'a P>,
          M: MessagePositions<P>,
          W: WriteColor,
          P: 'a + Display;
}

/// Message severity levels.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Severity {
    /// Purely informative message, not implying any sort of criticism.
    ///
    /// These should only be printed if a verbose option is given.
    Info,
    /// A purely stylistic criticism.
    ///
    /// These should not be displayed unless a specific option is given.
    Remark,
    /// Ax criticism.
    ///
    /// These represent code that may be inefficient, inhibiting
    /// optimization, or overly verbose, but not dangerous or severe
    /// enough to warrant a warning.
    Lint,
    /// A warning.
    ///
    /// These represent code that is dangerous, or severely
    /// inefficient enough to terminate compilation if an option is
    /// given, but that does not inhibit the completion of
    /// compilation.
    Warning,
    /// An error, which should terminate compilation.
    ///
    /// These refer to hard syntactic or semantic errors detected in
    /// the program.
    Error,
    /// Internal compiler error.
    ///
    /// These are for reporting compiler bugs, not problems in the
    /// source program.  These will likely terminate compilation.
    Internal
}

/// Highlighting styles.
///
/// These determine how relevant text is highlighted.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Highlighting {
    /// Highlight the text itself, in the foreground.
    Foreground,
    /// Highlight the background.
    ///
    /// This is useful for highlighting whitespace.
    Background
}

/// A [`MessageWriter`] that writes human-readable messages with full
/// context.
///
/// This will print and highlight the code that gave rise to the error.
pub struct MessageFullWriter<'a> {
    sources: &'a Sources<'a>
}

/// A [`MessageWriter`] that writes human-readable messages without context.
///
/// This will print basic descriptions and positions, but will not
/// print and highlight code.
pub struct MessageSimpleWriter;

pub struct MessageMinimalWriter;

impl Severity {
    /// Get the [`Color`] for this `Severity` level.
    #[inline]
    pub fn color(&self) -> Color {
        match self {
            Severity::Info => Color::Green,
            Severity::Remark => Color::Blue,
            Severity::Lint => Color::Cyan,
            Severity::Warning => Color::Yellow,
            Severity::Error => Color::Red,
            Severity::Internal => Color::Magenta
        }
    }

    /// Write a colored version of this `Severity` to the terminal
    /// `out` if it supports color.
    ///
    /// The color will be written with high intensity set.  If the
    /// terminal does not support colors, then the plain severity
    /// level will be written out.
    #[inline]
    pub fn write_color<W>(&self, out: &mut W) -> Result<(), Error>
    where W: WriteColor {
        if out.supports_color() {
            out.set_color(ColorSpec::new()
                          .set_fg(Some(self.color()))
                          .set_intense(true))?;
            write!(out, "{}", self)?;
            out.reset()
        } else {
            write!(out, "{}", self)
        }
    }
}

impl Display for Severity {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Severity::Info => write!(f, "Info"),
            Severity::Lint => write!(f, "Lint Warning"),
            Severity::Remark => write!(f, "Remark"),
            Severity::Warning => write!(f, "Warning"),
            Severity::Error => write!(f, "Error"),
            Severity::Internal => write!(f, "Internal Error")
        }
    }
}

impl MessageWriter for MessageFullWriter<'_> {
    fn write_msg<'a, M, P, W>(&self, msg: &'a M, out: &mut W) ->
        Result<(), Error>
    where &'a FilePosition<'a>: TryFrom<&'a P>,
          M: MessagePositions<P>,
          W: WriteColor,
          P: 'a + Display {
        msg.severity().write_color(out)?;

        if out.supports_color() {
            out.set_color(ColorSpec::new().set_bold(true))?;
            write!(out, ": {}", msg.severity())?;
            out.reset()?;
        } else {
            write!(out, ": {}", msg.severity())?;
        }

        let mut first = true;

        for (label, pos, severity) in msg.positions() {
            let mut color = ColorSpec::new();

            color.set_fg(Some(severity.color()));

            match label {
                Some(label) if first => {
                    writeln!(out)?;
                    write!(out, "  {}", label)?
                },
                Some(label) => write!(out, "  {}", label)?,
                None => ()
            }

            write!(out, " {}:", pos)?;

            let filepos: Result<&'a FilePosition<'a>, _> = pos.try_into();

            match filepos {
                Ok(FilePosition::Portion { offset, file_offsets}) => {

                    match self.sources.get_ctx(file_offsets.filename(),
                                               offset) {
                        Some(SourceContext::Single { prefix, selected,
                                                     suffix }) => {
                            writeln!(out, " {}:", pos)?;
                            write!(out, "{}", prefix)?;

                            if out.supports_color() {
                                out.set_color(&color)?;
                                write!(out, "{}", selected)?;
                                out.reset()?;
                            } else {
                                write!(out, "{}", selected)?;
                            }

                            writeln!(out, "{}", suffix)?;
                        },
                        Some(SourceContext::Multiple { prefix, first, middle,
                                                       last, suffix }) => {
                            writeln!(out, " {}:", pos)?;
                            write!(out, "{}", prefix)?;

                            if out.supports_color() {
                                out.set_color(&color)?;
                                write!(out, "{}", first)?;
                            } else {
                                writeln!(out, "{}", first)?;
                            }

                            let nlines = middle.len();

                            if nlines > 6 {
                                writeln!(out, "{}", middle[0])?;
                                writeln!(out, "{}", middle[1])?;
                                writeln!(out, "{}", middle[2])?;

                                if out.supports_color() {
                                    out.reset()?;
                                    writeln!(out, "...")?;
                                    out.set_color(&color)?;
                                } else {
                                    writeln!(out, "...")?;
                                }

                                writeln!(out, "{}", middle[nlines - 3])?;
                                writeln!(out, "{}", middle[nlines - 2])?;
                                writeln!(out, "{}", middle[nlines - 1])?;
                            } else {
                                for line in middle {
                                    writeln!(out, "{}", line)?;
                                }
                            }

                            write!(out, "{}", last)?;

                            if out.supports_color() {
                                out.reset()?;
                                write!(out, "{}", suffix)?;
                            } else {
                                writeln!(out, "{}", suffix)?;
                            }
                        },
                        None => {
                            write!(out, " {}", pos)?;
                        }
                    }
                },
                _ => {
                    write!(out, " {}", pos)?;
                }
            }

            first = false;
        }

        writeln!(out, "{}", msg.detail())
    }
}

impl MessageWriter for MessageSimpleWriter {
    fn write_msg<'a, M, P, W>(&self, msg: &'a M, out: &mut W) ->
        Result<(), Error>
    where &'a FilePosition<'a>: TryFrom<&'a P>,
          M: MessagePositions<P>,
          W: WriteColor,
          P: 'a + Display {
        msg.severity().write_color(out)?;

        if out.supports_color() {
            out.set_color(ColorSpec::new().set_bold(true))?;
            write!(out, ": {}", msg.brief())?;
            out.reset()?;
        } else {
            write!(out, ": {}", msg.brief())?;
        }

        let mut first = true;

        for (label, pos, _) in msg.positions() {
            match label {
                Some(label) if first => {
                    writeln!(out)?;
                    writeln!(out, "  {} {}", label, pos)?
                },
                Some(label) => writeln!(out, "  {} {}", label, pos)?,
                None => ()
            }
            first = false;
        }

        writeln!(out, "{}", msg.detail())
    }
}

impl MessageWriter for MessageMinimalWriter {
    fn write_msg<'a, M, P, W>(&self, msg: &'a M, out: &mut W) ->
        Result<(), Error>
    where &'a FilePosition<'a>: TryFrom<&'a P>,
          M: MessagePositions<P>,
          W: WriteColor,
          P: 'a + Display {
        write!(out, "{}: {}", msg.severity(), msg.brief())?;

        let mut first = true;

        for (label, pos, _) in msg.positions() {
            match label {
                Some(label) if first => {
                    writeln!(out)?;
                    writeln!(out, "  {} {}", label, pos)?
                },
                Some(label) => writeln!(out, "  {} {}", label, pos)?,
                None => ()
            }
            first = false;
        }

        Ok(())
    }
}
