use crate::Loc;
use std::{
	collections::{BTreeSet, HashSet},
	hash::Hash,
};

mod eq;
mod hash;
mod ord;
mod partial_eq;
mod partial_ord;

pub use eq::*;
pub use hash::*;
pub use ord::*;
pub use partial_eq::*;
pub use partial_ord::*;

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

impl<T: Strip> Strip for Box<T> {
	type Stripped = Box<T::Stripped>;

	fn strip(self) -> Self::Stripped {
		Box::new((*self).strip())
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

/// Borrowed located value ignoring location information.
#[derive(Debug)]
pub struct Stripped<'a, T: ?Sized>(&'a T);

macro_rules! primitive {
	($($id:ident),*) => {
		$(
			impl Strip for $id {
				type Stripped = Self;

				fn strip(self) -> Self::Stripped {
					self
				}
			}

			impl<U> StrippedPartialEq<U> for $id where $id: PartialEq<U> {
				fn stripped_eq(&self, other: &U) -> bool {
					self == other
				}
			}

			impl StrippedEq for $id {}

			impl<U> StrippedPartialOrd<U> for $id where $id: PartialOrd<U> {
				fn stripped_partial_cmp(&self, other: &U) -> Option<std::cmp::Ordering> {
					self.partial_cmp(other)
				}
			}

			impl StrippedOrd for $id {
				fn stripped_cmp(&self, other: &Self) -> std::cmp::Ordering {
					self.cmp(other)
				}
			}

			impl StrippedHash for $id {
				fn stripped_hash<H: std::hash::Hasher>(&self, state: &mut H) {
					self.hash(state)
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

macro_rules! float {
	($($id:ident),*) => {
		$(
			impl Strip for $id {
				type Stripped = Self;

				fn strip(self) -> Self::Stripped {
					self
				}
			}

			impl<U> StrippedPartialEq<U> for $id where $id: PartialEq<U> {
				fn stripped_eq(&self, other: &U) -> bool {
					self == other
				}
			}

			impl<U> StrippedPartialOrd<U> for $id where $id: PartialOrd<U> {
				fn stripped_partial_cmp(&self, other: &U) -> Option<std::cmp::Ordering> {
					self.partial_cmp(other)
				}
			}
		)*
	};
}

float! {
	f32,
	f64
}
