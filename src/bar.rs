use std::io::{Result, Write, stdout};

use crossterm::{
    ExecutableCommand, QueueableCommand,
    cursor::{self, MoveToColumn},
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, ClearType},
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

    fn output(&self, progress: f32) -> Result<()> {
        let progress_made = (progress * self.length as f32) as u16;

        let mut stdout = stdout();

        stdout
            .queue(cursor::Hide)?
            .queue(MoveToColumn(0))?
            .queue(SetForegroundColor(self.complete_color))?
            .queue(Print(self.generate_bar(progress_made)))?
            .queue(SetForegroundColor(self.incomplete_color))?
            .queue(Print(self.generate_bar(self.length - progress_made)))?
            .queue(ResetColor)?
            .queue(cursor::Show)?;

        stdout.flush()?;

        Ok(())
    }
}

pub trait OutputBar {
    fn get_bar(&self) -> &ProgressBar;
    fn output(&self, progress: f32) -> Result<()> {
        self.get_bar().output(progress)
    }
}
