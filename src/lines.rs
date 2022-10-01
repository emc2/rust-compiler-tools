use std::cmp::Ordering;

/// Representation of one line.
#[derive(Debug)]
struct Line<C> {
    /// Start of the line.
    start: usize,
    /// Content of the line.
    content: C
}

/// A structure holding lines in a source file.
#[derive(Debug)]
pub struct Lines<C> {
    /// Vector of lines, can be binary searched by position.
    lines: Vec<Line<C>>
}

impl<C> Lines<C> {
    /// Create a new `Lines`.
    #[inline]
    pub fn new() -> Self {
        Lines { lines: Vec::new() }
    }

    /// Create a new `Lines` with a specific initial capacity.
    #[inline]
    pub fn with_capacity(size: usize) -> Self {
        Lines { lines: Vec::with_capacity(size) }
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
    pub fn lookup(&self, pos: usize) -> (usize, usize, &C) {
        match self.lines.binary_search_by(|x| x.start.cmp(&pos)) {
            Ok(idx) => (idx, 0, &self.lines[idx].content),
            Err(idx) => {
                let start = self.lines[idx].start;

                (idx, pos - start, &self.lines[idx].content)
            }
        }
    }

    /// Add a line definition to the end of this `Lines`.
    #[inline]
    pub fn push_line(&mut self, start: usize, content: C) {
        self.lines.push(Line { start: start, content: content })
    }
}
