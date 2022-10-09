use std::borrow::Cow;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::convert::AsRef;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;
use std::hash::Hash;
use std::hash::Hasher;
use std::marker::PhantomData;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Index;
use std::path::Path;
use std::slice::SliceIndex;
use std::str::Bytes;
use std::str::CharIndices;
use std::str::Chars;
use std::str::EncodeUtf16;
use std::str::EscapeDebug;
use std::str::EscapeDefault;
use std::str::EscapeUnicode;
use std::str::FromStr;

/// A distinguished type for symbols.
///
/// These are pointers to interned strings, and function similar to
/// `&'a str`s, except that they are much more efficient to hash and
/// compare (these use the address of the interned string).  This is a
/// common technique employed in compiler implementation, as string
/// hashing and comparison is so common.
#[derive(Copy, Eq, Ord)]
pub struct Symbol<'a>(&'a str);

pub struct Symbols<'a> {
    lifetime: PhantomData<&'a str>,
    // Interned [Strings]
    interned: HashMap<String, ()>
}

/// Designated [Symbol] for the empty string.
///
/// This is used as the [Default] instance.
pub const NULL_SYM: Symbol<'static> = Symbol(&"");

impl<'a> Symbol<'a> {
    /// Get the id number for this `Symbol`.
    ///
    /// Id numbers are assigned arbitrarily, and not guaranteed to
    /// form a contiguous or dense range.
    #[inline]
    pub fn id(&self) -> usize {
        self.0.as_ptr() as usize
    }

    /// See [`str::len`].
    #[inline]
    pub const fn len(&self) -> usize {
        self.0.len()
    }

    /// See [`str::is_empty`].
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// See [`str::is_char_boundary`].
    #[inline]
    pub fn is_char_boundary(&self, idx: usize) -> bool {
        self.0.is_char_boundary(idx)
    }

    /// See [`str::as_bytes`].
    #[inline]
    pub const fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    /// See [`str::as_ptr`].
    #[inline]
    pub const fn as_ptr(&self) -> *const u8 {
        self.0.as_ptr()
    }

    /// See [`str::as_chars`].
    #[inline]
    pub fn chars(&self) -> Chars<'_> {
        self.0.chars()
    }

    /// See [`str::char_indices`].
    #[inline]
    pub fn char_indices(&self) -> CharIndices<'_> {
        self.0.char_indices()
    }

    /// See [`str::bytes`].
    #[inline]
    pub fn bytes(&self) -> Bytes<'_> {
        self.0.bytes()
    }

    /// See [`str::encode_utf16`].
    #[inline]
    pub fn encode_utf16(&self) -> EncodeUtf16<'_> {
        self.0.encode_utf16()
    }

    /// See [`str::parse`].
    #[inline]
    pub fn parse<F>(&self) -> Result<F, F::Err>
    where F: FromStr {
        self.0.parse()
    }

    /// See [`str::is_ascii`].
    #[inline]
    pub fn is_ascii(&self) -> bool {
        self.0.is_ascii()
    }

    /// See [`str::eq_ignore_ascii_case`].
    #[inline]
    pub fn eq_ignore_ascii_case(&self, other: &Symbol<'_>) -> bool {
        self.0.eq_ignore_ascii_case(other.0)
    }

    /// See [`str::escape_debug`].
    #[inline]
    pub fn escape_debug(&self) -> EscapeDebug<'_> {
        self.0.escape_debug()
    }

    /// See [`str::escape_default`].
    #[inline]
    pub fn escape_default(&self) -> EscapeDefault<'_> {
        self.0.escape_default()
    }

    /// See [`str::escape_unicode`].
    #[inline]
    pub fn escape_unicode(&self) -> EscapeUnicode<'_> {
        self.0.escape_unicode()
    }
}

impl<'a> Add<Symbol<'a>> for Cow<'a, str> {
    type Output = Cow<'a, str>;

    #[inline]
    fn add(self, rhs: Symbol<'a>) -> Self::Output {
        self.add(rhs.0)
    }
}

impl Add<Symbol<'_>> for String {
    type Output = String;

    #[inline]
    fn add(self, rhs: Symbol<'_>) -> Self::Output {
        self.add(rhs.0)
    }
}

impl<'a> AddAssign<Symbol<'a>> for Cow<'a, str> {
    #[inline]
    fn add_assign(&mut self, rhs: Symbol<'a>) {
        self.add_assign(rhs.0)
    }
}

impl AddAssign<Symbol<'_>> for String {
    #[inline]
    fn add_assign(&mut self, rhs: Symbol<'_>) {
        self.add_assign(rhs.0)
    }
}

impl AsRef<str> for Symbol<'_> {
    #[inline]
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl AsRef<[u8]> for Symbol<'_> {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl AsRef<OsStr> for Symbol<'_> {
    #[inline]
    fn as_ref(&self) -> &OsStr {
        OsStr::new(self.0)
    }
}

impl AsRef<Path> for Symbol<'_> {
    #[inline]
    fn as_ref(&self) -> &Path {
        Path::new(self.0)
    }
}

impl<'a> Clone for Symbol<'a> {
    #[inline]
    fn clone(&self) -> Symbol<'a> {
        Symbol(self.0)
    }
}

impl Default for Symbol<'_> {
    #[inline]
    fn default() -> Self {
        NULL_SYM
    }
}

impl Debug for Symbol<'_> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "#{}({})", self.id(), self.0)
    }
}

impl Display for Symbol<'_> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.0)
    }
}

impl<'a> From<Symbol<'a>> for &'a str {
    #[inline]
    fn from(s: Symbol<'a>) -> &'a str {
        s.0
    }
}

impl From<Symbol<'_>> for String {
    #[inline]
    fn from(s: Symbol<'_>) -> String {
        String::from(s.0)
    }
}

impl Hash for Symbol<'_> {
    #[inline]
    fn hash<H>(&self, state: &mut H)
    where H: Hasher {
        state.write_usize(self.id());
    }
}

impl<I> Index<I> for Symbol<'_>
where I: SliceIndex<str> {
    type Output = I::Output;

    #[inline]
    fn index(&self, idx: I) -> &Self::Output {
        self.0.index(idx)
    }
}

impl<'a> PartialEq<Cow<'a, str>> for Symbol<'a> {
    #[inline]
    fn eq(&self, other: &Cow<'a, str>) -> bool {
        self.0.eq(other)
    }
}

impl PartialEq<OsStr> for Symbol<'_> {
    #[inline]
    fn eq(&self, other: &OsStr) -> bool {
        self.0.eq(other)
    }
}

impl PartialEq<OsString> for Symbol<'_> {
    #[inline]
    fn eq(&self, other: &OsString) -> bool {
        self.0.eq(other)
    }
}

impl PartialEq<String> for Symbol<'_> {
    #[inline]
    fn eq(&self, other: &String) -> bool {
        self.0.eq(other)
    }
}

impl PartialEq<str> for Symbol<'_> {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        self.0.eq(other)
    }
}

impl<'a> PartialEq<Symbol<'a>> for Cow<'a, str> {
    #[inline]
    fn eq(&self, other: &Symbol<'a>) -> bool {
        self.eq(other.0)
    }
}

impl PartialEq<Symbol<'_>> for OsStr {
    #[inline]
    fn eq(&self, other: &Symbol<'_>) -> bool {
        self.eq(other.0)
    }
}

impl PartialEq<Symbol<'_>> for OsString {
    #[inline]
    fn eq(&self, other: &Symbol<'_>) -> bool {
        self.eq(other.0)
    }
}

impl PartialEq<Symbol<'_>> for String {
    #[inline]
    fn eq(&self, other: &Symbol<'_>) -> bool {
        self.eq(other.0)
    }
}

impl PartialEq<Symbol<'_>> for str {
    #[inline]
    fn eq(&self, other: &Symbol<'_>) -> bool {
        self.eq(other.0)
    }
}

impl PartialEq for Symbol<'_> {
    #[inline]
    fn eq(&self, other: &Symbol<'_>) -> bool {
        self.id() == other.id()
    }
}

impl PartialOrd<Symbol<'_>> for OsStr {
    #[inline]
    fn partial_cmp(&self, other: &Symbol<'_>) -> Option<Ordering> {
        self.partial_cmp(other.0)
    }
}

impl PartialOrd<Symbol<'_>> for OsString {
    #[inline]
    fn partial_cmp(&self, other: &Symbol<'_>) -> Option<Ordering> {
        self.partial_cmp(other.0)
    }
}

impl PartialOrd for Symbol<'_> {
    #[inline]
    fn partial_cmp(&self, other: &Symbol<'_>) -> Option<Ordering> {
        Some(self.id().cmp(&other.id()))
    }
}

unsafe impl Send for Symbol<'_> {}

unsafe impl Sync for Symbol<'_> {}

impl<'a> Symbols<'a> {
    /// Create a new `Symbols`.
    #[inline]
    pub fn new() -> Symbols<'a> {
        Symbols { lifetime: PhantomData, interned: HashMap::new() }
    }

    /// Create a new `Symbols` with a size hint.
    #[inline]
    pub fn with_capacity(size: usize) -> Symbols<'a> {
        Symbols { interned: HashMap::with_capacity(size),
                  lifetime: PhantomData }
    }

    /// Shring down this `Symbols` to fit the current contents.
    #[inline]
    pub fn shrink_to_fit(&mut self) {
        self.interned.shrink_to_fit()
    }

    /// Internal function to create a symbol.
    fn create_symbol_nonnull(&mut self, str: String) -> Symbol<'a> {
        match self.interned.entry(str) {
            Entry::Occupied(ent) => {
                unsafe {
                    let ptr = ent.key().as_str() as *const str;

                    Symbol(&*ptr)
                }
            },
            Entry::Vacant(ent) => {
                unsafe {
                    let ptr = ent.key().as_str() as *const str;

                    ent.insert(());

                    Symbol(&*ptr)
                }
            }
        }
    }

    /// Create a `Symbol` from a non-empty string.
    ///
    /// The argument `s` must not be equal to `""`.
    #[inline]
    pub fn symbol_nonnull<S>(&mut self, s: &S) -> Symbol<'a>
    where S: ToString {
        let str = s.to_string();

        assert!(str != "");

        self.create_symbol_nonnull(str)
    }

    /// Create a `Symbol` from a string.
    #[inline]
    pub fn symbol<S>(&mut self, s: &S) -> Symbol<'a>
    where S: ToString {
        let str = s.to_string();

        if !str.is_empty() {
            self.create_symbol_nonnull(str)
        } else {
            NULL_SYM
        }
    }
}
