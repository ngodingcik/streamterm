use scrap::{Capturer, Display};
use std::io::ErrorKind::WouldBlock;
use std::time::Duration;

// Raw screen frame in BGRA byte order (scrap's native layout on every platform).
// the renderer reads R/G/B at offsets +2/+1/+0 instead.
pub struct Frame {
    pub data:   Vec<u8>,
    pub width:  u32,
    pub height: u32,
}

pub struct CaptureStream {
    capturer: Capturer,
    pub width:  u32,
    pub height: u32,
}

impl CaptureStream {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let display = Display::primary()?;
        let width   = display.width()  as u32;
        let height  = display.height() as u32;
        let capturer = Capturer::new(display)?;
        Ok(Self { capturer, width, height })
    }

    pub fn capture(&mut self) -> Result<Frame, Box<dyn std::error::Error>> {
        loop {
            match self.capturer.frame() {
                Ok(frame) => {
                    return Ok(Frame {
                        data:   frame.to_vec(),
                        width:  self.width,
                        height: self.height,
                    });
                }
                Err(e) if e.kind() == WouldBlock => {
                    std::thread::sleep(Duration::from_millis(1));
                }
                Err(e) => return Err(e.into()),
            }
        }
    }
}
