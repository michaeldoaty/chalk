use std::fmt::Display;

pub fn format_output<T: Display>(code: i32, data: T) -> String {
    format!("\x1B[{}m{}\x1B[0m", code, data)
}
