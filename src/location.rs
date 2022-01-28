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

	/// Consumes this location and returns the file.
	pub fn into_file(self) -> F {
		self.file
	}

	/// Consumes this location and returns the span.
	pub fn into_span(self) -> Span {
		self.span
	}

	/// Returns a reference to the file's identifier.
	pub fn file(&self) -> &F {
		&self.file
	}

	/// Returns a mutable reference to the file's identifier.
	pub fn file_mut(&mut self) -> &mut F {
		&mut self.file
	}

	/// Sets the file and returns the previous one.
	pub fn set_file(&mut self, mut file: F) -> F {
		std::mem::swap(&mut self.file, &mut file);
		file
	}

	/// Returns the `Span` in the file.
	pub fn span(&self) -> Span {
		self.span
	}

	/// Returns a mutable reference to the span.
	pub fn span_mut(&mut self) -> &mut Span {
		&mut self.span
	}

	/// Sets the span and returns the previous one.
	pub fn set_span(&mut self, mut span: Span) -> Span {
		std::mem::swap(&mut self.span, &mut span);
		span
	}

	/// Maps the file identifier.
	pub fn map_file<G>(self, f: impl FnOnce(F) -> G) -> Location<G> {
		Location {
			file: f(self.file),
			span: self.span,
		}
	}
}
