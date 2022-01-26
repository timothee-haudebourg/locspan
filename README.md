# Code mapping utility types and traits.

This library provides essentials types and trait to locate syntax elements.

- [`Span`] describes a byte range in a source file, with an intuitive API to write lexers and parsers.
- [`Location<F>`] combines a `Span` with a file identifier `F` to pin point a syntactic element in a source file.
- [`Loc<T, F>`] wraps any value `T` and bind it to its location.

Extra traits are also provided to extend common types (`Option`, `Result`, etc.) with localization functions.
The crate integrates well with diagnostic reporting libraries such as
[`codespan-reporting`](https://crates.io/crates/codespan-reporting) to render beautiful error reports.

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
