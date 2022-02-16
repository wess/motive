# Motive
A developer environment manager with a nice future. But first let's just make a simple task runner that takes advantage of a _special_ version of Lua.

### Installing
> to do

### Getting started
Once Motive is installed run: `$ motive init` and this will create a *manifest* file in your current directory. Anything in the *manifest* you write is standard lua, the only exception is
defining tasks. Motive uses *task* to identify commands that can be run from the command line. For example:

```lua
-- task identifies what can be run using: $ motive taskname

task taskname -- this will get printed when running: $ motive list
  echo "Hello world"
  @ls -la
end

-- This function cannot be called from the command line and only available
-- in the manifest
function hello() 
  print("Hello, world!")
end

-- A task can call a lua function
task funcall -- Call a lua function
  hello()
end

```

Motive's Lua subset has some special magic in it that knows if you are calling Lua or not. You can use
functions and call the functions like normal and if you want to call something from then command line
you just type out the command (from within a Task declaration.). Like:

```lua

task hi
  ls -la
  @ls -la
end

```

The first call to `ls -la` will print the results to the terminal, and by adding `@` in front
of our shell command it will mute the output, ie: `@ls -la`


With your Manifest in place, simply call your task from the command line:

```shell
$ motive funcall
> Hello, world!
$ motive taskname
> Hello world
```

### Breaking changes
- `do` is no longer used when defining a task.

### What's included?
> Everything Lua offers with the addition of a fancy `exec` function that runs shell commands.

### Todo
- Easy ways to install.
- Built-in file watch, change, execute.
- Request client because why not.

# Contributing
Contributions are welcome, just make a pull request. If you are including new functionality, please write a usage example, and include any information that will impact installation and/or setup.

# License
*Motive* is released under the MIT license. See LICENSE for details.
