use super::{Stripped, StrippedPartialEq};
use crate::Loc;
use std::cmp::{Ordering, PartialOrd};

/// Defines the partial ordering of located values
/// without considering locations.
pub trait StrippedPartialOrd<U: ?Sized = Self>: StrippedPartialEq<U> {
	fn stripped_partial_cmp(&self, other: &U) -> Option<Ordering>;
}

impl<'a, 'b, T: StrippedPartialOrd<U>, U> PartialOrd<Stripped<'b, U>> for Stripped<'a, T> {
	fn partial_cmp(&self, other: &Stripped<'b, U>) -> Option<Ordering> {
		self.0.stripped_partial_cmp(other.0)
	}
}

impl<U, G, P, T: StrippedPartialOrd<U>, F, S> StrippedPartialOrd<Loc<U, G, P>> for Loc<T, F, S> {
	fn stripped_partial_cmp(&self, other: &Loc<U, G, P>) -> Option<Ordering> {
		self.value().stripped_partial_cmp(other.value())
	}
}

impl<T: StrippedPartialOrd<U>, U> StrippedPartialOrd<Box<U>> for Box<T> {
	fn stripped_partial_cmp(&self, other: &Box<U>) -> Option<Ordering> {
		(**self).stripped_partial_cmp(&**other)
	}
}

impl<T: StrippedPartialOrd<U>, U> StrippedPartialOrd<Option<U>> for Option<T> {
	fn stripped_partial_cmp(&self, other: &Option<U>) -> Option<Ordering> {
		match (self, other) {
			(None, None) => Some(Ordering::Equal),
			(None, Some(_)) => Some(Ordering::Less),
			(Some(_), None) => Some(Ordering::Greater),
			(Some(a), Some(b)) => a.stripped_partial_cmp(b),
		}
	}
}

impl<T: StrippedPartialOrd<U>, U> StrippedPartialOrd<Vec<U>> for Vec<T> {
	fn stripped_partial_cmp(&self, other: &Vec<U>) -> Option<Ordering> {
		let mut self_iter = self.iter();
		let mut other_iter = other.iter();

		loop {
			match (self_iter.next(), other_iter.next()) {
				(Some(a), Some(b)) => match a.stripped_partial_cmp(b) {
					Some(Ordering::Equal) => (),
					cmp => break cmp,
				},
				(None, Some(_)) => break Some(Ordering::Less),
				(Some(_), None) => break Some(Ordering::Greater),
				(None, None) => break Some(Ordering::Equal),
			}
		}
	}
}
