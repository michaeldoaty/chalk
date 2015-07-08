extern crate chalk;

use chalk::colors::Colors::*;
use chalk::Chalk;

fn main() {
    let red_bold = Chalk::chalk(Blue, "Hello World")
        .underline()
        .bg(White)
        .dim()
        .bold()
        .color();

    println!("{}", red_bold);

}
