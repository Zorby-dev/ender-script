pub static RESET: &str = "\x1b[0m";

pub static RED: &str = "\x1b[91m";
pub static YELLOW: &str = "\x1b[93m";
pub static GREY: &str = "\x1b[90m";
pub static BLACK: &str = "\x1b[30m";
pub static MAGENTA: &str = "\x1b[35m";
pub static GREEN: &str = "\x1b[32m";

pub static BG_GREY: &str = "\x1b[100m";

pub fn colored(color: &str, text: String) -> String {
    return color.to_string() + &text + &RESET.to_string();
}