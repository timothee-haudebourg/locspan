use std::borrow::{Borrow, BorrowMut};
use std::fmt;
use std::ops::{Deref, DerefMut};

/// Data and its metadata.
///
/// This is a simple wrapper around data that also embeds data of type `M`.
///
/// It is a tuple struct so it can be easily deconstructed using pattern matching.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct Meta<T, M>(pub T, pub M);

impl<T, M> Meta<T, M> {
	/// Creates a new value attached to its metadata.
	#[inline(always)]
	pub fn new(t: T, metadata: M) -> Self {
		Self(t, metadata)
	}

	/// Unwraps the value and discard its metadata.
	#[inline(always)]
	pub fn into_value(self) -> T {
		self.0
	}

	/// Discards the value and returns its metadata.
	#[inline(always)]
	pub fn into_metadata(self) -> M {
		self.1
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

	/// Returns a reference to the value's metadata.
	#[inline(always)]
	pub fn metadata(&self) -> &M {
		&self.1
	}

	/// Returns a mutable reference to the value's metadata.
	#[inline(always)]
	pub fn metadata_mut(&mut self) -> &mut M {
		&mut self.1
	}

	/// Maps the inner value.
	#[inline(always)]
	pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Meta<U, M> {
		Meta(f(self.0), self.1)
	}

	/// Converts the inner value.
	#[inline(always)]
	pub fn cast<U>(self) -> Meta<U, M>
	where
		U: From<T>,
	{
		Meta(self.0.into(), self.1)
	}

	/// Tries to map the inner value.
	#[inline(always)]
	pub fn try_map<U, E>(self, f: impl FnOnce(T) -> Result<U, E>) -> Result<Meta<U, M>, E> {
		Ok(Meta(f(self.0)?, self.1))
	}

	/// Tries to convert the inner value.
	#[inline(always)]
	pub fn try_cast<U>(self) -> Result<Meta<U, M>, U::Error>
	where
		U: TryFrom<T>,
	{
		Ok(Meta(self.0.try_into()?, self.1))
	}

	/// Maps the metadata.
	#[inline(always)]
	pub fn map_metadata<N>(self, f: impl FnOnce(M) -> N) -> Meta<T, N> {
		Meta(self.0, f(self.1))
	}

	/// Maps the metadata of a recursive data structure.
	#[inline(always)]
	pub fn map_metadata_recursively<N, F: FnMut(M) -> N>(self, mut f: F) -> Meta<T::Output, N>
	where
		T: MapMetadataRecursively<M, N>,
	{
		let meta = f(self.1);
		Meta(self.0.map_metadata_recursively(f), meta)
	}

	/// Tries to maps the metadata.
	#[inline(always)]
	pub fn try_map_metadata<N, E>(
		self,
		f: impl FnOnce(M) -> Result<N, E>,
	) -> Result<Meta<T, N>, E> {
		Ok(Meta(self.0, f(self.1)?))
	}

	/// Tries to map the metadata of a recursive data structure.
	#[inline(always)]
	pub fn try_map_metadata_recursively<N, E, F: FnMut(M) -> Result<N, E>>(
		self,
		mut f: F,
	) -> Result<Meta<T::Output, N>, E>
	where
		T: TryMapMetadataRecursively<M, N, E>,
	{
		let meta = f(self.1)?;
		Ok(Meta(self.0.try_map_metadata_recursively(f)?, meta))
	}

	/// Cast the metadata.
	#[inline(always)]
	pub fn cast_metadata<N>(self) -> Meta<T, N>
	where
		M: Into<N>,
	{
		Meta(self.0, self.1.into())
	}

	/// Casts the metadata of a recursive data structure.
	#[inline(always)]
	pub fn cast_metadata_recursively<N>(self) -> Meta<T::Output, N>
	where
		T: MapMetadataRecursively<M, N>,
		M: Into<N>,
	{
		self.map_metadata_recursively(M::into)
	}

	/// Tries to cast the metadata.
	#[inline(always)]
	pub fn try_cast_metadata<N>(self) -> Result<Meta<T, N>, M::Error>
	where
		M: TryInto<N>,
	{
		Ok(Meta(self.0, self.1.try_into()?))
	}

	/// Tries to cast the metadata of a recursive data structure.
	#[inline(always)]
	pub fn try_cast_metadata_recursively<N>(self) -> Result<Meta<T::Output, N>, M::Error>
	where
		T: TryMapMetadataRecursively<M, N, M::Error>,
		M: TryInto<N>,
	{
		self.try_map_metadata_recursively(M::try_into)
	}

	/// Borrows the value and its metadata.
	#[inline(always)]
	pub fn borrow(&self) -> Meta<&T, &M> {
		Meta(&self.0, &self.1)
	}

	/// Borrows the value and clones the metadata.
	#[inline(always)]
	pub fn borrow_value(&self) -> Meta<&T, M>
	where
		M: Clone,
	{
		Meta(&self.0, self.1.clone())
	}

	/// Borrows the file and clones the value.
	#[inline(always)]
	pub fn borrow_metadata(&self) -> Meta<T, &M>
	where
		T: Clone,
	{
		Meta(self.0.clone(), &self.1)
	}
}

impl<T, M> From<T> for Meta<T, M>
where
	M: Default,
{
	fn from(t: T) -> Self {
		Self(t, M::default())
	}
}

impl<'t, T: Clone, M> Meta<&'t T, M> {
	/// Clones the borrowed value and the file to return a new `Meta<T, F>`.
	#[inline(always)]
	pub fn cloned_value(&self) -> Meta<T, M>
	where
		M: Clone,
	{
		Meta(self.0.clone(), self.1.clone())
	}

	/// Clones the borrowed value and consume the file to return a new `Loc<T, F>`.
	#[inline(always)]
	pub fn into_cloned_value(self) -> Meta<T, M> {
		Meta(self.0.clone(), self.1)
	}
}

impl<T, M> Meta<Option<T>, M> {
	/// Unwraps the inner `Option`.
	#[inline(always)]
	pub fn unwrap(self) -> Meta<T, M> {
		self.map(Option::unwrap)
	}

	#[inline(always)]
	pub fn transpose(self) -> Option<Meta<T, M>> {
		match self.0 {
			Some(t) => Some(Meta(t, self.1)),
			None => None,
		}
	}
}

impl<T, M> Deref for Meta<T, M> {
	type Target = T;

	#[inline(always)]
	fn deref(&self) -> &T {
		self.value()
	}
}

impl<T, M> DerefMut for Meta<T, M> {
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut T {
		self.value_mut()
	}
}

impl<T, M> AsRef<T> for Meta<T, M> {
	#[inline(always)]
	fn as_ref(&self) -> &T {
		self.value()
	}
}

impl<T, M> AsMut<T> for Meta<T, M> {
	#[inline(always)]
	fn as_mut(&mut self) -> &mut T {
		self.value_mut()
	}
}

impl<T, M> Borrow<T> for Meta<T, M> {
	#[inline(always)]
	fn borrow(&self) -> &T {
		self.value()
	}
}

impl<T, M> BorrowMut<T> for Meta<T, M> {
	#[inline(always)]
	fn borrow_mut(&mut self) -> &mut T {
		self.value_mut()
	}
}

impl<T: fmt::Display, M> fmt::Display for Meta<T, M> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.0.fmt(f)
	}
}

/// Provides the `at` function to locate any value.
///
/// This trait is implemented for all types.
pub trait At: Sized {
	/// Wraps `self` inside a `Meta<Self, M>` using the given `metadata`.
	///
	/// Equivalent to `Meta(self, metadata)`.
	/// Usually called with a [`Location`](crate::Location) to locate a value in a source file.
	fn at<M>(self, metadata: M) -> Meta<Self, M>;
}

impl<T> At for T {
	#[inline(always)]
	fn at<M>(self, metadata: M) -> Meta<Self, M> {
		Meta(self, metadata)
	}
}

/// Provides a function to map the metadata inside a recursive data structure.
pub trait MapMetadataRecursively<M, N> {
	type Output;

	/// Maps the metadata, recursively.
	fn map_metadata_recursively<F: FnMut(M) -> N>(self, f: F) -> Self::Output;
}

impl<T, M, N> MapMetadataRecursively<M, N> for Meta<T, M>
where
	T: MapMetadataRecursively<M, N>,
{
	type Output = Meta<T::Output, N>;

	#[inline(always)]
	fn map_metadata_recursively<F: FnMut(M) -> N>(self, f: F) -> Self::Output {
		self.map_metadata_recursively(f)
	}
}

/// Provides a function that tries to map the metadata inside a recursive data structure.
pub trait TryMapMetadataRecursively<M, N, E> {
	type Output;

	/// Tries to map the metadata, recursively.
	fn try_map_metadata_recursively<F: FnMut(M) -> Result<N, E>>(
		self,
		f: F,
	) -> Result<Self::Output, E>;
}

impl<T, M, N, E> TryMapMetadataRecursively<M, N, E> for Meta<T, M>
where
	T: TryMapMetadataRecursively<M, N, E>,
{
	type Output = Meta<T::Output, N>;

	#[inline(always)]
	fn try_map_metadata_recursively<F: FnMut(M) -> Result<N, E>>(
		self,
		f: F,
	) -> Result<Self::Output, E> {
		self.try_map_metadata_recursively(f)
	}
}

/// Provides a transposition function from `Option<Meta<T, M>>` to `Meta<Option<T>, M>`.
pub trait MetaTranspose {
	/// Located value type.
	type Value;

	/// Metadata type.
	type Metadata;

	/// Transposes a `Option<Meta<Self::Value, Self::Metadata>>` into a `Meta<Option<Self::Value>, Self::Metadata>`.
	fn meta_transpose(
		self,
		none_metadata: impl FnOnce() -> Self::Metadata,
	) -> Meta<Option<Self::Value>, Self::Metadata>;
}

impl<T, M> MetaTranspose for Option<Meta<T, M>> {
	type Value = T;
	type Metadata = M;

	#[inline(always)]
	fn meta_transpose(self, none_metadata: impl FnOnce() -> M) -> Meta<Option<T>, M> {
		match self {
			Some(Meta(t, m)) => Meta(Some(t), m),
			None => Meta(None, none_metadata()),
		}
	}
}

/// Locates the error of a `Result<T, E>`.
pub trait ErrAt {
	/// Success type.
	type Value;

	/// Error type.
	type Error;

	/// Changes a `Result<Self::Value, Self::Error>` into a `Result<Self::Value, Meta<Self::Error, M>>` by wrapping
	/// any eventual error using the result of the `metadata` function.
	fn err_at<M>(self, metadata: impl FnOnce() -> M) -> Result<Self::Value, Meta<Self::Error, M>>;
}

impl<T, E> ErrAt for Result<T, E> {
	type Value = T;
	type Error = E;

	#[inline(always)]
	fn err_at<M>(self, metadata: impl FnOnce() -> M) -> Result<Self::Value, Meta<Self::Error, M>> {
		match self {
			Ok(t) => Ok(t),
			Err(e) => Err(Meta(e, metadata())),
		}
	}
}

/// Maps the located error of a `Result<T, Meta<E, F>>`.
pub trait MapLocErr {
	/// Success type.
	type Value;

	/// Error type.
	type Error;

	/// Metadata type.
	type Metadata;

	/// Changes a `Result<Self::Value, Meta<Self::Error, Self::Metadata>>` into a `Result<Self::Value, Meta<G, Self::Metadata>>`
	/// by mapping the error value using `f`.
	fn map_loc_err<G>(
		self,
		f: impl FnOnce(Self::Error) -> G,
	) -> Result<Self::Value, Meta<G, Self::Metadata>>;
}

impl<T, E, M> MapLocErr for Result<T, Meta<E, M>> {
	type Value = T;
	type Error = E;
	type Metadata = M;

	#[inline(always)]
	fn map_loc_err<G>(
		self,
		f: impl FnOnce(Self::Error) -> G,
	) -> Result<Self::Value, Meta<G, Self::Metadata>> {
		match self {
			Ok(t) => Ok(t),
			Err(Meta(e, m)) => Err(Meta(f(e), m)),
		}
	}
}
