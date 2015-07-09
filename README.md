<h1 align="center">
	<img height="320" src="https://raw.github.com/michaeldoaty/chalk/master/chalk.jpg" alt="chalk">
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
