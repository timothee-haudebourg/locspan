use crate::Loc;
use std::{
	collections::{BTreeSet, HashSet},
	hash::Hash,
};

/// Type that can be stripped of its location information.
pub trait Strip {
	type Stripped;

	fn strip(self) -> Self::Stripped;
}

impl<T: Strip, F> Strip for Loc<T, F> {
	type Stripped = T::Stripped;

	fn strip(self) -> Self::Stripped {
		self.0.strip()
	}
}

impl<T: Strip> Strip for Option<T> {
	type Stripped = Option<T::Stripped>;

	fn strip(self) -> Self::Stripped {
		self.map(T::strip)
	}
}

impl<T: Strip> Strip for Vec<T> {
	type Stripped = Vec<T::Stripped>;

	fn strip(self) -> Self::Stripped {
		self.into_iter().map(T::strip).collect()
	}
}

impl<T: Strip> Strip for HashSet<T>
where
	T::Stripped: Hash + Eq,
{
	type Stripped = HashSet<T::Stripped>;

	fn strip(self) -> Self::Stripped {
		self.into_iter().map(T::strip).collect()
	}
}

impl<T: Strip> Strip for BTreeSet<T>
where
	T::Stripped: Ord,
{
	type Stripped = BTreeSet<T::Stripped>;

	fn strip(self) -> Self::Stripped {
		self.into_iter().map(T::strip).collect()
	}
}

pub trait BorrowStripped {
	fn stripped(&self) -> Stripped<Self>;
}

impl<T> BorrowStripped for T {
	fn stripped(&self) -> Stripped<Self> {
		Stripped(self)
	}
}

/// Defines the equality of located values
/// without considering locations.
///
/// ## Example
///
/// ```
/// use locspan::{Loc, Span, Location, StrippedPartialEq, BorrowStripped};
///
/// #[derive(PartialEq, Eq, Debug)]
/// struct MyValue(u32);
///
/// impl StrippedPartialEq for MyValue {
///   fn stripped_eq(&self, other: &Self) -> bool {
///     self == other
///   }
/// }
///
/// let a = Loc(MyValue(0), Location::new("a", Span::new(0, 1)));
/// let b = Loc(MyValue(0), Location::new("b", Span::new(2, 4)));
///
/// // `a` and `b` are not equals,
/// // because their associated `Location`s are different.
/// assert_ne!(a, b);
///
/// // However using `BorrowStripped::stripped` we can
/// // compare the inner values regardless of the locations.
/// assert_eq!(a.stripped(), b.stripped());
/// ```
pub trait StrippedPartialEq {
	fn stripped_eq(&self, other: &Self) -> bool;
}

impl<T: StrippedPartialEq, F> StrippedPartialEq for Loc<T, F> {
	fn stripped_eq(&self, other: &Self) -> bool {
		self.value().stripped_eq(other.value())
	}
}

impl<T: StrippedPartialEq> StrippedPartialEq for Option<T> {
	fn stripped_eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Some(a), Some(b)) => a.stripped_eq(b),
			(None, None) => true,
			_ => false,
		}
	}
}

impl<T: StrippedPartialEq> StrippedPartialEq for Vec<T> {
	fn stripped_eq(&self, other: &Self) -> bool {
		self.len() == other.len() && self.iter().zip(other).all(|(a, b)| a.stripped_eq(b))
	}
}

/// Borrowed located value ignoring location information.
#[derive(Debug)]
pub struct Stripped<'a, T: ?Sized>(&'a T);

impl<'a, 'b, T: StrippedPartialEq> PartialEq<Stripped<'b, T>> for Stripped<'a, T> {
	fn eq(&self, other: &Stripped<'b, T>) -> bool {
		self.0.stripped_eq(other.0)
	}
}
