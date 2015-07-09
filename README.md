<h1 align="center">
	<a href="https://flic.kr/p/9WsQ5C" target="_blank">
	  <img height="320" src="https://raw.github.com/michaeldoaty/chalk/master/chalk.jpg" alt="chalk">
	<a/>
</h1>
# Chalk
Chalk is a terminal styling library for rust.

```rust
extern crate chalk;

use chalk::colors::Colors::*;
use chalk::Chalk;

fn main() {
    let red_blue = Chalk::new(Red, "Hello World")
        .underline()
        .bg(Blue)
        .bold()
        .color();

    println!("{}", red_blue);
}
```
