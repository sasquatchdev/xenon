// FIXME: Remove for prod.
#![allow(unused)]

/// References a location winthin some source.
#[derive(Debug, Clone)]
pub struct Location {
  /// The index within _source_.
  /// => source[index] should be valid
  index: usize,
  source: String,
}

impl Location {
  /// Creates a new `Location`
  pub fn new(index: usize, source: String) -> Self {
    Self { index, source }
  }
  
  /// Creates a `Location` for the start of the source
  pub fn new_start(source: String) -> Self {
    Self::new(0, source)
  }

  // Creates a _Location_ for the end of the source
  pub fn new_end(source: String) -> Self {
    Self::new(source.len() - 1, source)
  }

  /// Advances to the next index
  pub fn advance(&mut self) {
    self.index += 1;
  }

  /// Returns a copy that is advanced to the next index
  pub fn advanced(&self) -> Self {
    Self { source: self.source.clone(), index: self.index + 1 }
  }
}

/// References a span within some source.
/// Wrapper around two `Locations`
#[derive(Debug, Clone)]
pub struct Span {
  start: Location,
  end: Location,
}

impl Span {
  /// Creates a new `Span`.
  /// Should almost never be used. Use `new_shared` instead
  pub fn new(start: Location, end: Location) -> Self {
    Self { start, end }
  }

  // Creates a new `Span` with the same source
  pub fn new_shared(start: usize, end: usize, source: String) -> Self {
    let start = Location::new(start, source.clone());
    let end = Location::new(end, source);
    Self::new(start, end)
  }

  // Creates a new `Span` from a start and length
  pub fn new_length(start: usize, length: usize, source: String) -> Self {
    Self::new_shared(start, start + length, source)
  }
}