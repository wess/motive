//
// result.rs
// motive
// 
// Author: Wess Cope (me@wess.io)
// Created: 12/01/2021
// 
// Copywrite (c) 2021 Wess.io
//

use std::result;

pub type Result<T> = result::Result<T, Box<dyn std::error::Error + Send + Sync>>;