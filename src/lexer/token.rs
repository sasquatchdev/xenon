use crate::common::{flag::Flag, span::Span};

/// Represents a single token in the source code.
/// This could be a keyword, identifier, literal, etc.
#[derive(Debug, Clone)]
pub struct Token {
  span: Span,
  flag: Flag,
  kind: TokenKind,
  lexeme: String
}

/// Represents the kind of a token.
/// This is used to determine how to interpret the token.
#[derive(Debug, Clone)]
pub enum TokenKind {
  FloatingPointLiteral(f64),

  /*  
    There are no "signed-int" literals since negative
    numbers come from the unary inversion operator 
  */
  UnsignedIntegerLiteral(u64),

  /*
    Arithmetic Binary Operators
  */
  Plus, Minus, Asterisk, Slash, Percent,
}