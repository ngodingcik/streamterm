use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute, terminal,
};
use std::io::{Write, stdout};

pub struct Terminal {}

impl Terminal {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }

    pub fn size(&self) -> Result<(u16, u16), Box<dyn std::error::Error>> {
        let (cols, rows) = terminal::size()?;
        Ok((cols, rows))
    }

    pub fn hide_cursor(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        execute!(stdout(), Hide)?;
        Ok(())
    }

    pub fn restore(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        execute!(stdout(), Show, MoveTo(0, 0))?;
        // reset SGR
        print!("\x1b[0m\n");
        stdout().flush()?;
        Ok(())
    }

    pub fn print_frame(&self, frame: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut out = stdout();
        execute!(out, MoveTo(0, 0))?;
        write!(out, "{}\x1b[0m", frame)?;
        out.flush()?;
        Ok(())
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        let _ = self.restore();
    }
}
