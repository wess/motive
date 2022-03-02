//
// entry.rs
// motive
// 
// Author: wess (me@wess.io)
// Created: 02/17/2022
// 
// Copywrite (c) 2022 Wess.io
//

use super::token::Token;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Entry {
  pub token:Token,
  pub text:String,
}
