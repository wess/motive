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
  
    stdout().write_all(&result.stdout).unwrap();
    stderr().write_all(&result.stderr).unwrap();
  
    Ok(())
  }
  

  fn translate(source:String) -> (String, HashMap<String, String>) {
    let mut name:String = string!("");
    let mut description:String = string!("");
    let mut body:Vec<String> = vec![string!("function()")];
  
    let mut result:Vec<String> = vec![];
    let mut registry:HashMap<String, String> = HashMap::new();
  
    let mut in_task = false;
    for line in source.lines().into_iter() {
      if line.starts_with("task") {
        in_task = true;

        let mut current = string!(line);
        
        if line.contains("--") {
          let parts = line.split("--").collect::<Vec<&str>>();
          current = string!(parts[0]);
          description = string!(parts[1]);
        }
  
        name = current
          .replace("task", "")
          .replace("do", "")
          .trim()
          .to_string();
  

        continue;
      }

      
      if in_task {
        body.push(line.trim().to_string());
  
        if line.ends_with("end") {
          in_task = false;
  
          let task_name = name.clone();
          result.push(format!("motive.tasks.{} = {}", task_name, body.join("\n")));


          registry.insert(name.clone(), description.clone());

          name = string!("");
          description = string!("");
          body = vec![string!("function()")];
        }
    
        continue;
      }
  
      result.push(line.trim().to_string());
    }
  
    (
      result.join("\n"),
      registry
    )
  }
  
}