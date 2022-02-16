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

pub struct Engine {
  lua: Lua,
  source:String,
  pub tasks:HashMap<String, String>,
}

impl Engine {
  pub fn new(source:String) -> crate::Result<Engine> {
    let (src, registry) = Self::translate(source);

    let this = Engine {
      lua: Lua::new(),
      source: src,
      tasks: registry,
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
    let keys = self.tasks.clone().into_iter().map(|(k, _)| k).collect::<Vec<String>>();

    return keys.contains(&name);
  }

  pub fn list(&self) {
    println!();

    println!("{}\n", "Motive Tasks:".bold());

    for (k, v) in &self.tasks {
      let mut out:Vec<String> = vec![format!("{}", k.bold())];
        
      if v.len() > 0 {
        out.push(format!(" - {}", v));
      }

      console_info!("{}", out.join("\t"));
    }

    println!();
  }

  pub async fn run(&self, task:String) -> LuaResult<()> {
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

  fn translate(raw:String) -> (String, HashMap<String, String>) {
    let function_names = Self::function_names(raw.clone());
    let keywords = IGNORED_KEYWORDS.to_vec();
    let mut registry = HashMap::new();

    let mut source:Vec<String> = vec![string!("motive.tasks = {}\n")];


    for (index, line) in raw.lines().into_iter().enumerate() {
      if line.starts_with("task") {
        let mut current = string!(line);

        let mut description = string!("");
        let parts = line.split("--").collect::<Vec<&str>>();
        
        if line.contains("--") {
          current = string!(parts[0]);
          description = string!(parts[1]);
        }

        let name = current
        .replace("task", "")
        .trim()
        .to_string();

        registry.insert(name.clone(), description.clone());

        source.push(
          format!("motive.tasks.{} = function()\n", name)
        );

        continue;
      }

      if line.starts_with("--") || line.is_empty() || index < 1 {
        source.push(string!(line));

        continue;
      }
      

      let chunks:Vec<String> = line.trim().split(" ").into_iter().map(|s| s.to_string()).collect();
      if let Some(word) = chunks.first() {
        if keywords.iter().filter(|&k| word.contains(k)).count() > 0 {
          source.push(string!(line));
          continue;
        }
      }

      let pattern = regex::Regex::new(r"[\t|\s]+(?P<fname>\w+)\s*\(\).*").unwrap();
      for name in &function_names {
        if let Some(m) = pattern.captures(line.trim()) {
          if m.len() > 0 {
            println!("F: {}", name);
            source.push(string!(line));
  
            break;  
          } 
        }
      }

      source.push(format!("  exec(\"{}\")", line.trim().replace("\"", "")));
    }

    let source = source.join("\n");

    (format!(r#"{}"#, source).to_string(), registry)
  }
  
}
