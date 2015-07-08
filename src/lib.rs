pub mod colors;
pub mod styles;

use colors::*;
use styles::*;

pub struct Chalk {
    string: String
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

#[test]
fn it_works() {
}
