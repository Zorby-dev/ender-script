use std::fmt::Display;

#[derive(Clone, Copy)]
#[allow(unused)]
pub enum Color {
    Reset,
    Red,
    Yellow,
    LightYellow,
    Grey,
    Black,
    Magenta,
    Green,
    Blue,
    LightBlue
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::Reset => write!(f, "{}", Color::reset()),
            Color::Red => write!(f, "{}", Color::red()),
            Color::Yellow => write!(f, "{}", Color::yellow()),
            Color::LightYellow => write!(f, "{}", Color::light_yellow()),
            Color::Grey => write!(f, "{}", Color::grey()),
            Color::Black => write!(f, "{}", Color::black()),
            Color::Magenta => write!(f, "{}", Color::magenta()),
            Color::Green => write!(f, "{}", Color::green()),
            Color::Blue => write!(f, "{}", Color::blue()),
            Color::LightBlue => write!(f, "{}", Color::light_blue())
        }
    }
}

impl Color {
    pub fn reset() -> &'static str { "\u{001b}[0m" }

    pub fn red() -> &'static str { "\u{001b}[38;5;197m" }

    pub fn yellow() -> &'static str { "\u{001b}[38;5;220m" }

    pub fn light_yellow() -> &'static str { "\u{001b}[38;5;11m" }

    pub fn grey() -> &'static str { "\u{001b}[90m" }

    pub fn black() -> &'static str { "\u{001b}[30m" }

    pub fn magenta() -> &'static str { "\u{001b}[35m" }

    pub fn green() -> &'static str { "\u{001b}[38;5;34m" }

    pub fn blue() -> &'static str { "\u{001b}[38;5;26m" }

    pub fn light_blue() -> &'static str { "\u{001b}[38;5;45m" }
}

pub fn colored(color: Color, text: &str) -> String {
    return color.to_string() + text + Color::reset();
}