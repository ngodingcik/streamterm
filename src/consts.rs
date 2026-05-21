// Maps sub-pixel index (0-7) to the corresponding braille dot bit.
// Layout within a 2x4 braille cell:
//   col 0  col 1
//   dot1   dot4   <- row 0
//   dot2   dot5   <- row 1
//   dot3   dot6   <- row 2
//   dot7   dot8   <- row 3
pub const BIT_MAP: [u8; 8] = [0x01, 0x08, 0x02, 0x10, 0x04, 0x20, 0x40, 0x80];

// (dx, dy) offsets within a 2x4 block, ordered to match BIT_MAP indices.
pub const PX_OFF: [(usize, usize); 8] = [
    (0, 0), (1, 0),
    (0, 1), (1, 1),
    (0, 2), (1, 2),
    (0, 3), (1, 3),
];

// Luminance spread below this threshold -> treat the cell as a solid color block.
pub const DEADZONE_THRESH: u32 = 28;

// Pixels must exceed avg_lum + this margin to be assigned to the foreground.
// Reduces flickering on gradients near the threshold boundary.
pub const HYST_MARGIN: u32 = 3;

// Weight (out of 8) pulled toward the cell average during luminance smoothing.
// Higher values blur more, 0 disables smoothing entirely.
pub const SMOOTH_WEIGHT: u32 = 0;
