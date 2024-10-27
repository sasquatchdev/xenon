pub mod token;
pub mod r#impl;

pub struct Lexer {
  source: String,
  current: usize,
}

impl Lexer {
  pub fn new(source: String) -> Self {
    Self { source, current: 0 }
  }

  pub fn advance(&mut self) {
    self.advance_by(1);
  }

  pub fn advance_by(&mut self, by: usize) {
    self.current += by;
  }

  pub fn chars(&self) -> Vec<char> {
    self.source.chars().collect::<Vec<char>>()
  }

  pub fn peek(&self) -> char {
    self.chars()[self.current]
  }

  pub fn pop(&mut self) -> char {
    self.advance();
    self.chars()[self.current - 1]
  }
}