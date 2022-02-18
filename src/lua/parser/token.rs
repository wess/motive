//
// token.rs
// motive
// 
// Author: wess (me@wess.io)
// Created: 02/17/2022
// 
// Copywrite (c) 2022 Wess.io
//

use std::fmt;
use logos::Logos;
use num_derive::{FromPrimitive, ToPrimitive};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Logos, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, EnumIter, FromPrimitive, ToPrimitive)]
#[repr(u16)]
pub enum Token {
  #[regex(r"[\r?\n]")]
  Newline,

  #[regex(r"[ \t]*")]
  Whitespace,

  #[regex(r#""(?:\\"|\\'|[^"])*""#)]
  #[regex(r#"'(?:\\"|\\'|[^'])*'"#)]
  LiteralString,

  #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
  Identifier,
  #[token("#")]
  #[token("{")]
  #[token("}")]
  #[token("(")]
  #[token(")")]
  #[token(",")]
  #[token("=")]
  Symbols,

  #[token("end")]
  End,   

  #[token("function")]
  Function,

  #[token("local")]
  Local,     

  #[token("until( \t)+?(.*)$")]
  Until,

  #[token("else")]
  #[token("elseif")]
  #[token("for")]
  #[token("if")]
  #[token("repeat")]
  #[token("while")]
  Block,

  #[token("and")]
  #[token("break")]
  #[token("do")]
  #[token("false")]
  #[token("in")]
  #[token("nil")]
  #[token("not")]
  #[token("or")]
  #[token("require")]
  #[token("return")]
  #[token("then")]
  #[token("true")]
  #[token("print")]
  #[token("arguments")]
  BuiltIn,

  #[token("export")]
  Export,

  #[token("task")]
  Task,

  #[token("watch")]
  Watch,

  #[token("@")]
  Mute,

  #[regex(r"--[^\n]*")]
  Comment,

  #[error]
  Error,
}

impl Token {
  pub fn list() -> Vec<String> {
    let mut tokens: Vec<String> = vec![];

    for token in Token::iter() {
      tokens.push(token.to_string());
    }

    tokens
  }
}
  
impl fmt::Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(match self {
      Self::Newline => "newline",
      Self::LiteralString => "literal_string",
      Self::Whitespace => "whitespace",
      Self::Identifier => "identifier",
      Self::Symbols => "symbols",
      Self::End => "end",
      Self::Function => "function",
      Self::Local => "local",     
      Self::Until => "until",
      Self::Block => "block",
      Self::BuiltIn => "built-in",
      Self::Export => "export",
      Self::Task => "task",
      Self::Watch => "watch",
      Self::Mute => "@",
      Self::Comment => "comment",
      Self::Error => "Syntax error",
    })
  }
}
