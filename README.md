 # rustocean 🦀❤️                                                                                   
𝓗𝓮𝓻𝓮 𝓵𝓲𝓮𝓼 𝓸𝓷𝓮 𝔀𝓱𝓸𝓼𝓮 𝓷𝓪𝓶𝓮 𝔀𝓪𝓼 𝔀𝓻𝓲𝓽 𝓲𝓷 𝔀𝓪𝓽𝓮𝓻
 # rustocean 🦀❤️     
```
 _______ _______________________ _          _______         _______ _______ _______ 
(  ____ (  ___  )__   __(  ___  | \        (  ____ \\     /(  ___  |  ___  |  ____ \
| (    \/ (   ) |  ) (  | (   ) | (        | (    \/ )   ( | (   ) | (   ) | (    \/
| (__   | (___) |  | |  | (___) | |        | |     | (___) | (___) | |   | | (_____ 
|  __)  |  ___  |  | |  |  ___  | |        | |     |  ___  |  ___  | |   | (_____  )
| (     | (   ) |  | |  | (   ) | |        | |     | (   ) | (   ) | |   | |     ) |
| )     | )   ( |  | |  | )   ( | (____/\  | (____/\ )   ( | )   ( | (___) /\____) |
|/      |/     \|  )_(  |/     \(_______/  (_______//     \|/     \(_______)_______)
   

```
# Cargo Command 
```bash
Rust's package manager

USAGE:
    cargo [OPTIONS] [SUBCOMMAND]

OPTIONS:
    -V, --version               Print version info and exit
        --list                  List installed commands
        --explain <CODE>        Run `rustc --explain CODE`
    -v, --verbose               Use verbose output (-vv very verbose/build.rs output)
    -q, --quiet                 Do not print cargo log messages
        --color <WHEN>          Coloring: auto, always, never
        --frozen                Require Cargo.lock and cache are up to date
        --locked                Require Cargo.lock is up to date
        --offline               Run without accessing the network
        --config <KEY=VALUE>    Override a configuration value
    -Z <FLAG>                   Unstable (nightly-only) flags to Cargo, see 'cargo -Z help' for
                                details
    -h, --help                  Print help information

Some common cargo commands are (see all commands with --list):
    build, b    Compile the current package
    check, c    Analyze the current package and report errors, but don't build object files
    clean       Remove the target directory
    doc, d      Build this package's and its dependencies' documentation
    new         Create a new cargo package
    init        Create a new cargo package in an existing directory
    add         Add dependencies to a manifest file
    run, r      Run a binary or example of the local package
    test, t     Run the tests
    bench       Run the benchmarks
    update      Update dependencies listed in Cargo.lock
    search      Search registry for crates
    publish     Package and upload this package to the registry
    install     Install a Rust binary. Default location is $HOME/.cargo/bin
    uninstall   Uninstall a Rust binary

See 'cargo help <command>' for more information on a specific command.

```
# Data type
```
int(unsize or size)

float

bool

char
```

# Some questions                                                                                


## stack overflow

### ways solved

1、优化代码，减少递归深度或者函数调用栈深度。尽量使用循环替代递归调用，避免无限递归。

2、增加栈内存大小，可以通过设置环境变量RUST_MIN_STACK来增加栈的大小。例如：

RUST_MIN_STACK=8388608 cargo run
这里将栈的大小设置为8MB。

3、使用堆内存来存储数据，避免使用栈内存。可以使用Rust中的Vec或Box等数据结构来代替栈内存。

4、使用尾递归优化。在Rust中，可以使用关键字#[recursion_limit]来设置递归调用的最大深度。例如：

```rust
#![recursion_limit="10000"]
fn recursive_function(n: i32) {
    if n == 0 {
        return;
    }
    recursive_function(n - 1);
}
```
这里将递归调用的最大深度设置为10000。


# Chapter-12 I/O operation
## some features:
```rust
use std::env::args;
use std::env::var:

```

```bash
cargo run or test -- args(parameters)

set enviroment varieble like:

IGNORE_CASE=1 cargo run
```

## Some steps to refactor command line
**>Split your program into a main.rs and a lib.rs and move your program’s logic to lib.rs.** 

**>As long as your command line parsing logic is small, it can remain in main.rs.**  

**>When the command line parsing logic starts getting complicated, extract it from main.rs and move it to lib.rs.**  

**>Write a test that fails and run it to make sure it fails for the reason you expect.**  

**>Write or modify just enough code to make the new test pass.**  

**>Refactor the code you just added or changed and make sure the tests continue to pass.**

**>Repeat from step 1!** 



# Chapter-13 Closures and Iterators 


1、Closures, a function-like construct you can store in a variable  

2、Iterators, a way of processing a series of elements  

3、How to use closures and iterators to improve the I/O project in Chapter 12  

4、The performance of closures and iterators (Spoiler alert: they’re faster than you might think!)

## Closures: Anonymous function that capture their environment
move keyword will force the closure have the ownership of parameter.  
capture varible from environment  
||   
## Processing a series of Item with Iterator

**trait Iterator**
```
iter、into_iter、iter_mut

.next()

.map()
.filter()
```


# Chapter-14 features of cargo 

## Customizing builds with Release Profiles
```
add [profile.dev/release] into Cargo.toml file
eg:
[profile.dev]
opt-level = 0
[profile.release]
opt-level = 3

```
## Publishing a crate to crate.io
```
login in https://crates.io with github account

cargo login **API key** 
cargo publish
cargo yank 

**the publish is permanent,so you should be careful** 

```
with some metadata like licecse and description  
use three slashes to write a doc comments 
```rust

///

//!

[package]
name = "guessing_game"
version = "0.1.0"
edition = "2021"
description = "A fun game where you guess what number the computer has chosen."
license = "MIT OR Apache-2.0"

[dependencies]


```
## cargo doc and workspace

**Sharing code with Cargo and crates.io is part of what makes the Rust ecosystem useful for many different tasks. Rust’s standard library is small and stable, but crates are easy to share, use, and improve on a timeline different from that of the language. Don’t be shy about sharing code that’s useful to you on crates.io; it’s likely that it will be useful to someone else as well!** 


# Smart pointer

**
Box<T> for allocating values on the heap  
Rc<T>, a reference counting type that enables multiple ownership  
Ref<T> and RefMut<T>, accessed through RefCell<T>, a type that enforces the borrowing rules at runtime instead of compile time  
**


**
Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.  
Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> allows only immutable borrows checked at compile time; RefCell<T> allows immutable or mutable borrows checked at runtime.  
Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.  
**

## Box<T> point data on the heap
```
Box::new();
```
## Deref trait with smart pointer


From &T to &U when T: Deref<Target=U>  
From &mut T to &mut U when T: DerefMut<Target=U>  
From &mut T to &U when T: Deref<Target=U>  

```
Box<T>
Rc<T>
Ref<T>

trait
Deref
Drop
```
**Summary**

This chapter covered how to use smart pointers to make different guarantees and trade-offs from those Rust makes by default with regular references. The Box<T> type has a known size and points to data allocated on the heap. The Rc<T> type keeps track of the number of references to data on the heap so that data can have multiple owners. The RefCell<T> type with its interior mutability gives us a type that we can use when we need an immutable type but need to change an inner value of that type; it also enforces the borrowing rules at runtime instead of at compile time.

Also discussed were the Deref and Drop traits, which enable a lot of the functionality of smart pointers. We explored reference cycles that can cause memory leaks and how to prevent them using Weak<T>.  

# Fearless Concurrency

## Message passing and Mutexes 
Mutexes have a reputation for being difficult to use because you have to remember two rules:  

You must attempt to acquire the lock before using the data.  
When you’re done with the data that the mutex guards, you must unlock the data so other threads can acquire the lock.  



# Object
##use trait

# Pattern 
##match if let while let 


##I am coming up  trust your self
