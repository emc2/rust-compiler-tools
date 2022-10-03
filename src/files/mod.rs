use crate::lines::LineOffsets;
use crate::nondistinct::Nondistinct;
use std::borrow::Cow;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fs::Metadata;
use std::hash::Hash;
use std::hash::Hasher;
use std::io::Error;
use std::iter::IntoIterator;
use std::path::Ancestors;
use std::path::Components;
use std::path::Iter;
use std::path::Path;
use std::path::PathBuf;
use std::path::StripPrefixError;

/// A distinguished type for file paths.
///
/// These are pointers to interned strings, and function similar to
/// [`Path`]s, except that they are much more efficient to hash and
/// compare (these use the address of the interned string).  This is a
/// common technique employed in compiler implementation, as string
/// hashing and comparison is so common.
#[derive(Clone, Copy)]
pub struct Filename<'a>(&'a Path);

/// Interned filenames table, for producing [`Filename`]s.
pub struct Filenames {
    /// Interned [Path]s
    interned: HashMap<PathBuf, ()>
}

/// Line offsets for a given file.
///
/// Note that the line offsets are assumed to be completely determined
/// by the filename; thus, the offsets themselves are not used in
/// equality, comparison, and hashing.
#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct FileOffsets<'a> {
    /// The name of the file.
    filename: Filename<'a>,
    /// Lines structure for the file.
    line_offsets: Nondistinct<LineOffsets>,
}

impl<'a> Filename<'a> {
    /// Get the id number for this `Filename`.
    ///
    /// Id numbers are assigned arbitrarily, and not guaranteed to
    /// form a contiguous or dense range.
    #[inline]
    pub fn id(&self) -> usize {
        ((self.0 as *const _) as *const u8) as usize
    }

    /// See [`Path::as_os_str`].
    #[inline]
    pub fn as_os_str(&self) -> &OsStr {
        self.0.as_os_str()
    }

    /// See [`Path::to_str`].
    #[inline]
    pub fn to_str(&self) -> Option<&str> {
        self.0.to_str()
    }

    /// See [`Path::to_string_lossy`].
    #[inline]
    pub fn to_string_lossy(&self) -> Cow<'_, str> {
        self.0.to_string_lossy()
    }

    /// See [`Path::to_path_buf`].
    #[inline]
    pub fn to_path_buf(&self) -> PathBuf {
        self.0.to_path_buf()
    }

    /// See [`Path::is_absolute`].
    #[inline]
    pub fn is_absolute(&self) -> bool {
        self.0.is_absolute()
    }

    /// See [`Path::is_relative`].
    #[inline]
    pub fn is_relative(&self) -> bool {
        self.0.is_relative()
    }

    /// See [`Path::has_root`].
    #[inline]
    pub fn has_root(&self) -> bool {
        self.0.has_root()
    }

    /// See [`Path::parent`].
    #[inline]
    pub fn parent(&self) -> Option<&Path> {
        self.0.parent()
    }

    /// See [`Path::ancestors`].
    #[inline]
    pub fn ancestors(&self) -> Ancestors {
        self.0.ancestors()
    }

    /// See [`Path::file_name`].
    #[inline]
    pub fn file_name(&self) -> Option<&OsStr> {
        self.0.file_name()
    }

    /// See [`Path::strip_prefix`].
    #[inline]
    pub fn strip_prefix<P>(&self, base: P) -> Result<&Path, StripPrefixError>
    where P: AsRef<Path> {
        self.0.strip_prefix(base)
    }

    /// See [`Path::starts_with`].
    #[inline]
    pub fn starts_with<P>(&self, base: P) -> bool
    where P: AsRef<Path> {
        self.0.starts_with(base)
    }

    /// See [`Path::ends_with`].
    #[inline]
    pub fn ends_with<P>(&self, child: P) -> bool
    where P: AsRef<Path> {
        self.0.ends_with(child)
    }

    /// See [`Path::file_stem`].
    #[inline]
    pub fn file_stem(&self) -> Option<&OsStr> {
        self.0.file_stem()
    }

    /// See [`Path::extension`].
    #[inline]
    pub fn extension(&self) -> Option<&OsStr> {
        self.0.extension()
    }

    /// See [`Path::join`].
    #[inline]
    pub fn join<P>(&self, path: P) -> PathBuf
    where P: AsRef<Path> {
        self.0.join(path)
    }

    /// See [`Path::with_file_name`].
    #[inline]
    pub fn with_file_name<S>(&self, file: S) -> PathBuf
    where S: AsRef<OsStr> {
        self.0.with_file_name(file)
    }

    /// See [`Path::with_extension`].
    #[inline]
    pub fn with_extension<S>(&self, file: S) -> PathBuf
    where S: AsRef<OsStr> {
        self.0.with_extension(file)
    }

    /// See [`Path::components`].
    #[inline]
    pub fn components(&self) -> Components<'_> {
        self.0.components()
    }

    /// See [`Path::iter`].
    #[inline]
    pub fn iter(&self) -> Iter<'_> {
        self.0.iter()
    }

    /// See [`Path::display`].
    #[inline]
    pub fn display(&self) -> std::path::Display<'_> {
        self.0.display()
    }

    /// See [`Path::metadata`].
    #[inline]
    pub fn metadata(&self) -> Result<Metadata, Error> {
        self.0.metadata()
    }

    /// See [`Path::symlink_metadata`].
    #[inline]
    pub fn symlink_metadata(&self) -> Result<Metadata, Error> {
        self.0.symlink_metadata()
    }

    /// See [`Path::read_link`].
    #[inline]
    pub fn read_link(&self) -> Result<PathBuf, Error> {
        self.0.read_link()
    }

    /// See [`Path::exists`].
    #[inline]
    pub fn exists(&self) -> bool {
        self.0.exists()
    }

    /// See [`Path::is_file`].
    #[inline]
    pub fn is_file(&self) -> bool {
        self.0.is_file()
    }

    /// See [`Path::is_dir`].
    #[inline]
    pub fn is_dir(&self) -> bool {
        self.0.is_dir()
    }
}

impl AsRef<OsStr> for Filename<'_> {
    #[inline]
    fn as_ref(&self) -> &OsStr {
        self.0.as_ref()
    }
}

impl Debug for Filename<'_> {
    #[inline]
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(formatter)
    }
}

impl Display for Filename<'_> {
    #[inline]
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(formatter)
    }
}

impl Eq for Filename<'_> {}

impl<'a> From<Filename<'a>> for Cow<'a, Path> {
    #[inline]
    fn from(f: Filename<'a>) -> Cow<'a, Path> {
        Cow::from(f.0)
    }
}

impl Hash for Filename<'_> {
    #[inline]
    fn hash<H>(&self, state: &mut H)
    where H: Hasher {
        self.id().hash(state);
    }
}

impl<'a> IntoIterator for Filename<'a> {
    type Item = &'a OsStr;
    type IntoIter = Iter<'a>;

    #[inline]
    fn into_iter(self) -> Iter<'a> {
        self.0.into_iter()
    }
}

impl Ord for Filename<'_> {
    #[inline]
    fn cmp(&self, other: &Filename<'_>) -> Ordering {
        self.id().cmp(&other.id())
    }
}

impl<'a, 'b> PartialEq<&'a OsStr> for Filename<'b> {
    #[inline]
    fn eq(&self, other: &&'a OsStr) -> bool {
        self.0.eq(other)
    }
}

impl<'a, 'b> PartialEq<Filename<'b>> for &'a OsStr {
    #[inline]
    fn eq(&self, other: &Filename<'b>) -> bool {
        self.eq(other.0)
    }
}

impl<'a, 'b> PartialEq<Filename<'a>> for Cow<'b, Path> {
    #[inline]
    fn eq(&self, other: &Filename<'a>) -> bool {
        self.eq(other.0)
    }
}

impl<'a, 'b> PartialEq<Filename<'a>> for Cow<'b, OsStr> {
    #[inline]
    fn eq(&self, other: &Filename<'a>) -> bool {
        self.eq(other.0)
    }
}

impl PartialEq<Filename<'_>> for OsStr {
    #[inline]
    fn eq(&self, other: &Filename<'_>) -> bool {
        self.eq(other.0)
    }
}

impl PartialEq<Filename<'_>> for OsString {
    #[inline]
    fn eq(&self, other: &Filename<'_>) -> bool {
        self.eq(other.0)
    }
}

impl PartialEq<Filename<'_>> for Path {
    #[inline]
    fn eq(&self, other: &Filename<'_>) -> bool {
        self.eq(other.0)
    }
}

impl PartialEq<Filename<'_>> for PathBuf {
    #[inline]
    fn eq(&self, other: &Filename<'_>) -> bool {
        self.eq(other.0)
    }
}

impl<'a, 'b> PartialEq<Cow<'a, Path>> for Filename<'b> {
    #[inline]
    fn eq(&self, other: &Cow<'a, Path>) -> bool {
        self.0.eq(other)
    }
}

impl<'a, 'b> PartialEq<Cow<'a, OsStr>> for Filename<'b> {
    #[inline]
    fn eq(&self, other: &Cow<'a, OsStr>) -> bool {
        self.0.eq(other)
    }
}

impl PartialEq<OsStr> for Filename<'_> {
    #[inline]
    fn eq(&self, other: &OsStr) -> bool {
        self.0.eq(other)
    }
}

impl PartialEq<OsString> for Filename<'_> {
    #[inline]
    fn eq(&self, other: &OsString) -> bool {
        self.0.eq(other)
    }
}

impl PartialEq<Path> for Filename<'_> {
    #[inline]
    fn eq(&self, other: &Path) -> bool {
        self.0.eq(other)
    }
}

impl PartialEq<PathBuf> for Filename<'_> {
    #[inline]
    fn eq(&self, other: &PathBuf) -> bool {
        self.0.eq(other)
    }
}

impl PartialEq for Filename<'_> {
    #[inline]
    fn eq(&self, other: &Filename<'_>) -> bool {
        self.id().eq(&other.id())
    }
}

impl<'a, 'b> PartialOrd<&'a OsStr> for Filename<'b> {
    #[inline]
    fn partial_cmp(&self, other: &&'a OsStr) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

impl<'a, 'b> PartialOrd<Filename<'b>> for &'a OsStr {
    #[inline]
    fn partial_cmp(&self, other: &Filename<'b>) -> Option<Ordering> {
        self.partial_cmp(other.0)
    }
}

impl<'a, 'b> PartialOrd<Filename<'a>> for Cow<'b, Path> {
    #[inline]
    fn partial_cmp(&self, other: &Filename<'a>) -> Option<Ordering> {
        self.partial_cmp(other.0)
    }
}

impl<'a, 'b> PartialOrd<Filename<'a>> for Cow<'b, OsStr> {
    #[inline]
    fn partial_cmp(&self, other: &Filename<'a>) -> Option<Ordering> {
        self.partial_cmp(other.0)
    }
}

impl PartialOrd<Filename<'_>> for OsStr {
    #[inline]
    fn partial_cmp(&self, other: &Filename<'_>) -> Option<Ordering> {
        self.partial_cmp(other.0)
    }
}

impl PartialOrd<Filename<'_>> for OsString {
    #[inline]
    fn partial_cmp(&self, other: &Filename<'_>) -> Option<Ordering> {
        self.partial_cmp(other.0)
    }
}

impl PartialOrd<Filename<'_>> for Path {
    #[inline]
    fn partial_cmp(&self, other: &Filename<'_>) -> Option<Ordering> {
        self.partial_cmp(other.0)
    }
}

impl PartialOrd<Filename<'_>> for PathBuf {
    #[inline]
    fn partial_cmp(&self, other: &Filename<'_>) -> Option<Ordering> {
        self.partial_cmp(other.0)
    }
}

impl<'a, 'b> PartialOrd<Cow<'a, Path>> for Filename<'b> {
    #[inline]
    fn partial_cmp(&self, other: &Cow<'a, Path>) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

impl<'a, 'b> PartialOrd<Cow<'a, OsStr>> for Filename<'b> {
    #[inline]
    fn partial_cmp(&self, other: &Cow<'a, OsStr>) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

impl PartialOrd<OsStr> for Filename<'_> {
    #[inline]
    fn partial_cmp(&self, other: &OsStr) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

impl PartialOrd<OsString> for Filename<'_> {
    #[inline]
    fn partial_cmp(&self, other: &OsString) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

impl PartialOrd<Path> for Filename<'_> {
    #[inline]
    fn partial_cmp(&self, other: &Path) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

impl PartialOrd<PathBuf> for Filename<'_> {
    #[inline]
    fn partial_cmp(&self, other: &PathBuf) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

impl PartialOrd for Filename<'_> {
    #[inline]
    fn partial_cmp(&self, other: &Filename<'_>) -> Option<Ordering> {
        self.id().partial_cmp(&other.id())
    }
}

impl Filenames {
    /// Create a new `Filenames`.
    #[inline]
    pub fn new() -> Filenames {
        Filenames { interned: HashMap::new() }
    }

    /// Create a new `Filenames` with a size hint.
    #[inline]
    pub fn with_capacity(size: usize) -> Filenames {
        Filenames { interned: HashMap::with_capacity(size) }
    }

    /// Shring down this `Filenames` to fit the current contents.
    #[inline]
    pub fn shrink_to_fit(&mut self) {
        self.interned.shrink_to_fit()
    }

    /// Create a `Filename`.
    pub fn filename(&mut self, path: &Path) ->  Result<Filename<'_>, Error> {
        let path = path.canonicalize()?;

        match self.interned.entry(path) {
            Entry::Occupied(ent) => {
                unsafe {
                    let ptr = ent.key().as_ref() as *const Path;

                    Ok(Filename(&*ptr))
                }
            },
            Entry::Vacant(ent) => {
                unsafe {
                    let ptr = ent.key().as_ref() as *const Path;

                    ent.insert(());

                    Ok(Filename(&*ptr))
                }
            }
        }
    }
}

impl<'a> FileOffsets<'a> {
    /// Create a `FileOffsets` from its components.
    #[inline]
    pub fn new(filename: Filename<'a>, line_offsets: LineOffsets) -> Self {
        FileOffsets { line_offsets: Nondistinct::from(line_offsets),
                      filename: filename }
    }

    /// Get the [`Filename`] for this `FileOffsets`.
    #[inline]
    pub fn filename(&self) -> Filename<'a> {
        self.filename
    }

    /// Get the [`LineOffsets`] for this `FileOffsets`.
    #[inline]
    pub fn line_offsets(&self) -> &LineOffsets {
        &self.line_offsets.val
    }
}
