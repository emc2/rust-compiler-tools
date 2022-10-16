use crate::position::Positioned;
use std::cmp::Ordering;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;
use std::hash::Hash;
use std::hash::Hasher;

/// Wrapper for values that are excluded from normal notions of
/// distinctness in the context of another type.
///
/// `Nondistinct`'s [`Eq`], [`Ord`], [`PartialEq`], and [`PartialOrd`]
/// implementations consider all instances to be equivalent, and its
/// [`Hash`] implementation does not hash the inner value.  The effect
/// of this is that when a value is wrapped in `Nondistinct` in
/// another type, it will be effectively ignored by the corresponding
/// instances for that type.
///
/// This is primarily intended for excluding positions from such
/// instances in an AST or IR structure, which have no semantic
/// meaning and therefore should not be considered for the purposes of
/// equality comparison.  More generally, it can be used to attach
/// non-semantic metadata, without having to manually implement all of
/// the instances.
#[derive(Clone)]
pub struct Nondistinct<T> {
    pub val: T
}

impl<T> Debug for Nondistinct<T>
where T: Debug {
    #[inline]
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error> {
        self.val.fmt(formatter)
    }
}

impl<T> Default for Nondistinct<T>
where T: Default {
    #[inline]
    fn default() -> Self {
        Nondistinct { val: T::default() }
    }
}

impl<T> Display for Nondistinct<T>
where T: Display {
    #[inline]
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error> {
        self.val.fmt(formatter)
    }
}

impl<T> Eq for Nondistinct<T> {}

impl<T> Hash for Nondistinct<T> {
    #[inline]
    fn hash<H>(&self, _state: &mut H)
    where H: Hasher {}
}

impl<T> From<T> for Nondistinct<T> {
    #[inline]
    fn from(val: T) -> Self {
        Nondistinct { val: val }
    }
}

impl<T> Ord for Nondistinct<T> {
    #[inline]
    fn cmp(&self, _other: &Nondistinct<T>) -> Ordering {
        Ordering::Equal
    }
}

impl<T> PartialEq for Nondistinct<T> {
    #[inline]
    fn eq(&self, _other: &Nondistinct<T>) -> bool {
        true
    }
}

impl<T> PartialOrd for Nondistinct<T> {
    #[inline]
    fn partial_cmp(&self, other: &Nondistinct<T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<P> Positioned<P> for Nondistinct<P> {
    #[inline]
    fn position(&self) -> &P {
        &self.val
    }
}
