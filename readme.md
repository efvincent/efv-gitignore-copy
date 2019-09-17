# gitignore-copy

You know when you want to copy an entire subfolder of code from one computer
to another, say when you've got a new computer and you're retiring your old one?

You know how you don't want to copy all the built targets and npm module folders?

This utility does that for you, the hard way. Really just an excuse to 
build some more stuff in Rust.

## What it currently does
Call it with a path (or file, but more interesting with a path)
```
$ gitignore-copy ~/code
```
and it will dump out a list of everything it finds at and below that point. If there's a `.gitignore` it will report it and use it in that directory and in any directory below that directory that doesn't have it's own `.gitignore`. 

This way if you want you can stick a top level `.gitignore` that specifies things you know you don't want in any subdir like `npm`, `target`,  and `build` for example, even if the top level is not a repo itself. 

##  Coming soon
you know... actually copy the files to a target directory, recreating the tree with out the ignored stuff.