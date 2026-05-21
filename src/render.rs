use crate::capture::Frame;
use crate::consts::*;
use rayon::prelude::*;

// Encodes U+2800+bits directly into its three-byte UTF-8 representation.
//
// All 256 braille characters live in the range U+2800..=U+28FF, which maps to
// the three-byte UTF-8 form E2 [A0..A3] [80..BF]. The bit math is:
//
//   byte0 = 0xE2                 (top 4 bits of 0x28xx are always 0010)
//   byte1 = 0xA0 | (bits >> 6)   (next 6 bits, only bits 6-7 vary -> 0..3 added)
//   byte2 = 0x80 | (bits & 0x3F) (bottom 6 bits)
#[inline(always)]
fn braille_utf8(bits: u8) -> [u8; 3] {
    [0xE2, 0xA0 | (bits >> 6), 0x80 | (bits & 0x3F)]
}

// Writes a u8 value as decimal ASCII digits.
#[inline(always)]
fn push_decimal(buf: &mut Vec<u8>, n: u8) {
    if n >= 100 {
        buf.push(b'0' + n / 100);
        buf.push(b'0' + (n / 10) % 10);
        buf.push(b'0' + n % 10);
    } else if n >= 10 {
        buf.push(b'0' + n / 10);
        buf.push(b'0' + n % 10);
    } else {
        buf.push(b'0' + n);
    }
}

// Appends an ANSI SGR 24-bit color escape to buf.
// Foreground: ESC[38;2;R;G;Bm   Background: ESC[48;2;R;G;Bm
#[inline(always)]
fn push_color(buf: &mut Vec<u8>, r: u8, g: u8, b: u8, bg: bool) {
    buf.extend_from_slice(b"\x1b[");
    buf.push(if bg { b'4' } else { b'3' });
    buf.extend_from_slice(b"8;2;");
    push_decimal(buf, r);
    buf.push(b';');
    push_decimal(buf, g);
    buf.push(b';');
    push_decimal(buf, b);
    buf.push(b'm');
}

// Converts a raw BGRA screen frame into a terminal-ready byte sequence.
//
// Each terminal cell covers a 2x4 pixel block, rendered as a braille character
// (U+2800-U+28FF) with 24-bit foreground and background colors. Rows are
// processed in parallel via rayon, then joined into a single flat buffer.
pub fn render_frame(frame: &Frame, term_w: u16, term_h: u16) -> Vec<u8> {
    let tw = term_w as u32;
    let th = term_h as u32;
    let target_w = tw * 2;
    let target_h = th * 4;

    if target_w == 0 || target_h == 0 {
        return Vec::new();
    }

    let orig_w  = frame.width;
    let orig_h  = frame.height;
    let raw = &frame.data; // BGRA layout: byte 0=B 1=G 2=R 3=A

    // Maps every target pixel column/row to its
    // source coordinate. Built once per frame, indexing avoids repeated divides
    // inside the inner loop.
    let map_x: Vec<u32> = (0..target_w)
        .map(|x| ((x * orig_w) / target_w).min(orig_w.saturating_sub(1)))
        .collect();
    let map_y: Vec<u32> = (0..target_h)
        .map(|y| ((y * orig_h) / target_h).min(orig_h.saturating_sub(1)))
        .collect();

    // Each rayon task owns a row buffer and writes into it independently.
    let rows: Vec<Vec<u8>> = (0..th)
        .into_par_iter()
        .map(|cy| {
            let mut row: Vec<u8> = Vec::with_capacity(tw as usize * 22);
            let cy4 = (cy * 4) as usize;

            let mut px:   [(u8, u8, u8); 8] = [(0, 0, 0); 8];
            let mut lums: [u32; 8]          = [0; 8];

            let mut last_bg: (u16, u16, u16) = (256, 256, 256);
            let mut last_fg: (u16, u16, u16) = (256, 256, 256);

            for cx in 0..tw {
                let cx2 = (cx * 2) as usize;

                // Sample the 8 sub-pixels that make up this braille cell.
                for (i, &(dx, dy)) in PX_OFF.iter().enumerate() {
                    let tx   = (cx2 + dx).min((target_w - 1) as usize);
                    let ty   = (cy4 + dy).min((target_h - 1) as usize);
                    let sx   = map_x[tx] as usize;
                    let sy   = map_y[ty] as usize;
                    let idx  = (sy * orig_w as usize + sx) * 4;
                    // Read R/G/B from BGRA: R is at +2, G at +1, B at +0.
                    if idx + 2 < raw.len() {
                        px[i] = (raw[idx + 2], raw[idx + 1], raw[idx]);
                    }
                }

                // BT.601 luma approximation scaled by 10000 to stay integer.
                for i in 0..8 {
                    let (r, g, b) = px[i];
                    lums[i] = (r as u32 * 2126 + g as u32 * 7152 + b as u32 * 722) / 10000;
                }

                // Average luma over the cell.
                let avg_l: u32 = lums.iter().sum::<u32>() >> 3;

                for l in &mut lums {
                    *l = (*l * 7 + avg_l * SMOOTH_WEIGHT) >> 3;
                }

                let min_l = lums.iter().copied().min().unwrap_or(0);
                let max_l = lums.iter().copied().max().unwrap_or(0);

                // All sub-pixels are similar enough to collapse into a
                // solid background block. Output a space with matching bg color.
                if max_l - min_l < DEADZONE_THRESH {
                    let (mut sr, mut sg, mut sb) = (0u32, 0u32, 0u32);
                    for (r, g, b) in &px {
                        sr += *r as u32;
                        sg += *g as u32;
                        sb += *b as u32;
                    }
                    let ar = (sr >> 3) as u8;
                    let ag = (sg >> 3) as u8;
                    let ab = (sb >> 3) as u8;
                    let c  = (ar as u16, ag as u16, ab as u16);

                    if last_bg != c {
                        push_color(&mut row, ar, ag, ab, true);
                        last_bg = c;
                    }
                    if last_fg != c {
                        push_color(&mut row, ar, ag, ab, false);
                        last_fg = c;
                    }
                    row.push(b' ');
                    continue;
                }

                // Partition sub-pixels into bright (foreground) and dark (background)
                // groups. Build the braille bit pattern and average each group's color.
                let mut bits: u8      = 0;
                let mut fg_sum        = [0u32; 3];
                let mut bg_sum        = [0u32; 3];
                let mut fg_count: u32 = 0;
                let mut bg_count: u32 = 0;

                for i in 0..8 {
                    let (r, g, b) = px[i];
                    if lums[i] >= avg_l + HYST_MARGIN {
                        bits         |= BIT_MAP[i];
                        fg_sum[0]    += r as u32;
                        fg_sum[1]    += g as u32;
                        fg_sum[2]    += b as u32;
                        fg_count     += 1;
                    } else {
                        bg_sum[0]    += r as u32;
                        bg_sum[1]    += g as u32;
                        bg_sum[2]    += b as u32;
                        bg_count     += 1;
                    }
                }

                let (fr, fg, fb) = if fg_count > 0 {
                    (
                        (fg_sum[0] / fg_count) as u8,
                        (fg_sum[1] / fg_count) as u8,
                        (fg_sum[2] / fg_count) as u8,
                    )
                } else {
                    (0, 0, 0)
                };

                let (br, bg_r, bb) = if bg_count > 0 {
                    (
                        (bg_sum[0] / bg_count) as u8,
                        (bg_sum[1] / bg_count) as u8,
                        (bg_sum[2] / bg_count) as u8,
                    )
                } else {
                    (0, 0, 0)
                };

                // Only emit a new escape when the color actually changes.
                let bg_c = (br as u16, bg_r as u16, bb as u16);
                if last_bg != bg_c {
                    push_color(&mut row, br, bg_r, bb, true);
                    last_bg = bg_c;
                }

                let fg_c = (fr as u16, fg as u16, fb as u16);
                if bits != 0 && last_fg != fg_c {
                    push_color(&mut row, fr, fg, fb, false);
                    last_fg = fg_c;
                }

                row.extend_from_slice(&braille_utf8(bits));
            }

            // Reset all SGR attributes at end of row.
            row.extend_from_slice(b"\x1b[0m");
            row
        })
        .collect();

    let total_len: usize = rows.iter().map(|r| r.len() + 2).sum();
    let mut out: Vec<u8> = Vec::with_capacity(total_len);
    for (i, row) in rows.iter().enumerate() {
        out.extend_from_slice(row);
        if i + 1 < rows.len() {
            out.extend_from_slice(b"\r\n");
        }
    }
    out
}
