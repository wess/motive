# Motive
A developer environment manager with a nice future. But first let's just make a simple task runner that takes advantage of a _special_ version of Lua.

### Installing
> to do

### Getting started
Once Motive is installed run: `$ motive init` and this will create a *manifest* file in your current directory. Anything in the *manifest* you write is standard lua, the only exception is
defining tasks. Motive uses *task* to identify commands that can be run from the command line. For example:

```lua
-- task identifies what can be run using: $ motive taskname

task taskname do -- this will get printed when running: $ motive list
  print("Hello world")
end

-- This function cannot be called from the command line and only available
-- in the manifest
function hello() 
  print("Hello, world!")
end

-- A task can call a lua function
task funcall do -- Call a lua function
  hello()
end

```

# Contributing
Contributions are welcome, just make a pull request. If you are including new functionality, please write a usage example, and include any information that will impact installation and/or setup.

# License
*Motive* is released under the MIT license. See LICENSE for details.
