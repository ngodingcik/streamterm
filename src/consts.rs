pub const BIT_MAP: [u32; 8] = [0x01, 0x08, 0x02, 0x10, 0x04, 0x20, 0x40, 0x80];
pub const BRAILLE_BASE: u32 = 0x2800;

pub const PX_OFF: [(usize, usize); 8] = [
    (0, 0),
    (1, 0),
    (0, 1),
    (1, 1),
    (0, 2),
    (1, 2),
    (0, 3),
    (1, 3),
];

pub const DEADZONE_THRESH: u32 = 28;
pub const HYST_MARGIN: u32 = 3;
pub const SMOOTH_WEIGHT: u32 = 3;
