use super::Stripped;
use crate::Loc;

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
pub trait StrippedPartialEq<U: ?Sized = Self> {
	fn stripped_eq(&self, other: &U) -> bool;
}

impl<'a, 'b, T: StrippedPartialEq<U>, U> PartialEq<Stripped<'b, U>> for Stripped<'a, T> {
	fn eq(&self, other: &Stripped<'b, U>) -> bool {
		self.0.stripped_eq(other.0)
	}
}

impl<U, G, P, T: StrippedPartialEq<U>, F, S> StrippedPartialEq<Loc<U, G, P>> for Loc<T, F, S> {
	fn stripped_eq(&self, other: &Loc<U, G, P>) -> bool {
		self.value().stripped_eq(other.value())
	}
}

impl<T: StrippedPartialEq<U>, U> StrippedPartialEq<Box<U>> for Box<T> {
	fn stripped_eq(&self, other: &Box<U>) -> bool {
		(**self).stripped_eq(&**other)
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
