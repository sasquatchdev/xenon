// FIXME: Remove for prod.
#![allow(unused)]

#[derive(Debug, Clone)]
pub struct Location<'a> {
  index: usize,
  source: &'a str,
}

impl<'a> Location<'a> {
  pub fn new(index: usize, source: &'a str) -> Self {
    Self { index, source }
  }
  
  pub fn new_start(source: &'a str) -> Self {
    Self::new(0, source)
  }

  pub fn new_end(source: &'a str) -> Self {
    Self::new(source.len() - 1, source)
  }
}

#[derive(Debug, Clone)]
pub struct Span<'a> {
  start: Location<'a>,
  end: Location<'a>,
}

impl<'a> Span<'a> {
  pub fn new(start: Location<'a>, end: Location<'a>) -> Self {
    Self { start, end }
  }

  pub fn new_shared(start: usize, end: usize, source: &'a str) -> Self {
    let start = Location::new(start, source);
    let end = Location::new(end, source);
    Self::new(start, end)
  }

  pub fn new_length(start: usize, length: usize, source: &'a str) -> Self {
    Self::new_shared(start, start + length, source)
  }
}