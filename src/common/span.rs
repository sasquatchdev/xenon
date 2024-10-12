// FIXME: Remove for prod.
#![allow(unused)]

/// References a location winthin some source.
#[derive(Debug, Clone)]
pub struct Location<'a> {
  /// The index within _source_.
  /// => source[index] should be valid
  index: usize,
  source: &'a str,
}

impl<'a> Location<'a> {
  /// Creates a new `Location`
  pub fn new(index: usize, source: &'a str) -> Self {
    Self { index, source }
  }
  
  /// Creates a `Location` for the start of the source
  pub fn new_start(source: &'a str) -> Self {
    Self::new(0, source)
  }

  // Creates a _Location_ for the end of the source
  pub fn new_end(source: &'a str) -> Self {
    Self::new(source.len() - 1, source)
  }

  /// Advances to the next index
  pub fn advance(&mut self) {
    self.index += 1;
  }

  /// Returns a copy that is advanced to the next index
  pub fn advanced(&self) -> Self {
    Self { source: self.source, index: self.index + 1 }
  }
}

/// References a span within some source.
/// Wrapper around two `Locations`
#[derive(Debug, Clone)]
pub struct Span<'a> {
  start: Location<'a>,
  end: Location<'a>,
}

impl<'a> Span<'a> {
  /// Creates a new `Span`.
  /// Should almost never be used. Use `new_shared` instead
  pub fn new(start: Location<'a>, end: Location<'a>) -> Self {
    Self { start, end }
  }

  // Creates a new `Span` with the same source
  pub fn new_shared(start: usize, end: usize, source: &'a str) -> Self {
    let start = Location::new(start, source);
    let end = Location::new(end, source);
    Self::new(start, end)
  }

  // Creates a new `Span` from a start and length
  pub fn new_length(start: usize, length: usize, source: &'a str) -> Self {
    Self::new_shared(start, start + length, source)
  }
}