use crossterm::style::Color;

const DEFAULT_CHAR: char = '▇';
const DEFAULT_LENGTH: u16 = 30;
const DEFAULT_COMPLETE_COLOR: Color = Color::Green;
const DEFAULT_INCOMPLETE_COLOR: Color = Color::Red;

pub struct ProgressBar {
    length: u16,
    bar_char: char,
    complete_color: Color,
    incomplete_color: Color,
}

impl ProgressBar {
    pub const fn default() -> Self {
        Self {
            length: DEFAULT_LENGTH,
            bar_char: DEFAULT_CHAR,
            complete_color: DEFAULT_COMPLETE_COLOR,
            incomplete_color: DEFAULT_INCOMPLETE_COLOR,
        }
    }
}
