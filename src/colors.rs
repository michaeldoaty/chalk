pub enum Colors {
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

pub fn color_code(color: Colors) -> i32 {
    match color {
        Colors::Normal => 0,
        Colors::Black => 30,
        Colors::Red => 31,
        Colors::Green => 32,
        Colors::Yellow => 33,
        Colors::Blue => 34,
        Colors::Magenta => 35,
        Colors::Cyan => 36,
        Colors::White => 37,
    }
}

pub fn bg_color_code(color: Colors) -> i32 {
    color_code(color) + 10
}
