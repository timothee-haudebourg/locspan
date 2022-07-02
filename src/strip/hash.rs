use super::Stripped;
use crate::Loc;
use std::hash::{Hash, Hasher};

/// Defines the partial ordering of located values
/// without considering locations.
pub trait StrippedHash {
	fn stripped_hash<H: Hasher>(&self, state: &mut H);
}

impl<'a, T: StrippedHash> Hash for Stripped<'a, T> {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.0.stripped_hash(state)
	}
}

impl<T: StrippedHash, F, S> StrippedHash for Loc<T, F, S> {
	fn stripped_hash<H: Hasher>(&self, state: &mut H) {
		self.value().stripped_hash(state)
	}
}

impl<T: StrippedHash> StrippedHash for Box<T> {
	fn stripped_hash<H: Hasher>(&self, state: &mut H) {
		(**self).stripped_hash(state)
	}
}

impl<T: StrippedHash> StrippedHash for Option<T> {
	fn stripped_hash<H: Hasher>(&self, state: &mut H) {
		match self {
			Self::None => 0x00.hash(state),
			Self::Some(t) => {
				0xff.hash(state);
				t.stripped_hash(state)
			}
		}
	}
}

impl<T: StrippedHash> StrippedHash for Vec<T> {
	fn stripped_hash<H: Hasher>(&self, state: &mut H) {
		0xff.hash(state);
		for value in self {
			value.stripped_hash(state)
		}
	}
}
