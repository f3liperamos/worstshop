
# Welcome to my Rust Worstshop (definitely not a typo) series

# Preparing to run the code
1. [Install Rust, using rustup](https://www.rust-lang.org/tools/install) or take a look at "Other installation methods
1. Clone this repo
1. [Optional] if you are using vscode, [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) extension is THE extension you want to install. Follow the popups after install

## Basic syntax, conventions and some code

Rust uses `snake_case` convention for writting variables and function names.

Semicolon matters. Forget to put just one of them and you'll have fight with the compiler.

Comments are the same as JS. // for single-line, /* for multiline */ 

```rust
use std::str; // import `str` from `std` package (built-in)
use std::{env, str}; // save lines some lines to import both env and str at once.

fn function_name(param: ParamType) -> ReturnType {
  let how_to_string = String::from("Hello World"); // remember semicolons are important (;
  let how_to_string_v2 = "Hello World".to_string(); // another way to create a String type
  let not_string_but_str = "Hello World!"; // In short, `str` is immutable and have fixed size in memory, `String` is the oposite.

  let mut mutable_string = String::new(); // mutable values NEED to be properly identified
  mutable_string.push_str("He");
  mutable_string.push_str("llo")

  /* 
   * everything that ends with `!` is a macro, there are a few built-ins and you can create your own.
   * I understand them as much as I understand black magic. And I swear I don't understand black magic.
   * /
  println!("{}", mutable_string); // -> Hello
  println!("That's how {} interpolation works {}", "string", "mate!") // -> That's how string interpolation works mate!
}
```

## Trust these links, not me. 
##### Okay, trust me only up to the point I'm recommend these links.
[Rust official book](https://doc.rust-lang.org/book/title-page.html) - One of many books I failed to finish and the reason I'm still not good at Rust

[Rust by example](https://doc.rust-lang.org/rust-by-example/index.html) - Another official material that I wish I had spend the time reading the previous book instead

[Learn rust in half an hour](https://fasterthanli.me/articles/a-half-hour-to-learn-rust) - 51 minutes is not half and hour. Great article anyway.

[Crust of Rust series](https://www.youtube.com/playlist?list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa) - This guy is bonkers.

[Rust cheat sheet](https://www.youtube.com/c/JonGjengset/videos) - Every single language deserve a cheat sheet website (sometimes good for frameworks also)

[[Not Rust] Advent of Code](https://adventofcode.com/2020) - It was the first time I heard about it, it's great to hone skills. Don't drop it like I did.

[Two beautiful Rust programs](https://matklad.github.io//2020/07/15/two-beautiful-programs.html) - Rust at its finest <3

[Why NOT Rust](https://matklad.github.io/2020/09/20/why-not-rust.html) - "Not All Programming is Systems Programming", "In most cases don't NEED ultimate performance"

[Rustlings exercises](https://github.com/rust-lang/rustlings) - Great exercises, check them out.

[Reddit post of a guy who's learning Rust for 3 months](https://www.reddit.com/r/rust/comments/khrt69/first_3_months_of_rust/)  - Inspirational, motivational and point to great learning material, 10/10 gave me depression once I realized how slow I am.

