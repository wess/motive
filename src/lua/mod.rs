//
// mod.rs
// motive
// 
// Author: wess (me@wess.io)
// Created: 02/11/2022
// 
// Copywrite (c) 2022 Wess.io
//

use std::{
  process::Command,
  io::{
    Write,
    stdout,
    stderr,
  }, collections::HashMap
};

use mlua::{
  prelude::*,
  chunk, 
  Variadic,
};

mod parser;

use colored::*;


const IGNORED_KEYWORDS: [&str; 22] = [
  "and",   
  "break",
  "do",    
  "else",
  "elseif",
  "end",   
  "false",     
  "for",
  "function",
  "if",
  "in",   
  "local",     
  "nil",     
  "not",      
  "or",
  "repeat",  
  "return",    
  "then",    
  "true",   
  "until",
  "while",
  "print",
];

pub struct EngineContext {
  pub functions: Vec<String>,
  pub tasks: HashMap<String, String>,
  pub watches: HashMap<String, String>,
  pub exports: HashMap<String, String>,
  pub assignments: Vec<String>,
}

impl Default for EngineContext {
  fn default() -> Self {
    Self {
      functions: Vec::new(),
      tasks: HashMap::new(),
      watches: HashMap::new(),
      exports: HashMap::new(),
      assignments: Vec::new(),
    }
  }
}

pub struct Engine {
  lua: Lua,
  source:String,
  pub context: EngineContext,
}

impl Engine {
  pub fn new(source:String) -> crate::Result<Engine> {
    let (src, context) = Self::translate(source);

    let this = Engine {
      lua: Lua::new(),
      source: src,
      context: context,
    };

    let motive = this.lua.create_table()?;
    motive.set("tasks", this.lua.create_table()?)?;
    motive.set("task_descriptions", this.lua.create_table()?)?;

    this.lua.globals().set("motive", motive)?;
    this.lua.globals().set("exec", this.lua.create_function(Self::exec)?)?;
  
    this.lua.load(&this.source).exec()?;
    
    Ok(this)
  }

  pub fn has_task(&self, name:String) -> bool {
    let keys = self.context.tasks.clone().into_iter().map(|(k, _)| k).collect::<Vec<String>>();

    return keys.contains(&name);
  }

  pub fn list(&self) {
    println!();

    println!("{}\n", "Motive Tasks:".bold());

    for (k, v) in &self.context.tasks {
      let mut out:Vec<String> = vec![format!("{}", k.bold())];
        
      if v.len() > 0 {
        out.push(format!(" - {}", v));
      }

      console_info!("{}", out.join("\t"));
    }

    println!();
  }

  pub async fn run(&self, task:String, vargs:Option<Vec<&str>>) -> LuaResult<()> {
    let args = self.lua.create_table().unwrap();

    for (i, arg) in vargs.unwrap_or(vec![]).iter().enumerate() {
      args.set(i, arg.clone()).unwrap();
    }
    
    self.lua.globals().set("arguments", args)?;

    let tasks = self.lua.globals().get::<_, LuaTable>("motive")?.get::<_, LuaTable>("tasks")?;
    let task:mlua::Function = tasks.get(task)?;
    
    task.call(0)?;
    Ok(())
  }  

  pub fn exec(_: &Lua, args: Variadic<String>) -> LuaResult<()> {
    let mut args = args.into_iter().map(|s| s.clone()).collect::<Vec<String>>();
    let mut cmd = args.remove(0);
  
    let mut muted = false;
    if cmd.starts_with("@") {
      muted = true;

      cmd = cmd.replace("@", "");
    }

    if cmd.contains(" ") {
      let mut parts:Vec<String> = cmd.split(" ").map(|s| s.to_string()).collect();
      cmd = parts.remove(0);
  
      parts.append(&mut args);
      
      args = parts;
    }
  
    let result = Command::new(cmd)
      .args(args)
      .output()
      .expect("failed to execute process");
  
    if !muted {
      stdout().write_all(&result.stdout).unwrap();
      stderr().write_all(&result.stderr).unwrap();
    }
  
    Ok(())
  }
  

  fn function_names(source:String) -> Vec<String> {
    let mut names:Vec<String> = vec![];
    let pattern = regex::Regex::new(r"function[\t|\s]+(?P<fname>\w+)\s*\(\).*").unwrap();

    for line in source.lines().into_iter() {
      match pattern.captures(line) {
        Some(m) => {
          match m.name("fname").map(|m| m.as_str()) {
            Some(n) => names.push(n.to_string()),
            None => {},
          };

        },
        None => {},
      }
    }

    names
  }

  fn translate(raw:String) -> (String, EngineContext) {
    let raw_source = &raw.clone();
    let mut parser = parser::Parser::new(raw_source);
    let source = parser.run().unwrap();

    println!("{}", source);

    (
      source,
      EngineContext {
        functions: parser.functions.clone(),
        tasks: parser.tasks.clone(),
        watches: parser.watches.clone(),
        exports: parser.exports.clone(),
        assignments: parser.assignments.clone(),
      }
    )
  }
  
}
