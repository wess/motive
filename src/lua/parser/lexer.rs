//
// lexer.rs
// motive
// 
// Author: wess (me@wess.io)
// Created: 02/17/2022
// 
// Copywrite (c) 2022 Wess.io
//

use logos::Logos;
use std::convert::TryFrom;
use std::ops::Range as StdRange;
use text_size::{TextRange, TextSize};

use super::token::Token;

pub struct Lexer<'a> {
  inner: logos::Lexer<'a, Token>,
}

impl<'a> Lexer<'a> {
  pub fn new(input: &'a str) -> Self {
    Self {
      inner: Token::lexer(input),
    }
  }
}

impl<'a> Iterator for Lexer<'a> {
  type Item = (&'a str, Token);

  fn next(&mut self) -> Option<Self::Item> {
    let kind = self.inner.next()?;
    let text = self.inner.slice();

    Some(
      (text, kind)
    )
  }
}
