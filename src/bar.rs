use std::io::{Result, Write, stdout};

use crossterm::{
    ExecutableCommand, QueueableCommand, cursor,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal,
};

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

    fn generate_bar(&self, len: u16) -> String {
        let mut res = String::new();
        for _ in 0..len {
            res.push(self.bar_char);
        }
        res
    }

    fn output(&self, progress: u16) -> Result<()> {
        let mut stdout = stdout();

        stdout
            .queue(cursor::Hide)?
            .queue(SetForegroundColor(self.complete_color))?
            .queue(Print(self.generate_bar(progress)))?
            .queue(SetForegroundColor(self.incomplete_color))?
            .queue(Print(self.generate_bar(self.length - progress)))?
            .queue(ResetColor)?
            .queue(terminal::Clear(terminal::ClearType::FromCursorDown))?
            .queue(cursor::Show)?;

        stdout.flush()?;

        Ok(())
    }
}

pub trait GetBar {
    fn get_bar(&self) -> &ProgressBar;
}

pub trait OutputBar: GetBar {
    fn output(&self) -> Result<()> {
        self.get_bar().output(5)
    }
}
