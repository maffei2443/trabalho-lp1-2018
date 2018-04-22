+ module:
    * is a *namespace*
    * contains definitions of functions/type
    * are *private* by *default* (as everything is about its namespace)
    * __pub__ makes an item public (most like *extern* in C)

+ **Rust only knows to look \[for modules\] in src/lib.rs by default.**

\[Taked from: [rust_book, second edition]\]
[rust_book, second edition] : https://doc.rust-lang.org/book/second-edition/ch07-01-mod-and-the-filesystem.html
### Rules of Module Filesystems

Let’s summarize the rules of modules with regard to files:

* If a module named `foo` has no submodules, you should put the declarations
  for `foo` in a file named *foo.rs*.
* If a module named `foo` does have submodules, you should put the declarations
  for `foo` in a file named *foo/mod.rs*.