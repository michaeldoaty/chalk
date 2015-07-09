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
