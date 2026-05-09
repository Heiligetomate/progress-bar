use crate::default_values::*;
use crossterm::{
    QueueableCommand,
    cursor::{self, MoveToColumn},
    style::{Color, Print, ResetColor, SetForegroundColor},
};
use std::io::{Result, Stdout, Write, stdout};

pub struct ProgressBar {
    length: u16,
    bar_char: char,
    complete_color: Color,
    incomplete_color: Color,
    text_color: Color,
    running: bool,
}

pub struct Percentage {
    val: f64,
}

impl Percentage {
    pub fn new(val: f64) -> Result<Self> {
        if val > 1.0 {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "The progress is more than 100%"));
        }
        Ok(Self { val })
    }

    pub fn calc(val: f64, max: f64) -> Result<Self> {
        let res = val / max;
        Self::new(res)
    }
}

impl ProgressBar {
    pub fn new(length: u16, bar_char: char, complete_color: Color, incomplete_color: Color, text_color: Color) -> Self {
        Self {
            length,
            bar_char,
            complete_color,
            incomplete_color,
            text_color,
            running: false,
        }
    }

    pub fn default_with_length(length: u16) -> Self {
        Self::new(length, DEFAULT_CHAR, DEFAULT_COMPLETE_COLOR, DEFAULT_INCOMPLETE_COLOR, DEFAULT_TEXT_COLOR)
    }

    pub const fn default() -> Self {
        Self {
            length: DEFAULT_LENGTH,
            bar_char: DEFAULT_CHAR,
            complete_color: DEFAULT_COMPLETE_COLOR,
            incomplete_color: DEFAULT_INCOMPLETE_COLOR,
            text_color: DEFAULT_TEXT_COLOR,
            running: false,
        }
    }

    fn generate_bar(&self, len: u16) -> String {
        let mut res = String::new();
        for _ in 0..len {
            res.push(self.bar_char);
        }
        res
    }

    fn bar(&self, stdout: &mut Stdout, progress: f64) -> Result<()> {
        let progress_made = (progress * self.length as f64) as u16;
        // TODO: breaks when it goes over one line
        stdout
            .queue(MoveToColumn(0))?
            .queue(SetForegroundColor(self.complete_color))?
            .queue(Print(self.generate_bar(progress_made)))?
            .queue(SetForegroundColor(self.incomplete_color))?
            .queue(Print(self.generate_bar(self.length - progress_made)))?
            .queue(ResetColor)?;
        Ok(())
    }

    fn percentage(&self, stdout: &mut Stdout, progress: f64) -> Result<()> {
        let percentage_text = format!("{}%", (progress * 100 as f64) as u16);
        stdout
            .queue(MoveToColumn(self.length + 2))?
            .queue(SetForegroundColor(self.text_color))?
            .queue(Print(percentage_text))?
            .queue(ResetColor)?;
        Ok(())
    }

    pub fn output(&mut self, percentage: Percentage, display_percentage: bool) -> Result<()> {
        let progress = percentage.val;
        let mut stdout = stdout();

        if !self.running {
            self.running = true;
            stdout.queue(cursor::Hide)?;
        }

        self.bar(&mut stdout, progress)?;

        if display_percentage {
            self.percentage(&mut stdout, progress)?;
        }

        if progress == 1.0 {
            stdout.queue(cursor::Show)?;
        }

        stdout.flush()?;

        if progress == 1.0 {
            println!("");
            self.running = false;
        }

        Ok(())
    }
}

pub trait OutputBar {
    fn get_bar(&mut self) -> &mut ProgressBar;

    fn output(&mut self, progress: Percentage, display_percentage: bool) -> Result<()> {
        self.get_bar()
            .output(progress, display_percentage)
    }
}

pub trait MacroOutputBar {
    fn output(&mut self, display_percentage: bool) -> Result<()>;
}
