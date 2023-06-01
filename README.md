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
                                                                                 
```bash

cargo new 
cagro run 
cargo check
cargo test -- 
cargo build

```

************************************************************************************
An I/O Project:
  I have to combine what I've learned concepts in passed several months;
	Come on!

```rust

	std::env::args;
        //make a Iterator which will produce a series of values, then we  can put them into collections with collect function.

	std::fs; 
	// read_to_string, make the contents into string.
```

**********************************************************************************
********************递归调用产生了栈溢出：stack overflow
********************
**********************************************************************************
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
