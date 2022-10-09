use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;
use std::ops::Add;
use std::ops::AddAssign;

/// A structure holding lines in a source file.
#[derive(Debug)]
pub struct LineOffsets {
    /// Vector of offsets for the start of each line, can be binary
    /// searched by position.
    lines: Vec<usize>
}

#[derive(Clone, Copy, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Offset(usize);

impl Add for Offset {
    type Output = Offset;

    fn add(self, rhs: Offset) -> Offset {
        Offset(self.0 + rhs.0)
    }
}

impl Add<&'_ Offset> for Offset {
    type Output = Offset;

    fn add(self, rhs: &'_ Offset) -> Offset {
        Offset(self.0 + rhs.0)
    }
}

impl Add<usize> for Offset {
    type Output = Offset;

    fn add(self, rhs: usize) -> Offset {
        Offset(self.0 + rhs)
    }
}

impl AddAssign for Offset {
    fn add_assign(&mut self, rhs: Offset) {
        self.0 += rhs.0
    }
}

impl AddAssign<&'_ Offset> for Offset {
    fn add_assign(&mut self, rhs: &'_ Offset) {
        self.0 += rhs.0
    }
}

impl AddAssign<usize> for Offset {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs
    }
}

impl Debug for Offset {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.0)
    }
}

impl Display for Offset {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.0)
    }
}

impl From<usize> for Offset {
    #[inline]
    fn from(val: usize) -> Offset {
        Offset(val)
    }
}

impl From<Offset> for usize {
    #[inline]
    fn from(val: Offset) -> usize {
        val.0
    }
}

impl From<&'_ Offset> for usize {
    #[inline]
    fn from(val: &'_ Offset) -> usize {
        val.0
    }
}

impl LineOffsets {
    /// Create a new `LineOffsets`.
    #[inline]
    pub fn new() -> Self {
        LineOffsets { lines: Vec::new() }
    }

    /// Create a new `LineOffsets` with a specific initial capacity.
    #[inline]
    pub fn with_capacity(size: usize) -> Self {
        LineOffsets { lines: Vec::with_capacity(size) }
    }

    /// Shrink this structure to fit its current contents.
    #[inline]
    pub fn shrink_to_fit(&mut self) {
        self.lines.shrink_to_fit()
    }

    /// Lookup the line to which `pos` belongs.
    ///
    /// This will convert the absolute file offset `pos` into a line
    /// number and offset pair.
    #[inline]
    pub fn lookup(&self, pos: Offset) -> (usize, usize) {
        if self.lines.len() > 0 {
            match self.lines.binary_search(&pos.0) {
                Ok(idx) => (idx + 1, 0),
                Err(idx) if idx > 0 => {
                    let start = self.lines[idx - 1];

                    (idx, pos.0 - start)
                },
                Err(_) => (0, pos.0)
            }
        } else {
            (0, pos.0)
        }
    }

    /// Add a line definition to the end of this `LineOffsets`.
    #[inline]
    pub fn push_line(&mut self, start: usize) {
        self.lines.push(start)
    }
}
