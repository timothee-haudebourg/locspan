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
pub trait StrippedPartialEq<U = Self> {
	fn stripped_eq(&self, other: &U) -> bool;
}

impl<U, G, P, T: StrippedPartialEq<U>, F, S> StrippedPartialEq<Loc<U, G, P>> for Loc<T, F, S> {
	fn stripped_eq(&self, other: &Loc<U, G, P>) -> bool {
		self.value().stripped_eq(other.value())
	}
}

impl<T: StrippedPartialEq<U>, U> StrippedPartialEq<Option<U>> for Option<T> {
	fn stripped_eq(&self, other: &Option<U>) -> bool {
		match (self, other) {
			(Some(a), Some(b)) => a.stripped_eq(b),
			(None, None) => true,
			_ => false,
		}
	}
}

impl<T: StrippedPartialEq<U>, U> StrippedPartialEq<Vec<U>> for Vec<T> {
	fn stripped_eq(&self, other: &Vec<U>) -> bool {
		self.len() == other.len() && self.iter().zip(other).all(|(a, b)| a.stripped_eq(b))
	}
}

/// Borrowed located value ignoring location information.
#[derive(Debug)]
pub struct Stripped<'a, T: ?Sized>(&'a T);

impl<'a, 'b, T: StrippedPartialEq<U>, U> PartialEq<Stripped<'b, U>> for Stripped<'a, T> {
	fn eq(&self, other: &Stripped<'b, U>) -> bool {
		self.0.stripped_eq(other.0)
	}
}

macro_rules! primitive {
	($($id:ident),*) => {
		$(
			impl Strip for $id {
				type Stripped = Self;

				fn strip(self) -> Self::Stripped {
					self
				}
			}

			impl StrippedPartialEq for $id {
				fn stripped_eq(&self, other: &Self) -> bool {
					self == other
				}
			}
		)*
	};
}

primitive! {
	bool,
	u8,
	u16,
	u32,
	u64,
	i8,
	i16,
	i32,
	i64,
	usize,
	isize,
	char,
	String
}
