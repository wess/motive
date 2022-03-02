//
// error.rs
// motive
// 
// Author: wess (me@wess.io)
// Created: 02/18/2022
// 
// Copywrite (c) 2022 Wess.io
//

#[derive(Debug)]
pub enum Error {
  Io(std::io::Error),
  Notify(notify::Error),
}

impl std::fmt::Display for Error {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self {
          Self::Io(error) => error.fmt(fmt),
          Self::Notify(error) => error.fmt(fmt),
      }
  }
}

impl std::error::Error for Error {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
      match self {
          Self::Io(error) => error.source(),
          Self::Notify(error) => error.source(),
      }
  }
}

impl From<std::io::Error> for Error {
  fn from(err: std::io::Error) -> Self {
      Self::Io(err)
  }
}

impl From<notify::Error> for Error {
  fn from(err: notify::Error) -> Self {
      if let notify::Error::Io(err) = err {
          err.into()
      } else {
          Self::Notify(err)
      }
  }
}