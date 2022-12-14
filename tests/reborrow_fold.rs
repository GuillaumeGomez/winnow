#![allow(dead_code)]
// #![allow(unused_variables)]

use std::str;

use nom::bytes::is_not;
use nom::character::char;
use nom::combinator::map_res;
use nom::multi::fold_many0;
use nom::prelude::*;
use nom::sequence::delimited;
use nom::IResult;

fn atom<'a>(_tomb: &'a mut ()) -> impl FnMut(&'a [u8]) -> IResult<&'a [u8], String> {
  move |input| {
    map_res(is_not(" \t\r\n"), str::from_utf8)
      .map(ToString::to_string)
      .parse(input)
  }
}

// FIXME: should we support the use case of borrowing data mutably in a parser?
fn list<'a>(i: &'a [u8], tomb: &'a mut ()) -> IResult<&'a [u8], String> {
  delimited(
    char('('),
    fold_many0(atom(tomb), String::new, |acc: String, next: String| {
      acc + next.as_str()
    }),
    char(')'),
  )(i)
}
