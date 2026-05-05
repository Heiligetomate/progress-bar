use std::io::{Result, Stdout, Write, stdout};

use crossterm::{
    QueueableCommand,
    cursor::{self, MoveToColumn},
    style::{Color, Print, ResetColor, SetForegroundColor},
};

const DEFAULT_CHAR: char = '▇';
const DEFAULT_LENGTH: u16 = 100;
const DEFAULT_COMPLETE_COLOR: Color = Color::Green;
const DEFAULT_INCOMPLETE_COLOR: Color = Color::Red;
const DEFAULT_TEXT_COLOR: Color = Color::White;

pub struct ProgressBar {
    length: u16,
    bar_char: char,
    complete_color: Color,
    incomplete_color: Color,
    text_color: Color,
}

impl ProgressBar {
    pub const fn default() -> Self {
        Self {
            length: DEFAULT_LENGTH,
            bar_char: DEFAULT_CHAR,
            complete_color: DEFAULT_COMPLETE_COLOR,
            incomplete_color: DEFAULT_INCOMPLETE_COLOR,
            text_color: DEFAULT_TEXT_COLOR,
        }
    }

    fn generate_bar(&self, len: u16) -> String {
        let mut res = String::new();
        for _ in 0..len {
            res.push(self.bar_char);
        }
        res
    }

    fn bar(&self, stdout: &mut Stdout, progress: f32) -> Result<()> {
        let progress_made = (progress * self.length as f32) as u16;
        /// TODO: breaks when it goes over one line
        stdout
            .queue(cursor::Hide)?
            .queue(MoveToColumn(0))?
            .queue(SetForegroundColor(self.complete_color))?
            .queue(Print(self.generate_bar(progress_made)))?
            .queue(SetForegroundColor(self.incomplete_color))?
            .queue(Print(self.generate_bar(self.length - progress_made)))?
            .queue(ResetColor)?
            .queue(cursor::Show)?;
        Ok(())
    }

    fn percentage(&self, stdout: &mut Stdout, progress: f32) -> Result<()> {
        let percentage_text = format!("{}%", (progress * 100 as f32) as u16);
        stdout
            .queue(cursor::Hide)?
            .queue(MoveToColumn(self.length + 2))?
            .queue(SetForegroundColor(self.text_color))?
            .queue(Print(percentage_text))?
            .queue(ResetColor)?
            .queue(cursor::Show)?;
        Ok(())
    }

    fn output(&self, progress: f32) -> Result<()> {
        let progress_made = (progress * self.length as f32) as u16;

        let mut stdout = stdout();
        self.bar(&mut stdout, progress)?;
        self.percentage(&mut stdout, progress)?;
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
