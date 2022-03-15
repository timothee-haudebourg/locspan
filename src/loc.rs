use crate::{Location, Span};
use std::borrow::{Borrow, BorrowMut};
use std::ops::{Deref, DerefMut};

/// Located data.
///
/// This is a simple wrapper around data that can be located in a source file.
/// It is useful to wrap abstract syntax tree nodes.
///
/// It is a tuple struct so it can be easily deconstructed using pattern matching.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct Loc<T, F>(pub T, pub Location<F>);

impl<T, F> Loc<T, F> {
	/// Creates a new located value.
	#[inline(always)]
	pub fn new(t: T, location: Location<F>) -> Self {
		Self(t, location)
	}

	/// Unwraps the value and discard its location.
	#[inline(always)]
	pub fn into_value(self) -> T {
		self.0
	}

	/// Discards the value and returns its location.
	#[inline(always)]
	pub fn into_location(self) -> Location<F> {
		self.1
	}

	/// Discards the value and returns its file.
	#[inline(always)]
	pub fn into_file(self) -> F {
		self.1.into_file()
	}

	/// Discards the value and returns its span.
	#[inline(always)]
	pub fn into_span(self) -> Span {
		self.1.into_span()
	}

	/// Returns a reference to the wrapped value.
	#[inline(always)]
	pub fn value(&self) -> &T {
		&self.0
	}

	/// Returns a mutable reference to the wrapped value.
	#[inline(always)]
	pub fn value_mut(&mut self) -> &mut T {
		&mut self.0
	}

	/// Returns a reference to the value's location.
	#[inline(always)]
	pub fn location(&self) -> &Location<F> {
		&self.1
	}

	/// Returns a mutable reference to the value's location.
	#[inline(always)]
	pub fn location_mut(&mut self) -> &mut Location<F> {
		&mut self.1
	}

	/// Returns the value's span.
	#[inline(always)]
	pub fn span(&self) -> Span {
		self.1.span()
	}

	/// Returns a mutable reference the value's span.
	#[inline(always)]
	pub fn span_mut(&mut self) -> &mut Span {
		self.1.span_mut()
	}

	/// Sets the value's span and returns the previous one.
	#[inline(always)]
	pub fn set_span(&mut self, span: Span) -> Span {
		self.1.set_span(span)
	}

	/// Returns a reference to the value's source file.
	#[inline(always)]
	pub fn file(&self) -> &F {
		self.1.file()
	}

	/// Returns a mutable reference to the value's source file.
	#[inline(always)]
	pub fn file_mut(&mut self) -> &mut F {
		self.1.file_mut()
	}

	/// Sets the value's file and returns the previous one.
	#[inline(always)]
	pub fn set_file(&mut self, file: F) -> F {
		self.1.set_file(file)
	}

	/// Maps the inner value.
	#[inline(always)]
	pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Loc<U, F> {
		Loc(f(self.0), self.1)
	}

	/// Converts the inner value.
	#[inline(always)]
	pub fn cast<U>(self) -> Loc<U, F>
	where
		U: From<T>,
	{
		Loc(self.0.into(), self.1)
	}

	/// Tries to map the inner value.
	#[inline(always)]
	pub fn try_map<U, E>(self, f: impl FnOnce(T) -> Result<U, E>) -> Result<Loc<U, F>, E> {
		Ok(Loc(f(self.0)?, self.1))
	}

	/// Tries to convert the inner value.
	#[inline(always)]
	pub fn try_cast<U>(self) -> Result<Loc<U, F>, U::Error>
	where
		U: TryFrom<T>,
	{
		Ok(Loc(self.0.try_into()?, self.1))
	}

	/// Maps the value's location.
	#[inline(always)]
	pub fn map_location<G>(self, f: impl FnOnce(Location<F>) -> Location<G>) -> Loc<T, G> {
		Loc(self.0, f(self.1))
	}

	/// Maps the value's location's file.
	#[inline(always)]
	pub fn map_file<G>(self, f: impl FnOnce(F) -> G) -> Loc<T, G> {
		Loc(self.0, self.1.map_file(f))
	}

	/// Borrows the value and file.
	#[inline(always)]
	pub fn borrow(&self) -> Loc<&T, &F> {
		Loc(&self.0, self.1.borrow())
	}

	/// Borrows the value and clones the file.
	#[inline(always)]
	pub fn borrow_value(&self) -> Loc<&T, F>
	where
		F: Clone,
	{
		Loc(&self.0, self.1.clone())
	}

	/// Borrows the file and clones the value.
	#[inline(always)]
	pub fn borrow_file(&self) -> Loc<T, &F>
	where
		T: Clone,
	{
		Loc(self.0.clone(), self.1.borrow())
	}
}

impl<'t, T: Clone, F> Loc<&'t T, F> {
	/// Clones the borrowed value and the file to return a new `Loc<T, F>`.
	#[inline(always)]
	pub fn cloned_value(&self) -> Loc<T, F>
	where
		F: Clone,
	{
		Loc(self.0.clone(), self.1.clone())
	}

	/// Clones the borrowed value and consume the file to return a new `Loc<T, F>`.
	#[inline(always)]
	pub fn into_cloned_value(self) -> Loc<T, F> {
		Loc(self.0.clone(), self.1)
	}
}

impl<'f, T, F: Clone> Loc<T, &'f F> {
	/// Clones the value and the borrowed file to return a new `Loc<T, F>`.
	#[inline(always)]
	pub fn cloned_file(&self) -> Loc<T, F>
	where
		T: Clone,
	{
		Loc(self.0.clone(), self.1.cloned())
	}

	/// Clones the borrowed file and consumes the value to return a new `Loc<T, F>`.
	#[inline(always)]
	pub fn into_cloned_file(self) -> Loc<T, F> {
		Loc(self.0, self.1.cloned())
	}
}

impl<'t, 'f, T: Clone, F: Clone> Loc<&'t T, &'f F> {
	/// Clones the borrowed value and file to return a new `Loc<T, F>`.
	pub fn cloned(&self) -> Loc<T, F> {
		Loc(self.0.clone(), self.1.cloned())
	}
}

impl<T, F> Loc<Option<T>, F> {
	/// Unwraps the inner `Option`.
	#[inline(always)]
	pub fn unwrap(self) -> Loc<T, F> {
		self.map(Option::unwrap)
	}

	#[inline(always)]
	pub fn transpose(self) -> Option<Loc<T, F>> {
		match self.0 {
			Some(t) => Some(Loc(t, self.1)),
			None => None,
		}
	}
}

impl<T, F> Deref for Loc<T, F> {
	type Target = T;

	#[inline(always)]
	fn deref(&self) -> &T {
		self.value()
	}
}

impl<T, F> DerefMut for Loc<T, F> {
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut T {
		self.value_mut()
	}
}

impl<T, F> AsRef<T> for Loc<T, F> {
	#[inline(always)]
	fn as_ref(&self) -> &T {
		self.value()
	}
}

impl<T, F> AsMut<T> for Loc<T, F> {
	#[inline(always)]
	fn as_mut(&mut self) -> &mut T {
		self.value_mut()
	}
}

impl<T, F> Borrow<T> for Loc<T, F> {
	#[inline(always)]
	fn borrow(&self) -> &T {
		self.value()
	}
}

impl<T, F> BorrowMut<T> for Loc<T, F> {
	#[inline(always)]
	fn borrow_mut(&mut self) -> &mut T {
		self.value_mut()
	}
}

/// Provides the `at` function to locate any value.
///
/// This trait is implemented for all types.
pub trait At: Sized {
	/// Wraps `self` inside a `Loc<Self, F>` using the given `location`.
	///
	/// Equivalent to `Loc(self, location)`.
	fn at<F>(self, location: Location<F>) -> Loc<Self, F>;
}

impl<T> At for T {
	fn at<F>(self, location: Location<F>) -> Loc<Self, F> {
		Loc(self, location)
	}
}

/// Provides a transposition function from `Option<Loc<T, F>>` to `Loc<Option<T>, F>`.
pub trait TransposeLoc {
	/// Located value type.
	type Value;

	/// File id type.
	type FileId;

	/// Transposes a `Option<Loc<Self::Value, Self::FileId>>` into a `Loc<Option<Self::Value>, Self::FileId>`.
	fn transpose_loc(
		self,
		none_location: impl FnOnce() -> Location<Self::FileId>,
	) -> Loc<Option<Self::Value>, Self::FileId>;
}

impl<T, F> TransposeLoc for Option<Loc<T, F>> {
	type Value = T;
	type FileId = F;

	#[inline(always)]
	fn transpose_loc(self, none_location: impl FnOnce() -> Location<F>) -> Loc<Option<T>, F> {
		match self {
			Some(Loc(t, loc)) => Loc(Some(t), loc),
			None => Loc(None, none_location()),
		}
	}
}

/// Locates the error of a `Result<T, E>`.
pub trait ErrAt {
	/// Success type.
	type Value;

	/// Error type.
	type Error;

	/// Changes a `Result<Self::Value, Self::Error>` into a `Result<Self::Value, Loc<Self::Error, F>>` by wrapping
	/// any eventual error using the result of the `location` function.
	fn err_at<F>(
		self,
		location: impl FnOnce() -> Location<F>,
	) -> Result<Self::Value, Loc<Self::Error, F>>;
}

impl<T, E> ErrAt for Result<T, E> {
	type Value = T;
	type Error = E;

	#[inline(always)]
	fn err_at<F>(
		self,
		location: impl FnOnce() -> Location<F>,
	) -> Result<Self::Value, Loc<Self::Error, F>> {
		match self {
			Ok(t) => Ok(t),
			Err(e) => Err(Loc(e, location())),
		}
	}
}

/// Maps the located error of a `Result<T, Loc<E, F>>`.
pub trait MapLocErr {
	/// Success type.
	type Value;

	/// Error type.
	type Error;

	/// File id type.
	type FileId;

	/// Changes a `Result<Self::Value, Loc<Self::Error, Self::FileId>>` into a `Result<Self::Value, Loc<G, Self::FileId>>`
	/// by mapping the error value using `f`.
	fn map_loc_err<G>(
		self,
		f: impl FnOnce(Self::Error) -> G,
	) -> Result<Self::Value, Loc<G, Self::FileId>>;
}

impl<T, E, F> MapLocErr for Result<T, Loc<E, F>> {
	type Value = T;
	type Error = E;
	type FileId = F;

	#[inline(always)]
	fn map_loc_err<G>(
		self,
		f: impl FnOnce(Self::Error) -> G,
	) -> Result<Self::Value, Loc<G, Self::FileId>> {
		match self {
			Ok(t) => Ok(t),
			Err(Loc(e, loc)) => Err(Loc(f(e), loc)),
		}
	}
}
