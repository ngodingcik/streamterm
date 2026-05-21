mod capture;
mod consts;
mod render;
mod term;

use capture::CaptureStream;
use render::render_frame;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use term::Terminal;

// 60 fps = 1000ms / 60 = 16.666ms per frame
const FRAME_BUDGET: Duration = Duration::from_millis(16);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal  = Terminal::new()?;
    let mut capturer  = CaptureStream::new()?;

    terminal.hide_cursor()?;

    let running = Arc::new(AtomicBool::new(true));
    let flag    = running.clone();
    ctrlc::set_handler(move || flag.store(false, Ordering::Relaxed))?;

    while running.load(Ordering::Relaxed) {
        let t0 = Instant::now();

        let frame          = capturer.capture()?;
        let (term_w, term_h) = terminal.size()?;
        let output         = render_frame(&frame, term_w, term_h);
        terminal.print_frame(&output)?;

        // Sleep only the portion of the budget that was not spent working.
        let elapsed = t0.elapsed();
        if elapsed < FRAME_BUDGET {
            std::thread::sleep(FRAME_BUDGET - elapsed);
        }
    }

    Ok(())
}
