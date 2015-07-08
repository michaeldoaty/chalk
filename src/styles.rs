pub enum Styles {
    Bold,
    Dim,
    Italic,
    Underline,
    SlowBlink,
    FastBlink,
    Invert,
    CrossOut,
}

pub fn style_code(style: Styles) -> i32 {
    match style {
        Styles::Bold => 1,
        Styles::Dim => 2,
        Styles::Italic => 3,
        Styles::Underline => 4,
        Styles::SlowBlink => 5,
        Styles::FastBlink => 6,
        Styles::Invert => 7,
        Styles::CrossOut => 9,
    }
}
