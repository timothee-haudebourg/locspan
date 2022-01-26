use crate::Span;

/// Syntax element location.
///
/// Provides a file identifier (of type `F`) and a [`Span`] in this file.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct Location<F> {
	/// File id.
	file: F,

	/// Span.
	span: Span,
}

impl<F> Location<F> {
	/// Creates a new location referring to the given `span` in the given `file`.
	pub fn new(file: F, span: impl Into<Span>) -> Self {
		Self {
			file,
			span: span.into(),
		}
	}

	/// Consumes this location and returns a pair
	/// containing the file and span.
	pub fn into_parts(self) -> (F, Span) {
		(self.file, self.span)
	}

	/// Returns a reference to the file's identifier.
	pub fn file(&self) -> &F {
		&self.file
	}

	/// Returns the `Span` in the file.
	pub fn span(&self) -> Span {
		self.span
	}

	/// Maps the file identifier.
	pub fn map_file<G>(self, f: impl FnOnce(F) -> G) -> Location<G> {
		Location {
			file: f(self.file),
			span: self.span,
		}
	}
}
