//
// mod.rs
// motive
// 
// Author: wess (me@wess.io)
// Created: 02/17/2022
// 
// Copywrite (c) 2022 Wess.io
//

use std::collections::HashMap;
use std::iter::Peekable;
use logos::Logos;

mod token;
mod lexer;
mod entry;

use lexer::Lexer;
use token::Token;
use entry::Entry;

pub struct Parser<'a> {
  lexer: Peekable<Lexer<'a>>,
  source: &'a str,
  previous: Option<Entry>,
  pub functions: Vec<String>,
  pub tasks: HashMap<String, String>,
  pub watches: HashMap<String, String>,
  pub exports: HashMap<String, String>,
  pub assignments: Vec<String>,
}

impl<'a> Parser<'a> {
  pub fn new(input: &'a str) -> Self {
    Self{
      lexer: Lexer::new(input).peekable(),
      source: input,
      previous: None,
      functions: Vec::new(),
      tasks: HashMap::new(),
      watches: HashMap::new(),
      exports: HashMap::new(),
      assignments: Vec::new(),
    }
  }

  fn peek(&mut self) -> Option<Entry> {
    self.lexer
    .peek()
    .map(|(text, kind)| Entry{text: string!(*text), token: *kind})
  }

  fn chomp(&mut self) {
    self.previous = self.peek();

    match self.lexer.next() {
      Some((_text, _kind)) => {},
      None => {}
    }
  }

  fn parse_function(&mut self) -> Vec<String> {
    let mut source:Vec<String> = vec![];
    let mut current:Vec<String> = vec![];

    while let Some(entry) = self.peek() {
      match entry.token {
        Token::Newline => {
          if current.len() > 0 {
            source.push(current.join(""));
            current.clear();
          }
        },
        Token::End => {
          source.push(string!("end"));

          break;
        }
        Token::Identifier => {
          if source.len() == 0 {
            self.functions.push(entry.text.clone());
          }

          current.push(entry.text.to_string());
        },
        Token::Comment => {},
        Token::Function |
        Token::Symbols |
        Token::LiteralString => {
          current.push(entry.text.to_string());
        },
         
        _ => {
          current.push(entry.text.to_string());
        }
  
      }

      self.chomp();
    }

    source
  }
 
  fn parse_task(&mut self) -> Vec<String> {
    let mut name:String = string!("");
    let mut description:String = string!("");
    let mut source:Vec<String> = vec![];
    let mut current:Vec<String> = vec![];

    while let Some(entry) = self.peek() {
      match entry.token {
        Token::Newline => {
          if current.len() > 0 {
            source.push(current.join(""));
            current.clear();
          }
        },
        Token::End => {
          source.push(string!("end"));

          break;
        },
        Token::Block => {
          let mut block = self.parse_block();
          source.append(&mut block);
        }
        Token::Task => {},
        Token::Identifier => {
          if !self.functions.contains(&entry.text) && source.len() == 0 {
            name = entry.text.clone();

            current.push(
              format!("motive.tasks.{} = function()", entry.text.to_string())
            );
          } else {
            current.push(entry.text);
          }
        },
        Token::Comment => {
          if description.len() == 0 {
            description = entry.text.replace("--", "").trim().to_string();
          }
        },
        Token::Symbols |
        Token::LiteralString => {
          current.push(entry.text.to_string());
        },
         
        _ => {
          current.push(entry.text.to_string());
        }
  
      }

      self.chomp();
    }

    self.tasks.insert(name.clone(), description.clone());
    source
  }

  fn parse_block(&mut self) -> Vec<String> {
    let mut source:Vec<String> = vec![];
    let mut current:Vec<String> = vec![];

    while let Some(entry) = self.peek() {
      match entry.token {
        Token::Newline => {
          if current.len() > 0 {
            source.push(current.join(""));
            current.clear();
          }
        },
        Token::End => {
          source.push(string!("end"));

          break;
        },
        Token::Block => {
          if source.len() > 0 {
            let mut block = self.parse_block();
            source.append(&mut block);
          } else {
            current.push(entry.text);
          }
        }
        Token::Comment => {},
        Token::Symbols |
        Token::Local |
        Token::BuiltIn |
        Token::LiteralString => {
          current.push(entry.text.to_string());
        },
         
        _ => {
          current.push(entry.text.to_string());
        }
  
      }

      self.chomp();
    
    }

    source
  }

  pub fn parse_local(&mut self) -> String {
    self.chomp();

    let mut source:Vec<String> = vec![string!("local")];
  
    while let Some(entry) = self.peek() {
      match entry.token {
        Token::Newline => {
          break;
        }
        Token::Identifier => {
          self.assignments.push(entry.text.clone());

          source.push(entry.text);
        },
        _ => {
          source.push(entry.text);
        }
      }

      self.chomp();
    }

    source.join("")
  }

  pub fn run(&mut self) -> crate::Result<String> {
    let mut source:Vec<String> = vec![string!("-- Generated by Motive")];

    while let Some(entry) = self.peek() {
      match entry.token {
        Token::Function => {
          let mut function = self.parse_function();
          source.append(&mut function);
        },
        Token::Local => {
          let local = self.parse_local();

          source.push(local);
        },
        Token::Task => {
          let mut task = self.parse_task();
          source.append(&mut task);
        },
        Token::Newline |
        Token::Comment => {},
        _ => {
        }
      }

      self.chomp();
    }

    Ok(source.join("\n"))
  }
}