use crate::common::{flag::Flag, span::Span};

pub struct Token<'a> {
  span: Span<'a>,
  flag: Flag<'a>,
  kind: TokenKind,
  lexeme: &'a str
}

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