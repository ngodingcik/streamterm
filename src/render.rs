use crate::consts::*;
use image::RgbaImage;
use rayon::prelude::*;
use std::fmt::Write;

pub fn render_frame(img: &RgbaImage, term_w: u16, term_h: u16) -> String {
    let target_w = (term_w as u32) * 2;
    let target_h = (term_h as u32) * 4;

    if target_w == 0 || target_h == 0 {
        return String::new();
    }

    let orig_w = img.width();
    let orig_h = img.height();
    let raw = img.as_raw();

    let mut map_x = vec![0_u32; target_w as usize];
    for x in 0..target_w {
        map_x[x as usize] = ((x * orig_w) / target_w).min(orig_w.saturating_sub(1));
    }
    
    let mut map_y = vec![0_u32; target_h as usize];
    for y in 0..target_h {
        map_y[y as usize] = ((y * orig_h) / target_h).min(orig_h.saturating_sub(1));
    }

    let lines: Vec<String> = (0..(term_h as u32))
        .into_par_iter()
        .map(|cy| {
            let mut row = String::with_capacity((term_w as usize) * 20);
            let cy4 = cy * 4;

            let mut px_cache = [(0u32, 0u32, 0u32); 8];
            let mut lums_cache = [0u32; 8];
            
            let mut last_bg = (256, 256, 256);
            let mut last_fg = (256, 256, 256);

            for cx in 0..(term_w as u32) {
                let cx2 = cx * 2;

                for (i, &(dx, dy)) in PX_OFF.iter().enumerate() {
                    let tx = (cx2 + dx as u32).min(target_w - 1);
                    let ty = (cy4 + dy as u32).min(target_h - 1);

                    let src_x = map_x[tx as usize];
                    let src_y = map_y[ty as usize];

                    let idx = ((src_y * orig_w + src_x) * 4) as usize;

                    if idx + 2 < raw.len() {
                        px_cache[i] = (raw[idx] as u32, raw[idx + 1] as u32, raw[idx + 2] as u32);
                    }
                }

                for i in 0..8 {
                    let (r, g, b) = px_cache[i];
                    lums_cache[i] = (r * 2126 + g * 7152 + b * 722) / 10000;
                }

                let avg_l = lums_cache.iter().sum::<u32>() >> 3;

                for i in 0..8 {
                    lums_cache[i] = (lums_cache[i] * 7 + avg_l * SMOOTH_WEIGHT) >> 3;
                }

                let min_l = *lums_cache.iter().min().unwrap_or(&0);
                let max_l = *lums_cache.iter().max().unwrap_or(&0);

                if max_l - min_l < DEADZONE_THRESH {
                    let mut sr = 0;
                    let mut sg = 0;
                    let mut sb = 0;

                    for i in 0..8 {
                        let (r, g, b) = px_cache[i];
                        sr += r;
                        sg += g;
                        sb += b;
                    }

                    let ar = sr >> 3;
                    let ag = sg >> 3;
                    let ab = sb >> 3;

                    if last_bg != (ar, ag, ab) {
                        let _ = write!(row, "\x1b[48;2;{};{};{}m", ar, ag, ab);
                        last_bg = (ar, ag, ab);
                    }
                    if last_fg != (ar, ag, ab) {
                        let _ = write!(row, "\x1b[38;2;{};{};{}m", ar, ag, ab);
                        last_fg = (ar, ag, ab);
                    }
                    row.push(' ');
                    continue;
                }

                let mut bits = 0;
                let mut bs = [0, 0, 0];
                let mut ds = [0, 0, 0];
                let mut bc = 0;
                let mut dc = 0;
                let thresh = avg_l;

                for i in 0..8 {
                    let (r, g, b) = px_cache[i];
                    if lums_cache[i] >= thresh + HYST_MARGIN {
                        bits |= BIT_MAP[i];
                        bs[0] += r;
                        bs[1] += g;
                        bs[2] += b;
                        bc += 1;
                    } else {
                        ds[0] += r;
                        ds[1] += g;
                        ds[2] += b;
                        dc += 1;
                    }
                }

                let (fr, fg, fb) = if bc > 0 {
                    (bs[0] / bc, bs[1] / bc, bs[2] / bc)
                } else {
                    (0, 0, 0)
                };

                let (br, bg, bb) = if dc > 0 {
                    (ds[0] / dc, ds[1] / dc, ds[2] / dc)
                } else {
                    (0, 0, 0)
                };

                if last_bg != (br, bg, bb) {
                    let _ = write!(row, "\x1b[48;2;{};{};{}m", br, bg, bb);
                    last_bg = (br, bg, bb);
                }
                if bits > 0 && last_fg != (fr, fg, fb) {
                    let _ = write!(row, "\x1b[38;2;{};{};{}m", fr, fg, fb);
                    last_fg = (fr, fg, fb);
                }

                let braille = std::char::from_u32(BRAILLE_BASE + bits).unwrap_or(' ');
                row.push(braille);
            }
            row.push_str("\x1b[0m");
            row
        })
        .collect();

    lines.join("\r\n")
}
