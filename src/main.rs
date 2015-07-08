extern crate chalk;

use chalk::colors::Colors::*;
use chalk::Chalk;

fn main() {
    let red_bold = Chalk::chalk(White, "Hello World")
        .underline()
        .bg(Red)
        .fast_blink()
        .bold()
        .color();

    println!("{}", red_bold);

}
