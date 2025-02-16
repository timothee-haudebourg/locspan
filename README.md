# Code mapping utility types and traits.

[![CI](https://github.com/timothee-haudebourg/locspan/workflows/CI/badge.svg)](https://github.com/timothee-haudebourg/locspan/actions)
[![Crate informations](https://img.shields.io/crates/v/locspan.svg?style=flat-square)](https://crates.io/crates/locspan)
[![License](https://img.shields.io/crates/l/locspan.svg?style=flat-square)](https://github.com/timothee-haudebourg/locspan#license)
[![Documentation](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://docs.rs/locspan)

<!-- cargo-rdme start -->

This library provides essentials types and trait to locate syntax elements.

- `Span` describes a byte range in a source file. It is very similar to
  `Range<usize>` but is not an iterator and implements `Copy`. It also
   provides an intuitive API to write lexers and parsers.
- `Location<F>` combines a `Span` with a file identifier `F` to pin point a
  syntactic element in a source file.

The crate integrates well with
[`codespan-reporting`](https://crates.io/crates/codespan-reporting) library
to render beautiful error reports. Enable the `reporting` feature to add
dedicated methods to convert a `Location` value into a
`codespan_reporting::diagnostic::Label`.

<!-- cargo-rdme end -->

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
