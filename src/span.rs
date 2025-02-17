use std::ops::{Index, IndexMut, Range};

/// Range of bytes in a source file.
///
/// This is very similar to the [`Range<usize>`] type unless it is not an iterator
/// and implements the `Copy` trait.
#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct Span {
	/// Start byte (included).
	pub start: usize,

	/// End byte (excluded).
	pub end: usize,
}

impl Span {
	/// Creates a new `Span` by providing the index of its starting byte (included) and ending byte (excluded).
	///
	/// If the `end` position is lower than the `start` position,
	/// then the `start` position is used as ending position instead.
	#[inline(always)]
	pub fn new(start: usize, end: usize) -> Self {
		Self {
			start,
			end: std::cmp::max(start, end),
		}
	}

	/// Size of the span in bytes.
	#[inline(always)]
	pub fn len(&self) -> usize {
		if self.start > self.end {
			0
		} else {
			self.end - self.start
		}
	}

	/// Checks if the span is empty.
	///
	/// Returns `true` if the start position is equal to the end position,
	/// and `false` otherwise.
	/// The end position can never be lower than the start position.
	#[inline(always)]
	pub fn is_empty(&self) -> bool {
		self.end == self.start
	}

	/// Returns the range of bytes inside the span.
	#[inline(always)]
	pub fn range(&self) -> Range<usize> {
		self.start..self.end
	}

	/// Checks if the given byte index if included in the span.
	#[inline(always)]
	pub fn contains(&self, index: usize) -> bool {
		self.start >= index && index < self.end
	}

	/// Computes the union of two spans.
	///
	/// If the two spans do not overlap,
	/// all the positions in between will be included in the resulting span.
	#[inline(always)]
	pub fn union(&self, other: Self) -> Self {
		Self {
			start: std::cmp::min(self.start, other.start),
			end: std::cmp::max(self.end, other.end),
		}
	}

	/// Extend this span to include `other`.
	///
	/// This is the *in-place* version of [`union`](Self::union).
	#[inline(always)]
	pub fn append(&mut self, other: Self) {
		self.start = std::cmp::min(self.start, other.start);
		self.end = std::cmp::max(self.end, other.end)
	}

	/// Computes the intersection of two spans.
	///
	/// If the two spans do not overlap,
	/// then the empty span located at the start of the most advanced span
	/// (maximum of the start of the two spans) is returned.
	#[inline(always)]
	pub fn inter(&self, other: Self) -> Self {
		let start = std::cmp::max(self.start, other.start);
		Self {
			start,
			end: std::cmp::max(start, std::cmp::min(self.end, other.end)),
		}
	}

	/// Clear the span by moving its start position to its end position.
	#[inline(always)]
	pub fn clear(&mut self) {
		self.start = self.end
	}

	/// Return the "next" span: the empty span starting at the end of this one.
	#[inline(always)]
	pub fn next(&self) -> Self {
		self.end.into()
	}

	/// Push `count` bytes to the span.
	///
	/// Move its end position by `count`.
	#[inline(always)]
	pub fn push(&mut self, count: usize) {
		self.end += count
	}
}

impl From<usize> for Span {
	fn from(pos: usize) -> Self {
		Self::new(pos, pos)
	}
}

impl From<Range<usize>> for Span {
	fn from(range: Range<usize>) -> Self {
		Self::new(range.start, range.end)
	}
}

impl From<Span> for Range<usize> {
	fn from(span: Span) -> Self {
		Self {
			start: span.start,
			end: span.end,
		}
	}
}

impl IntoIterator for Span {
	type Item = usize;
	type IntoIter = Range<usize>;

	fn into_iter(self) -> Self::IntoIter {
		self.range()
	}
}

impl IntoIterator for &Span {
	type Item = usize;
	type IntoIter = Range<usize>;

	fn into_iter(self) -> Self::IntoIter {
		self.range()
	}
}

impl Index<Span> for str {
	type Output = str;

	fn index(&self, span: Span) -> &str {
		self.index(span.range())
	}
}

impl IndexMut<Span> for str {
	fn index_mut(&mut self, span: Span) -> &mut str {
		self.index_mut(span.range())
	}
}

impl Index<Span> for String {
	type Output = str;

	fn index(&self, span: Span) -> &str {
		self.index(span.range())
	}
}

impl IndexMut<Span> for String {
	fn index_mut(&mut self, span: Span) -> &mut str {
		self.index_mut(span.range())
	}
}

/// Value with a span.
pub trait Spanned {
	fn span(&self) -> Span;
}

impl Spanned for Span {
	fn span(&self) -> Span {
		*self
	}
}

/// Value with an optional span.
pub trait MaybeSpanned {
	fn optional_span(&self) -> Option<Span>;
}

impl MaybeSpanned for Span {
	fn optional_span(&self) -> Option<Span> {
		Some(*self)
	}
}
