use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute,
    terminal,
};
use std::io::{BufWriter, Write, stdout};

const BUF_CAP: usize = 1 << 20;

pub struct Terminal {
    out: BufWriter<std::io::Stdout>,
}

impl Terminal {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            out: BufWriter::with_capacity(BUF_CAP, stdout()),
        })
    }

    pub fn size(&self) -> Result<(u16, u16), Box<dyn std::error::Error>> {
        let (cols, rows) = terminal::size()?;
        Ok((cols, rows))
    }

    pub fn hide_cursor(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        execute!(self.out, Hide)?;
        Ok(())
    }

    pub fn print_frame(&mut self, frame: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        execute!(self.out, MoveTo(0, 0))?;
        self.out.write_all(frame)?;
        self.out.write_all(b"\x1b[0m")?;
        self.out.flush()?;
        Ok(())
    }

    fn restore(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        execute!(self.out, Show, MoveTo(0, 0))?;
        self.out.write_all(b"\x1b[0m\n")?;
        self.out.flush()?;
        Ok(())
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        // Ignore errors during cleanup so the process exits cleanly.
        let _ = self.restore();
    }
}
