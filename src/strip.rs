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
