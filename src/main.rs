mod capture;
mod consts;
mod render;
mod term;

use capture::CaptureStream;
use render::render_frame;
use std::time::Duration;
use term::Terminal;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = Terminal::new()?;
    let mut capturer = CaptureStream::new()?;

    terminal.hide_cursor()?;

    let (tx, rx) = std::sync::mpsc::channel();
    ctrlc::set_handler(move || {
        let _ = tx.send(());
    })?;

    loop {
        if rx.try_recv().is_ok() {
            break;
        }

        let frame = capturer.capture()?;
        let (term_w, term_h) = terminal.size()?;

        let output = render_frame(&frame, term_w, term_h);
        terminal.print_frame(&output)?;

        std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
