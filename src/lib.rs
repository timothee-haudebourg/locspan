//! This library provides essentials types and trait to locate syntax elements.
//!
//! - `Span` describes a byte range in a source file. It is very similar to
//!   `Range<usize>` but is not an iterator and implements `Copy`. It also
//!    provides an intuitive API to write lexers and parsers.
//! - `Location<F>` combines a `Span` with a file identifier `F` to pin point a
//!   syntactic element in a source file.
//!
//! The crate integrates well with
//! [`codespan-reporting`](https://crates.io/crates/codespan-reporting) library
//! to render beautiful error reports. Enable the `reporting` feature to add
//! dedicated methods to convert a `Location` value into a
//! `codespan_reporting::diagnostic::Label`.
mod location;
mod span;

#[cfg(feature = "reporting")]
mod reporting;

pub use location::*;
pub use span::*;
