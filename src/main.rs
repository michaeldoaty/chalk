extern crate chalk;

use chalk::colors::*;
use chalk::Chalk;

fn main() {
    let red_bold = Chalk::chalk(Colors::Red, "Hello World")
        .underline()
        .bg(Colors::Blue)
        .bold()
        .color();

    println!("{}", red_bold);

}
