extern crate chalk;

struct Chalk {
    string: String
}

enum Colors {
    Normal,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

enum Styles {
    Bold,
    Faint,
    Italic,
    Underline,
    SlowBlink,
    FastBlink,
    Invert,
    CrossOut,
}

fn color_code(color: Colors) -> i32 {
    use Colors::*;

    match color {
        Normal => 0,
        Black => 30,
        Red => 31,
        Green => 32,
        Yellow => 33,
        Blue => 34,
        Magenta => 35,
        Cyan => 36,
        White => 37,
    }
}

fn bg_color_code(color: Colors) -> i32 {
    color_code(color) + 10
}

fn style_code(style: Styles) -> i32 {
    use Styles::*;

    match style {
        Bold => 1,
        Faint => 2,
        Italic => 3,
        Underline => 4,
        SlowBlink => 5,
        FastBlink => 6,
        Invert => 7,
        CrossOut => 9,
    }
}

impl Chalk {
    pub fn chalk<'a>(color: Colors, string: &'a str) -> Chalk {
        let color_code = color_code(color);

        let chalk = Chalk{
            string: format!("\x1B[{}m{}\x1B[0m", color_code, string)
        };

        chalk
    }

    pub fn bg(&mut self, color: Colors) -> &mut Chalk {
        self.string = format!("\x1B[{}m{}\x1B[0m", bg_color_code(color), &self.string);
        self
    }

    pub fn color(&mut self) -> String {
        self.string.to_string()
    }

    pub fn bold(&mut self) -> &mut Chalk {
        self.string = Chalk::style(Styles::Bold, &self.string);
        self
    }

    pub fn italic(&mut self) -> &mut Chalk {
        self.string = Chalk::style(Styles::Italic, &self.string);
        self
    }

    pub fn underline(&mut self) -> &mut Chalk {
        self.string = Chalk::style(Styles::Underline, &self.string);
        self
    }

    pub fn faint(&mut self) -> &mut Chalk {
        self.string = Chalk::style(Styles::Faint, &self.string);
        self
    }

    pub fn slow_blink(&mut self) -> &mut Chalk {
        self.string = Chalk::style(Styles::SlowBlink, &self.string);
        self
    }

    pub fn fast_blink(&mut self) -> &mut Chalk {
        self.string = Chalk::style(Styles::FastBlink, &self.string);
        self
    }

    fn style(style: Styles, string: &String) -> String {
        let style_code = style_code(style);

        format!("\x1B[{}m{}\x1B[0m", style_code, string)
    }
}

fn main() {
    use Colors::*;

    let red_bold = Chalk::chalk(Red, "Hello World")
        .underline()
        .bold()
        .color();

    println!("{}", red_bold);

}
