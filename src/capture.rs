use image::{Rgba, RgbaImage};
use scrap::{Capturer, Display};
use std::io::ErrorKind::WouldBlock;
use std::time::Duration;

pub struct CaptureStream {
    capturer: Capturer,
    width: u32,
    height: u32,
}

impl CaptureStream {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let display = Display::primary()?;
        let width = display.width() as u32;
        let height = display.height() as u32;
        let capturer = Capturer::new(display)?;

        Ok(Self {
            capturer,
            width,
            height,
        })
    }

    pub fn capture(&mut self) -> Result<RgbaImage, Box<dyn std::error::Error>> {
        loop {
            match self.capturer.frame() {
                Ok(frame) => {
                    let mut img = RgbaImage::new(self.width, self.height);

                    for (i, pixel) in frame.chunks(4).enumerate() {
                        let x = (i as u32) % self.width;
                        let y = (i as u32) / self.width;
                        let b = pixel[0];
                        let g = pixel[1];
                        let r = pixel[2];
                        let a = pixel[3];
                        img.put_pixel(x, y, Rgba([r, g, b, a]));
                    }

                    return Ok(img);
                }
                Err(error) => {
                    if error.kind() == WouldBlock {
                        std::thread::sleep(Duration::from_millis(16));
                        continue;
                    } else {
                        return Err(error.into());
                    }
                }
            }
        }
    }
}
