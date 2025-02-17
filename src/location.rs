use crate::{MaybeSpanned, Span, Spanned};

/// Syntax element location.
///
/// Provides a file identifier (of type `F`) and a [`Span`] in this file.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct Location<F> {
	/// File id.
	pub file: F,

	/// Span.
	pub span: Span,
}

impl<F> Location<F> {
	/// Creates a new location referring to the given `span` in the given `file`.
	#[inline(always)]
	pub fn new(file: F, span: Span) -> Self {
		Self { file, span }
	}

	/// Consumes this location and returns a pair
	/// containing the file and span.
	#[inline(always)]
	pub fn into_parts(self) -> (F, Span) {
		(self.file, self.span)
	}

	/// Consumes this location and returns the file.
	#[inline(always)]
	pub fn into_file(self) -> F {
		self.file
	}

	/// Consumes this location and returns the span.
	#[inline(always)]
	pub fn into_span(self) -> Span {
		self.span
	}

	/// Maps the file identifier.
	#[inline(always)]
	pub fn map<G>(self, f: impl FnOnce(F) -> G) -> Location<G> {
		Location {
			file: f(self.file),
			span: self.span,
		}
	}

	/// Copies the span and borrows the file to create a new `Location<&F>`.
	#[inline(always)]
	pub fn as_ref(&self) -> Location<&F> {
		Location::new(&self.file, self.span)
	}

	/// Converts the location.
	#[inline(always)]
	pub fn cast<G>(self) -> Location<G>
	where
		F: Into<G>,
	{
		Location::new(self.file.into(), self.span)
	}
}

impl<F> Location<F> {
	/// Sets the end of the location span to `end`, and returns itself.
	#[inline(always)]
	pub fn until(mut self, end: usize) -> Self {
		self.span.end = end;
		self
	}

	/// Append `span` to the location span, and returns itself.
	#[inline(always)]
	pub fn with(mut self, span: Span) -> Self {
		self.span.append(span);
		self
	}
}

impl<F: Clone> Location<&F> {
	/// Clones the borrowed file to return a new `Location<F>`.
	#[inline(always)]
	pub fn cloned(&self) -> Location<F> {
		Location::new(self.file.clone(), self.span)
	}
}

/// Value with a location.
pub trait Located {
	type File;

	fn location(&self) -> Location<&Self::File>;
}

impl<F> Located for Location<F> {
	type File = F;

	fn location(&self) -> Location<&Self::File> {
		self.as_ref()
	}
}

impl<T: Located> Spanned for T {
	fn span(&self) -> Span {
		self.location().span
	}
}

/// Value with an optional location.
pub trait MaybeLocated {
	type File;

	fn optional_location(&self) -> Option<Location<&Self::File>>;
}

impl<T: MaybeLocated> MaybeSpanned for T {
	fn optional_span(&self) -> Option<Span> {
		self.optional_location().map(Location::into_span)
	}
}

impl<T: Located> MaybeLocated for T {
	type File = T::File;

	fn optional_location(&self) -> Option<Location<&Self::File>> {
		Some(self.location())
	}
}
