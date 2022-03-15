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
	#[inline(always)]
	pub fn new(file: F, span: impl Into<Span>) -> Self {
		Self {
			file,
			span: span.into(),
		}
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

	/// Returns a reference to the file's identifier.
	#[inline(always)]
	pub fn file(&self) -> &F {
		&self.file
	}

	/// Returns a mutable reference to the file's identifier.
	#[inline(always)]
	pub fn file_mut(&mut self) -> &mut F {
		&mut self.file
	}

	/// Sets the file and returns the previous one.
	#[inline(always)]
	pub fn set_file(&mut self, mut file: F) -> F {
		std::mem::swap(&mut self.file, &mut file);
		file
	}

	/// Returns the `Span` in the file.
	#[inline(always)]
	pub fn span(&self) -> Span {
		self.span
	}

	/// Returns a mutable reference to the span.
	#[inline(always)]
	pub fn span_mut(&mut self) -> &mut Span {
		&mut self.span
	}

	/// Sets the span and returns the previous one.
	#[inline(always)]
	pub fn set_span(&mut self, mut span: Span) -> Span {
		std::mem::swap(&mut self.span, &mut span);
		span
	}

	/// Maps the file identifier.
	#[inline(always)]
	pub fn map_file<G>(self, f: impl FnOnce(F) -> G) -> Location<G> {
		Location {
			file: f(self.file),
			span: self.span,
		}
	}

	/// Sets the end of the location span to `end`, and returns itself.
	#[inline(always)]
	pub fn until(mut self, end: usize) -> Self {
		self.span.set_end(end);
		self
	}

	/// Append `span` to the location span, and returns itself.
	#[inline(always)]
	pub fn with(mut self, span: Span) -> Self {
		self.span.append(span);
		self
	}

	/// Copies the span and borrows the file to create a new `Location<&F>`.
	#[inline(always)]
	pub fn borrow(&self) -> Location<&F> {
		Location::new(&self.file, self.span)
	}
}

impl<'a, F: Clone> Location<&'a F> {
	/// Clones the borrowed file to return a new `Location<F>`.
	#[inline(always)]
	pub fn cloned(&self) -> Location<F> {
		Location::new(self.file.clone(), self.span)
	}
}
