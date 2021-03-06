//
// lib.rs
// motive
// 
// Author: wess (me@wess.io)
// Created: 02/11/2022
// 
// Copywrite (c) 2022 Wess.io
//

#![allow(dead_code)]
#![allow(unused_imports)]

use std::env;

use clap::{
  App,
  AppSettings,
  Arg,
};

extern crate logos;

#[macro_use]
extern crate oxide;

pub mod macros;

pub mod result;
pub use result::Result;

mod lua;
mod watch;

mod commands;
use commands::{
  Init,
};


pub async fn run() -> Result<()> {
  if cfg!(debug_assertions) {
  } else {
    std::panic::set_hook(Box::new(|_| {
      console_error!("Something went wrong...");
    }));  
  }

  let current_dir = env::current_dir().unwrap();
  let manifest = current_dir.join("manifest");

  let mut engine:Option<lua::Engine> = None;

  if manifest.exists() {
    let content = file_read!(&manifest);
    engine = Some(lua::Engine::new(content).unwrap());
  }

  let mut app = 
    App::new("Motive")
      .version(env!("CARGO_PKG_VERSION"))
      .about("Developer environment manager.")
      .before_help("\n")
      .after_help("\n")
      .setting(AppSettings::AllowExternalSubcommands)
      .setting(AppSettings::ArgRequiredElseHelp)
      .setting(AppSettings::TrailingVarArg)
      // .arg(Arg::new("vargs").multiple_values(true))
      .subcommand(Init::app())
      .subcommand(
        App::new("list")
              .about("List available tasks from Manifest")
      );


  let mut help = vec![];
  app.write_help(&mut help).unwrap();

  let matches = app.get_matches();
  match matches.subcommand_name() {
    Some("init") => Init::run(),
    Some("list") => {
      match engine {
        Some(e) => e.list(),
        None => console_panic!("No manifest found"),
      }
    },
    Some(cmd) => {
      println!();

      let e = match engine {
        Some(e) => e,
        None => console_panic!("No manifest found, run `motive init` to create one"),
      };

      if e.has_task(string!(cmd)) {
        let mut vargs: Option<Vec<&str>> = None;

        match matches.subcommand() {
          Some((_, sub_m)) => {

            match sub_m.values_of("") {
              Some(v) => vargs = Some(v.collect::<Vec<&str>>()),
              None => {},
            }
          },
          None => {},
        }

        e.run(
          string!(cmd),
          vargs,
        ).await?;
      } else {
        console_error!("Unknown command: {}", cmd);
        println!("{}", String::from_utf8_lossy(&help));  
      }
    },
    None => {},
  }

  println!();
  Ok(())
}