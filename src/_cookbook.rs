//! # Cookbook
//!
//! These are short recipes for accomplishing common tasks.
//!
//! * [Whitespace](#whitespace)
//!   + [Wrapper combinators that eat whitespace before and after a parser](#wrapper-combinators-that-eat-whitespace-before-and-after-a-parser)
//! * [Comments](#comments)
//!   + [`// C++/EOL-style comments`](#-ceol-style-comments)
//!   + [`/* C-style comments */`](#-c-style-comments-)
//! * [Identifiers](#identifiers)
//!   + [`Rust-Style Identifiers`](#rust-style-identifiers)
//! * [Literal Values](#literal-values)
//!   + [Escaped Strings](#escaped-strings)
//!   + [Integers](#integers)
//!     - [Hexadecimal](#hexadecimal)
//!     - [Octal](#octal)
//!     - [Binary](#binary)
//!     - [Decimal](#decimal)
//!   + [Floating Point Numbers](#floating-point-numbers)
//! * [Implementing `FromStr`](#implementing-fromstr)
//!
//! ## Whitespace
//!
//!
//!
//! ### Wrapper combinators that eat whitespace before and after a parser
//!
//! ```rust
//! use winnow::prelude::*;
//! use winnow::{
//!   error::ParseError,
//!   sequence::delimited,
//!   character::multispace0,
//! };
//!
//! /// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
//! /// trailing whitespace, returning the output of `inner`.
//! fn ws<'a, F, O, E: ParseError<&'a str>>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
//!   where
//!   F: FnMut(&'a str) -> IResult<&'a str, O, E>,
//! {
//!   delimited(
//!     multispace0,
//!     inner,
//!     multispace0
//!   )
//! }
//! ```
//!
//! To eat only trailing whitespace, replace `delimited(...)` with `terminated(&inner, multispace0)`.
//! Likewise, the eat only leading whitespace, replace `delimited(...)` with `preceded(multispace0,
//! &inner)`. You can use your own parser instead of `multispace0` if you want to skip a different set
//! of lexemes.
//!
//! ## Comments
//!
//! ### `// C++/EOL-style comments`
//!
//! This version uses `%` to start a comment, does not consume the newline character, and returns an
//! output of `()`.
//!
//! ```rust
//! use winnow::prelude::*;
//! use winnow::{
//!   error::ParseError,
//!   sequence::pair,
//!   bytes::take_till1,
//! };
//!
//! pub fn peol_comment<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, (), E>
//! {
//!   pair('%', take_till1("\n\r"))
//!     .value(()) // Output is thrown away.
//!     .parse_next(i)
//! }
//! ```
//!
//! ### `/* C-style comments */`
//!
//! Inline comments surrounded with sentinel tags `(*` and `*)`. This version returns an output of `()`
//! and does not handle nested comments.
//!
//! ```rust
//! use winnow::prelude::*;
//! use winnow::{
//!   error::ParseError,
//!   bytes::{tag, take_until},
//! };
//!
//! pub fn pinline_comment<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, (), E> {
//!   (
//!     "(*",
//!     take_until("*)"),
//!     "*)"
//!   )
//!     .value(()) // Output is thrown away.
//!     .parse_next(i)
//! }
//! ```
//!
//! ## Identifiers
//!
//! ### `Rust-Style Identifiers`
//!
//! Parsing identifiers that may start with a letter (or underscore) and may contain underscores,
//! letters and numbers may be parsed like this:
//!
//! ```rust
//! use winnow::prelude::*;
//! use winnow::{
//!   branch::alt,
//!   multi::many0_count,
//!   sequence::pair,
//!   character::{alpha1, alphanumeric1},
//!   bytes::tag,
//! };
//!
//! pub fn identifier(input: &str) -> IResult<&str, &str> {
//!   pair(
//!     alt((alpha1, "_")),
//!     many0_count(alt((alphanumeric1, "_")))
//!   )
//!      .recognize()
//!      .parse_next(input)
//! }
//! ```
//!
//! Let's say we apply this to the identifier `hello_world123abc`. The first `alt` parser would
//! recognize `h`. The `pair` combinator ensures that `ello_world123abc` will be piped to the next
//! `alphanumeric0` parser, which recognizes every remaining character. However, the `pair` combinator
//! returns a tuple of the results of its sub-parsers. The `recognize` parser produces a `&str` of the
//! input text that was parsed, which in this case is the entire `&str` `hello_world123abc`.
//!
//! ## Literal Values
//!
//! ### Escaped Strings
//!
//! ```rust
#![doc = include_str!("../examples/string.rs")]
//! ```
//!
//! ### Integers
//!
//! The following recipes all return string slices rather than integer values. How to obtain an
//! integer value instead is demonstrated for hexadecimal integers. The others are similar.
//!
//! The parsers allow the grouping character `_`, which allows one to group the digits by byte, for
//! example: `0xA4_3F_11_28`. If you prefer to exclude the `_` character, the lambda to convert from a
//! string slice to an integer value is slightly simpler. You can also strip the `_` from the string
//! slice that is returned, which is demonstrated in the second hexadecimal number parser.
//!
//! If you wish to limit the number of digits in a valid integer literal, replace `many1` with
//! `many_m_n` in the recipes.
//!
//! #### Hexadecimal
//!
//! The parser outputs the string slice of the digits without the leading `0x`/`0X`.
//!
//! ```rust
//! use winnow::prelude::*;
//! use winnow::{
//!   branch::alt,
//!   multi::{many0, many1},
//!   sequence::{preceded, terminated},
//!   bytes::one_of,
//!   bytes::tag,
//! };
//!
//! fn hexadecimal(input: &str) -> IResult<&str, &str> { // <'a, E: ParseError<&'a str>>
//!   preceded(
//!     alt(("0x", "0X")),
//!     many1(
//!       terminated(one_of("0123456789abcdefABCDEF"), many0('_'))
//!     ).recognize()
//!   )(input)
//! }
//! ```
//!
//! If you want it to return the integer value instead, use map:
//!
//! ```rust
//! use winnow::prelude::*;
//! use winnow::{
//!   branch::alt,
//!   multi::{many0, many1},
//!   sequence::{preceded, terminated},
//!   bytes::one_of,
//!   bytes::tag,
//! };
//!
//! fn hexadecimal_value(input: &str) -> IResult<&str, i64> {
//!   preceded(
//!     alt(("0x", "0X")),
//!     many1(
//!       terminated(one_of("0123456789abcdefABCDEF"), many0('_'))
//!     ).recognize()
//!   ).map_res(
//!     |out: &str| i64::from_str_radix(&str::replace(&out, "_", ""), 16)
//!   ).parse_next(input)
//! }
//! ```
//!
//! #### Octal
//!
//! ```rust
//! use winnow::prelude::*;
//! use winnow::{
//!   branch::alt,
//!   multi::{many0, many1},
//!   sequence::{preceded, terminated},
//!   bytes::one_of,
//!   bytes::tag,
//! };
//!
//! fn octal(input: &str) -> IResult<&str, &str> {
//!   preceded(
//!     alt(("0o", "0O")),
//!     many1(
//!       terminated(one_of("01234567"), many0('_'))
//!     ).recognize()
//!   )(input)
//! }
//! ```
//!
//! #### Binary
//!
//! ```rust
//! use winnow::prelude::*;
//! use winnow::{
//!   branch::alt,
//!   multi::{many0, many1},
//!   sequence::{preceded, terminated},
//!   bytes::one_of,
//!   bytes::tag,
//! };
//!
//! fn binary(input: &str) -> IResult<&str, &str> {
//!   preceded(
//!     alt(("0b", "0B")),
//!     many1(
//!       terminated(one_of("01"), many0('_'))
//!     ).recognize()
//!   )(input)
//! }
//! ```
//!
//! #### Decimal
//!
//! ```rust
//! use winnow::prelude::*;
//! use winnow::{
//!   IResult,
//!   multi::{many0, many1},
//!   sequence::terminated,
//!   bytes::one_of,
//! };
//!
//! fn decimal(input: &str) -> IResult<&str, &str> {
//!   many1(
//!     terminated(one_of("0123456789"), many0('_'))
//!   )
//!     .recognize()
//!     .parse_next(input)
//! }
//! ```
//!
//! ### Floating Point Numbers
//!
//! The following is adapted from [the Python parser by Valentin Lorentz](https://github.com/ProgVal/rust-python-parser/blob/master/src/numbers.rs).
//!
//! ```rust
//! use winnow::prelude::*;
//! use winnow::{
//!   branch::alt,
//!   multi::{many0, many1},
//!   combinator::opt,
//!   sequence::{preceded, terminated},
//!   bytes::one_of,
//! };
//!
//! fn float(input: &str) -> IResult<&str, &str> {
//!   alt((
//!     // Case one: .42
//!     (
//!       '.',
//!       decimal,
//!       opt((
//!         one_of("eE"),
//!         opt(one_of("+-")),
//!         decimal
//!       ))
//!     ).recognize()
//!     , // Case two: 42e42 and 42.42e42
//!     (
//!       decimal,
//!       opt(preceded(
//!         '.',
//!         decimal,
//!       )),
//!       one_of("eE"),
//!       opt(one_of("+-")),
//!       decimal
//!     ).recognize()
//!     , // Case three: 42. and 42.42
//!     (
//!       decimal,
//!       '.',
//!       opt(decimal)
//!     ).recognize()
//!   ))(input)
//! }
//!
//! fn decimal(input: &str) -> IResult<&str, &str> {
//!   many1(
//!     terminated(one_of("0123456789"), many0('_'))
//!   )
//!     .recognize()
//!     .parse_next(input)
//! }
//! ```
//!
//! # Implementing `FromStr`
//!
//! The [`FromStr` trait][std::str::FromStr] provides
//! a common interface to parse from a string.
//!
//! ```rust
#![doc = include_str!("../examples/css/parser.rs")]
//! ```