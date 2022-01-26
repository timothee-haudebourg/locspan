//! This library provides essentials types and trait to locate syntax elements.
//!
//! - `Span` describes a byte range in a source file, with an intuitive API to write lexers and parsers.
//! - `Location<F>` combines a `Span` with a file identifier `F` to pin point a syntactic element in a source file.
//! - `Loc<T, F>` wraps any value `T` and bind it to its location.
//!
//! Extra traits are also provided to extend common types (`Option`, `Result`, etc.) with localization functions.
//! The crate integrates well with diagnostic reporting libraries such as
//! [`codespan-reporting`](https://crates.io/crates/codespan-reporting) to render beautiful error reports.
mod loc;
mod location;
mod span;

pub use loc::*;
pub use location::*;
pub use span::*;
