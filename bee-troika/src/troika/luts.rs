use crate::constants::{COLUMNS, NUM_MAX_ROUNDS, SLICES};
use crate::types::Trit;

pub const ROUND_CONSTANTS: [[Trit; COLUMNS * SLICES]; NUM_MAX_ROUNDS] = [
    [
        2, 2, 2, 2, 1, 2, 0, 1, 0, 1, 1, 0, 2, 0, 1, 0, 1, 1, 0, 0, 1, 2, 1, 1, 1, 0, 0,
        2, 0, 2, 1, 0, 2, 2, 2, 1, 0, 2, 2, 0, 0, 1, 2, 2, 1, 0, 1, 0, 1, 2, 2, 2, 0, 1,
        2, 2, 1, 1, 2, 1, 1, 2, 0, 2, 0, 2, 0, 0, 0, 0, 2, 1, 1, 2, 1, 0, 1, 0, 2, 1, 1,
        0, 0, 2, 2, 2, 2, 0, 1, 1, 2, 1, 2, 2, 0, 1, 2, 2, 2, 0, 1, 0, 2, 2, 0, 2, 1, 1,
        2, 1, 2, 1, 0, 0, 2, 1, 0, 0, 1, 2, 2, 1, 1, 1, 0, 1, 0, 2, 2, 0, 2, 2, 2, 0, 2,
        2, 1, 0, 0, 0, 2, 1, 0, 0, 1, 1, 1, 2, 2, 1, 0, 1, 0, 1, 1, 1, 1, 0, 1, 2, 2, 1,
        0, 1, 0, 2, 0, 1, 2, 0, 1, 2, 2, 2, 2, 1, 0, 0, 0, 0, 2, 1, 0, 2, 1, 1, 2, 0, 2,
        1, 0, 0, 0, 1, 0, 2, 1, 2, 0, 1, 2, 1, 0, 2, 0, 2, 1, 0, 0, 1, 2, 0, 2, 2, 2, 0,
        1, 0, 2, 0, 1, 0, 2, 1, 2, 1, 2, 2, 1, 1, 2, 0, 2, 2, 1, 0, 0, 2, 0, 2, 1, 0, 1,
    ],
    [
        1, 1, 1, 0, 2, 2, 0, 2, 0, 1, 0, 2, 1, 1, 0, 0, 1, 1, 1, 2, 0, 1, 1, 2, 0, 1, 1,
        1, 2, 0, 2, 2, 2, 0, 2, 1, 1, 2, 1, 0, 2, 1, 0, 2, 1, 0, 0, 2, 0, 0, 0, 0, 0, 0,
        1, 1, 0, 0, 1, 0, 0, 0, 1, 1, 0, 2, 0, 0, 0, 2, 1, 1, 0, 1, 2, 0, 1, 2, 1, 1, 1,
        0, 1, 0, 0, 0, 1, 0, 2, 2, 0, 2, 0, 2, 1, 0, 2, 1, 0, 0, 1, 2, 2, 0, 0, 0, 0, 1,
        0, 2, 2, 2, 1, 1, 0, 1, 0, 2, 1, 2, 2, 2, 1, 0, 2, 2, 0, 2, 0, 1, 2, 1, 0, 1, 0,
        0, 1, 1, 0, 1, 2, 2, 2, 0, 0, 1, 0, 0, 1, 2, 1, 1, 1, 2, 0, 0, 0, 2, 1, 0, 2, 1,
        2, 2, 1, 2, 1, 0, 0, 0, 2, 0, 0, 0, 2, 2, 1, 2, 2, 0, 0, 1, 2, 2, 1, 0, 0, 2, 1,
        2, 2, 2, 0, 1, 1, 1, 1, 2, 0, 1, 1, 2, 2, 1, 0, 1, 2, 0, 2, 2, 1, 0, 1, 2, 1, 0,
        1, 0, 1, 1, 2, 1, 1, 2, 2, 2, 1, 0, 2, 0, 0, 0, 1, 1, 2, 1, 0, 2, 0, 1, 1, 1, 2,
    ],
    [
        0, 2, 0, 2, 1, 2, 1, 1, 2, 1, 1, 2, 2, 2, 2, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 1, 0,
        0, 1, 1, 0, 0, 0, 1, 2, 2, 0, 1, 0, 2, 2, 1, 2, 2, 2, 2, 1, 2, 1, 1, 0, 0, 1, 0,
        2, 0, 2, 0, 1, 2, 0, 0, 2, 2, 2, 1, 1, 0, 0, 2, 0, 2, 2, 2, 2, 1, 2, 1, 0, 2, 0,
        2, 0, 2, 0, 2, 2, 0, 2, 2, 1, 2, 1, 2, 0, 0, 0, 0, 1, 0, 2, 1, 1, 2, 1, 0, 1, 0,
        2, 0, 1, 0, 0, 2, 2, 2, 2, 2, 1, 1, 0, 1, 2, 2, 0, 0, 2, 2, 1, 0, 1, 2, 2, 1, 0,
        2, 1, 1, 2, 1, 2, 0, 1, 0, 0, 1, 0, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 2, 0,
        2, 0, 0, 1, 0, 0, 1, 0, 2, 1, 0, 2, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 1, 0, 2, 0, 0,
        1, 2, 0, 1, 1, 2, 2, 0, 2, 2, 0, 0, 2, 2, 1, 2, 0, 0, 0, 1, 0, 2, 1, 0, 1, 2, 1,
        1, 0, 2, 0, 0, 2, 1, 1, 0, 1, 1, 2, 0, 0, 1, 1, 1, 0, 0, 2, 2, 2, 2, 1, 1, 2, 2,
    ],
    [
        1, 2, 2, 0, 2, 2, 0, 1, 0, 0, 0, 2, 0, 0, 0, 2, 1, 0, 2, 2, 0, 0, 1, 2, 1, 0, 0,
        1, 0, 1, 2, 2, 1, 2, 1, 0, 0, 1, 1, 2, 0, 0, 2, 2, 1, 0, 1, 2, 2, 2, 0, 2, 1, 1,
        2, 1, 2, 1, 1, 0, 2, 1, 0, 0, 1, 2, 0, 1, 1, 0, 0, 1, 0, 2, 0, 0, 2, 0, 2, 0, 0,
        2, 2, 0, 0, 0, 2, 1, 0, 0, 2, 0, 1, 1, 2, 1, 0, 1, 1, 0, 1, 2, 2, 0, 2, 2, 0, 0,
        0, 1, 2, 2, 0, 0, 0, 1, 1, 1, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 0, 0, 0,
        0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 0, 0, 0, 2, 2, 0, 1, 1, 2, 2, 2, 2, 2, 0,
        2, 2, 2, 1, 1, 0, 0, 1, 0, 0, 2, 2, 2, 1, 2, 0, 0, 0, 1, 2, 2, 2, 0, 1, 2, 1, 1,
        2, 2, 1, 1, 2, 0, 1, 0, 0, 1, 0, 2, 0, 2, 0, 1, 0, 0, 0, 2, 2, 2, 1, 1, 1, 0, 2,
        2, 2, 2, 2, 2, 2, 2, 1, 1, 2, 0, 0, 0, 0, 0, 2, 2, 0, 2, 2, 1, 0, 0, 2, 2, 0, 0,
    ],
    [
        0, 1, 1, 1, 1, 2, 0, 1, 1, 1, 1, 1, 0, 1, 2, 0, 2, 1, 0, 0, 2, 0, 1, 0, 1, 2, 0,
        1, 1, 0, 1, 1, 1, 1, 0, 0, 2, 0, 0, 0, 1, 0, 2, 2, 1, 0, 0, 1, 1, 0, 2, 1, 1, 1,
        1, 0, 1, 0, 1, 0, 1, 2, 0, 2, 0, 2, 1, 2, 1, 1, 0, 1, 1, 2, 2, 2, 2, 0, 1, 0, 0,
        2, 1, 1, 0, 1, 2, 1, 0, 1, 1, 1, 1, 0, 1, 1, 2, 2, 0, 1, 0, 2, 0, 0, 2, 1, 2, 2,
        1, 2, 2, 0, 0, 1, 2, 0, 0, 0, 0, 2, 1, 2, 2, 0, 2, 1, 0, 2, 1, 2, 0, 2, 0, 2, 0,
        0, 0, 2, 1, 1, 1, 2, 1, 0, 1, 2, 1, 1, 2, 1, 0, 2, 2, 1, 1, 0, 0, 0, 2, 0, 1, 1,
        2, 1, 2, 2, 2, 0, 1, 2, 2, 0, 1, 0, 1, 1, 2, 0, 2, 2, 2, 1, 1, 0, 2, 2, 1, 0, 2,
        2, 2, 1, 1, 1, 1, 1, 0, 1, 2, 2, 2, 2, 0, 0, 2, 0, 1, 2, 1, 0, 1, 1, 0, 0, 1, 0,
        1, 2, 2, 0, 0, 2, 0, 0, 1, 1, 2, 2, 1, 0, 0, 0, 2, 1, 2, 1, 0, 1, 1, 2, 2, 1, 2,
    ],
    [
        2, 2, 2, 0, 2, 1, 0, 0, 0, 1, 0, 1, 0, 2, 0, 1, 2, 1, 0, 1, 2, 2, 2, 1, 0, 1, 2,
        2, 1, 2, 1, 2, 1, 2, 1, 2, 0, 0, 2, 1, 2, 1, 2, 1, 1, 2, 0, 1, 2, 2, 1, 2, 0, 0,
        2, 0, 0, 2, 0, 0, 1, 2, 0, 0, 0, 0, 0, 0, 2, 2, 0, 2, 1, 0, 0, 0, 2, 2, 0, 0, 2,
        0, 1, 2, 2, 2, 0, 1, 0, 0, 1, 0, 2, 1, 1, 2, 1, 0, 0, 0, 2, 0, 1, 0, 0, 2, 1, 2,
        2, 0, 1, 1, 0, 1, 1, 2, 0, 2, 2, 2, 0, 0, 0, 2, 2, 1, 0, 2, 1, 1, 1, 2, 2, 1, 1,
        0, 0, 1, 2, 1, 1, 0, 2, 1, 2, 0, 2, 1, 0, 1, 1, 0, 2, 1, 1, 2, 0, 2, 0, 0, 1, 0,
        1, 0, 2, 1, 1, 0, 2, 0, 1, 2, 2, 0, 1, 1, 1, 2, 1, 0, 2, 2, 2, 2, 1, 1, 0, 2, 0,
        1, 2, 0, 2, 2, 1, 1, 2, 1, 0, 0, 1, 0, 1, 2, 0, 0, 2, 1, 0, 0, 1, 1, 0, 2, 0, 1,
        0, 1, 0, 1, 0, 1, 2, 1, 1, 1, 2, 1, 2, 1, 1, 1, 0, 2, 1, 0, 1, 2, 0, 2, 2, 1, 0,
    ],
    [
        0, 2, 1, 0, 1, 0, 1, 1, 1, 1, 1, 0, 2, 2, 1, 0, 1, 0, 0, 2, 1, 1, 1, 1, 2, 2, 0,
        1, 1, 1, 2, 0, 1, 1, 2, 2, 2, 1, 1, 2, 0, 2, 2, 1, 1, 2, 2, 0, 2, 1, 0, 1, 2, 0,
        1, 2, 0, 2, 0, 2, 1, 0, 0, 0, 0, 1, 1, 2, 2, 0, 1, 2, 0, 1, 1, 2, 1, 2, 2, 0, 0,
        0, 2, 2, 0, 1, 0, 2, 1, 1, 2, 2, 0, 2, 1, 2, 0, 1, 0, 1, 2, 0, 2, 2, 1, 0, 1, 1,
        1, 0, 1, 0, 1, 1, 2, 0, 1, 2, 0, 2, 1, 0, 2, 2, 0, 0, 0, 1, 2, 0, 0, 1, 0, 1, 1,
        1, 2, 0, 2, 2, 0, 1, 0, 1, 1, 2, 1, 0, 0, 2, 1, 1, 0, 2, 0, 2, 1, 1, 1, 1, 1, 1,
        2, 2, 2, 1, 2, 0, 0, 0, 1, 1, 1, 2, 0, 1, 2, 1, 1, 1, 1, 1, 2, 0, 0, 1, 0, 2, 0,
        0, 1, 2, 2, 2, 0, 2, 2, 0, 2, 2, 2, 1, 1, 0, 0, 0, 0, 0, 2, 2, 2, 1, 2, 2, 0, 0,
        2, 2, 2, 2, 0, 0, 2, 1, 0, 2, 2, 0, 1, 1, 0, 1, 0, 0, 1, 0, 2, 2, 0, 0, 2, 0, 0,
    ],
    [
        0, 2, 1, 0, 1, 0, 0, 0, 1, 2, 1, 0, 2, 2, 0, 2, 1, 2, 1, 2, 0, 1, 0, 0, 2, 2, 2,
        1, 1, 0, 1, 0, 1, 2, 2, 2, 2, 1, 0, 2, 1, 1, 2, 1, 0, 2, 1, 0, 0, 1, 0, 0, 2, 0,
        0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 2, 1, 0, 0, 0, 1, 0, 2, 1, 1, 0, 1, 2, 1,
        0, 2, 0, 1, 1, 0, 1, 1, 2, 0, 2, 1, 2, 0, 0, 0, 2, 2, 1, 2, 2, 1, 2, 1, 2, 2, 1,
        0, 0, 0, 0, 2, 1, 0, 0, 1, 1, 2, 0, 2, 1, 0, 1, 0, 1, 2, 2, 1, 2, 0, 2, 2, 1, 1,
        2, 0, 0, 1, 1, 0, 1, 2, 0, 2, 2, 2, 1, 0, 0, 1, 0, 1, 0, 2, 2, 1, 1, 0, 0, 1, 2,
        2, 1, 1, 2, 1, 2, 0, 2, 2, 0, 2, 0, 0, 1, 1, 1, 0, 0, 0, 1, 0, 2, 1, 1, 2, 2, 2,
        1, 0, 2, 0, 1, 0, 1, 1, 2, 1, 0, 2, 1, 1, 1, 0, 2, 0, 2, 0, 0, 1, 2, 2, 1, 2, 2,
        1, 0, 2, 2, 2, 0, 0, 0, 0, 1, 0, 1, 2, 1, 1, 1, 0, 1, 0, 1, 1, 1, 0, 2, 2, 0, 2,
    ],
    [
        1, 0, 1, 2, 1, 1, 0, 0, 2, 0, 2, 1, 1, 0, 1, 2, 1, 0, 2, 2, 1, 1, 0, 1, 1, 2, 0,
        1, 1, 2, 1, 0, 0, 2, 2, 0, 2, 2, 0, 2, 1, 1, 2, 0, 0, 0, 0, 0, 2, 1, 0, 2, 2, 1,
        0, 0, 2, 1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 0, 0, 0, 2, 0, 2, 1, 0, 0, 2,
        2, 2, 0, 2, 2, 0, 1, 1, 2, 2, 1, 0, 0, 0, 2, 2, 2, 1, 0, 1, 1, 2, 2, 2, 2, 2, 1,
        2, 0, 2, 1, 1, 0, 0, 2, 0, 1, 1, 2, 1, 1, 2, 1, 0, 1, 2, 2, 0, 0, 0, 0, 2, 2, 1,
        2, 2, 1, 1, 0, 2, 2, 1, 0, 0, 0, 2, 1, 1, 1, 1, 1, 1, 2, 2, 1, 1, 2, 0, 0, 0, 1,
        1, 0, 2, 0, 2, 2, 1, 1, 1, 0, 1, 2, 2, 0, 1, 2, 2, 2, 0, 1, 2, 2, 2, 0, 2, 1, 1,
        2, 0, 2, 1, 1, 0, 2, 1, 0, 2, 1, 2, 1, 1, 1, 0, 0, 0, 0, 2, 2, 0, 2, 2, 2, 2, 0,
        2, 2, 0, 0, 0, 2, 0, 1, 0, 0, 0, 1, 1, 2, 0, 1, 1, 0, 2, 1, 1, 2, 2, 0, 2, 0, 1,
    ],
    [
        0, 1, 0, 1, 2, 0, 1, 1, 1, 1, 2, 1, 1, 0, 0, 2, 1, 0, 1, 0, 0, 1, 2, 1, 1, 0, 2,
        2, 0, 0, 2, 1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 2, 0, 1, 2, 0, 2, 1, 2, 2, 2, 1, 0, 0,
        1, 2, 2, 0, 1, 2, 1, 1, 0, 2, 2, 2, 2, 0, 1, 0, 1, 1, 1, 2, 0, 1, 2, 1, 1, 0, 1,
        1, 2, 0, 0, 1, 0, 1, 0, 0, 2, 2, 2, 2, 0, 1, 2, 0, 1, 2, 2, 0, 1, 2, 0, 0, 0, 0,
        2, 2, 2, 0, 0, 2, 1, 0, 2, 2, 2, 1, 1, 0, 1, 0, 0, 1, 2, 2, 2, 1, 0, 2, 0, 0, 2,
        2, 1, 2, 1, 0, 2, 0, 0, 2, 1, 0, 2, 2, 0, 2, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 2, 0,
        0, 0, 0, 0, 0, 0, 2, 2, 0, 1, 0, 0, 0, 0, 2, 2, 0, 2, 1, 0, 2, 0, 2, 2, 0, 0, 2,
        0, 0, 2, 2, 0, 0, 1, 0, 0, 0, 0, 2, 0, 1, 2, 0, 0, 2, 0, 2, 0, 1, 0, 0, 2, 0, 0,
        2, 1, 1, 1, 0, 1, 0, 0, 0, 1, 1, 2, 2, 0, 2, 0, 2, 1, 1, 2, 1, 2, 0, 1, 2, 2, 1,
    ],
    [
        0, 0, 1, 1, 0, 0, 2, 0, 1, 1, 0, 1, 0, 2, 1, 0, 1, 2, 0, 0, 2, 2, 0, 0, 2, 1, 0,
        2, 0, 2, 0, 1, 0, 1, 0, 0, 2, 2, 1, 1, 1, 1, 2, 0, 1, 2, 1, 2, 2, 0, 1, 2, 0, 0,
        1, 1, 0, 2, 2, 0, 0, 2, 2, 1, 0, 1, 1, 0, 1, 0, 2, 1, 1, 2, 0, 0, 0, 2, 2, 0, 1,
        0, 2, 2, 1, 2, 2, 0, 2, 1, 2, 1, 1, 0, 0, 2, 0, 2, 2, 2, 0, 1, 2, 1, 0, 2, 0, 2,
        1, 2, 0, 1, 2, 0, 2, 2, 2, 2, 1, 0, 0, 0, 1, 0, 2, 0, 2, 1, 1, 2, 1, 0, 2, 2, 2,
        2, 1, 0, 0, 2, 0, 1, 2, 0, 2, 1, 1, 1, 0, 1, 0, 0, 1, 2, 1, 2, 2, 0, 2, 0, 0, 2,
        1, 1, 0, 2, 0, 1, 0, 0, 1, 1, 1, 1, 2, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1, 2, 0, 1,
        0, 0, 2, 0, 0, 2, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 2, 0, 1, 2, 0, 2, 0, 0, 2, 0, 1,
        0, 0, 1, 1, 0, 0, 0, 1, 1, 0, 0, 1, 0, 2, 2, 1, 1, 0, 2, 0, 0, 2, 1, 1, 2, 1, 1,
    ],
    [
        2, 0, 0, 1, 1, 0, 0, 0, 0, 2, 2, 2, 1, 0, 2, 2, 0, 2, 2, 2, 2, 1, 0, 1, 0, 0, 0,
        2, 0, 2, 1, 2, 2, 0, 2, 2, 0, 2, 2, 2, 0, 2, 0, 0, 0, 0, 0, 2, 1, 0, 1, 0, 1, 0,
        0, 2, 1, 0, 2, 2, 1, 2, 0, 1, 1, 0, 0, 1, 1, 0, 1, 0, 2, 0, 2, 0, 1, 0, 0, 2, 2,
        2, 2, 1, 1, 1, 0, 1, 2, 2, 0, 2, 2, 2, 2, 0, 1, 2, 2, 0, 0, 2, 0, 1, 2, 0, 2, 2,
        1, 0, 0, 1, 0, 0, 1, 0, 1, 1, 1, 0, 0, 0, 0, 0, 1, 0, 1, 2, 2, 2, 0, 0, 1, 0, 1,
        1, 2, 1, 1, 1, 2, 0, 1, 0, 2, 0, 0, 2, 1, 2, 0, 1, 2, 2, 0, 0, 1, 2, 1, 0, 0, 2,
        2, 1, 2, 2, 1, 2, 1, 1, 2, 1, 0, 0, 0, 0, 2, 0, 0, 0, 2, 1, 2, 0, 2, 0, 0, 1, 2,
        1, 2, 1, 1, 1, 0, 2, 2, 1, 1, 2, 0, 2, 2, 1, 1, 1, 2, 0, 2, 1, 0, 1, 2, 2, 1, 2,
        1, 2, 0, 2, 1, 2, 0, 0, 2, 1, 1, 1, 2, 2, 1, 2, 0, 1, 1, 2, 1, 1, 0, 0, 1, 0, 2,
    ],
    [
        2, 0, 0, 1, 2, 0, 0, 2, 1, 0, 1, 2, 2, 0, 2, 0, 1, 0, 2, 1, 2, 2, 0, 1, 1, 1, 2,
        0, 2, 0, 2, 2, 2, 1, 1, 2, 1, 1, 2, 0, 2, 2, 2, 0, 0, 0, 0, 2, 1, 0, 2, 1, 1, 1,
        0, 2, 1, 0, 0, 0, 1, 2, 2, 1, 0, 0, 1, 2, 1, 2, 2, 0, 1, 1, 0, 2, 1, 1, 0, 2, 2,
        2, 0, 1, 0, 1, 1, 1, 1, 2, 1, 2, 1, 1, 0, 1, 0, 1, 0, 1, 2, 0, 1, 0, 2, 1, 2, 1,
        1, 0, 0, 1, 2, 0, 2, 2, 0, 1, 2, 0, 2, 0, 1, 0, 0, 2, 0, 0, 1, 1, 1, 1, 0, 1, 0,
        0, 2, 1, 1, 0, 2, 0, 2, 0, 1, 1, 1, 1, 1, 2, 2, 1, 1, 2, 1, 0, 0, 1, 1, 0, 2, 0,
        0, 2, 1, 0, 1, 0, 1, 2, 0, 0, 1, 0, 2, 2, 1, 1, 0, 2, 2, 0, 2, 1, 1, 2, 1, 1, 1,
        0, 0, 2, 1, 0, 0, 0, 2, 2, 2, 1, 1, 0, 1, 2, 2, 2, 2, 2, 2, 1, 0, 1, 2, 1, 0, 0,
        0, 2, 1, 2, 1, 1, 2, 1, 2, 2, 1, 2, 2, 0, 0, 0, 1, 0, 0, 0, 0, 2, 1, 1, 1, 0, 0,
    ],
    [
        2, 0, 2, 1, 1, 2, 2, 2, 1, 0, 2, 2, 1, 0, 1, 1, 2, 1, 0, 1, 1, 1, 2, 0, 2, 0, 2,
        2, 0, 1, 1, 2, 1, 1, 2, 0, 0, 2, 2, 2, 0, 0, 0, 2, 2, 0, 2, 2, 1, 1, 1, 2, 2, 0,
        0, 0, 1, 2, 2, 1, 1, 2, 1, 1, 1, 2, 2, 0, 2, 0, 0, 0, 2, 1, 1, 2, 0, 1, 0, 1, 2,
        1, 1, 0, 2, 0, 1, 1, 1, 1, 0, 1, 1, 2, 1, 2, 1, 0, 2, 0, 0, 2, 0, 1, 2, 2, 0, 2,
        0, 0, 0, 1, 0, 2, 2, 0, 1, 0, 1, 1, 0, 2, 1, 0, 2, 1, 1, 0, 0, 1, 0, 0, 0, 0, 1,
        1, 2, 0, 0, 0, 2, 0, 1, 1, 2, 2, 2, 1, 2, 0, 1, 2, 2, 1, 1, 2, 0, 1, 0, 0, 2, 0,
        2, 0, 2, 0, 1, 0, 1, 0, 2, 1, 2, 1, 1, 1, 1, 2, 2, 0, 2, 2, 0, 2, 0, 1, 1, 2, 0,
        0, 0, 0, 1, 1, 2, 2, 2, 2, 1, 0, 1, 1, 2, 1, 1, 0, 2, 1, 2, 0, 2, 0, 0, 1, 1, 0,
        2, 1, 1, 1, 0, 2, 1, 0, 1, 0, 1, 2, 2, 1, 0, 0, 2, 2, 1, 1, 2, 0, 1, 1, 1, 2, 1,
    ],
    [
        2, 0, 2, 0, 2, 1, 1, 0, 1, 1, 1, 1, 2, 2, 1, 1, 0, 0, 1, 0, 1, 1, 0, 2, 1, 2, 0,
        0, 1, 0, 0, 1, 0, 2, 1, 2, 2, 0, 0, 0, 0, 2, 0, 2, 0, 2, 1, 1, 0, 2, 0, 2, 1, 2,
        2, 1, 1, 1, 2, 2, 2, 2, 0, 0, 2, 2, 1, 1, 1, 0, 1, 1, 0, 2, 1, 2, 2, 2, 0, 0, 0,
        1, 0, 2, 0, 1, 1, 1, 1, 1, 0, 2, 2, 1, 2, 1, 0, 0, 2, 1, 1, 1, 0, 2, 2, 1, 1, 1,
        1, 2, 2, 1, 1, 1, 2, 2, 0, 1, 1, 0, 2, 2, 1, 1, 2, 2, 2, 0, 1, 1, 1, 2, 0, 1, 1,
        1, 2, 2, 1, 1, 2, 0, 2, 1, 1, 1, 0, 2, 0, 2, 1, 2, 1, 2, 2, 1, 2, 2, 2, 2, 2, 1,
        0, 0, 0, 0, 1, 0, 0, 2, 1, 1, 2, 0, 1, 0, 0, 1, 1, 1, 0, 2, 0, 1, 0, 0, 1, 1, 2,
        1, 2, 1, 1, 0, 0, 1, 2, 0, 1, 2, 0, 1, 2, 0, 2, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1,
        0, 0, 0, 0, 1, 1, 0, 0, 2, 0, 2, 0, 1, 1, 0, 2, 1, 0, 2, 1, 1, 2, 0, 1, 0, 0, 0,
    ],
    [
        0, 1, 0, 2, 0, 1, 0, 2, 0, 1, 0, 2, 2, 1, 1, 2, 2, 1, 1, 2, 1, 1, 2, 0, 1, 0, 2,
        0, 0, 0, 0, 2, 0, 1, 2, 2, 0, 1, 0, 2, 0, 1, 0, 2, 2, 2, 1, 2, 2, 1, 1, 2, 1, 2,
        2, 0, 0, 0, 2, 0, 0, 1, 0, 2, 1, 1, 2, 0, 0, 2, 0, 2, 0, 1, 0, 2, 2, 0, 0, 2, 1,
        1, 1, 2, 1, 0, 1, 0, 1, 1, 2, 1, 0, 2, 2, 2, 1, 0, 2, 0, 2, 0, 1, 2, 2, 1, 0, 2,
        2, 1, 1, 0, 2, 0, 1, 0, 1, 1, 2, 1, 1, 2, 1, 1, 1, 0, 2, 0, 0, 0, 0, 0, 2, 2, 1,
        2, 0, 1, 0, 0, 2, 2, 1, 0, 1, 0, 1, 0, 1, 2, 1, 1, 2, 2, 1, 2, 1, 1, 1, 0, 0, 1,
        0, 0, 2, 0, 2, 2, 2, 0, 0, 0, 1, 0, 2, 0, 2, 1, 1, 1, 1, 0, 2, 2, 2, 2, 1, 2, 0,
        2, 1, 1, 2, 0, 2, 0, 1, 1, 2, 1, 0, 2, 1, 1, 1, 2, 2, 0, 2, 0, 0, 1, 2, 1, 1, 2,
        0, 1, 0, 2, 2, 1, 0, 0, 2, 0, 1, 2, 1, 1, 1, 1, 1, 0, 1, 0, 1, 0, 2, 0, 0, 2, 0,
    ],
    [
        2, 1, 2, 2, 2, 0, 0, 0, 2, 2, 2, 0, 1, 1, 1, 1, 2, 2, 2, 1, 2, 2, 1, 0, 1, 1, 1,
        2, 0, 0, 0, 1, 2, 0, 1, 1, 2, 2, 1, 1, 2, 0, 0, 2, 2, 1, 0, 2, 0, 2, 2, 0, 2, 1,
        1, 0, 2, 2, 0, 0, 0, 2, 1, 1, 1, 1, 0, 1, 1, 2, 1, 1, 2, 0, 2, 0, 0, 2, 0, 0, 0,
        2, 1, 1, 0, 0, 0, 0, 1, 2, 1, 1, 1, 2, 2, 0, 1, 2, 1, 0, 2, 1, 1, 2, 2, 0, 1, 2,
        0, 0, 1, 0, 1, 2, 2, 0, 0, 2, 2, 0, 1, 1, 2, 2, 1, 0, 2, 0, 2, 2, 2, 1, 0, 1, 0,
        2, 2, 0, 2, 2, 1, 2, 2, 2, 1, 0, 0, 0, 1, 0, 0, 1, 2, 1, 1, 2, 1, 0, 0, 0, 2, 1,
        0, 0, 0, 2, 1, 2, 2, 1, 0, 1, 2, 2, 1, 2, 0, 0, 1, 2, 1, 2, 0, 0, 1, 2, 2, 2, 1,
        1, 1, 2, 2, 2, 2, 1, 2, 2, 2, 1, 1, 1, 0, 2, 0, 0, 1, 2, 2, 2, 2, 1, 2, 0, 2, 2,
        2, 1, 0, 2, 0, 1, 1, 0, 2, 2, 1, 0, 2, 1, 2, 0, 1, 1, 1, 1, 0, 0, 2, 1, 0, 2, 1,
    ],
    [
        0, 2, 2, 1, 1, 0, 0, 0, 0, 0, 1, 1, 2, 1, 2, 2, 0, 0, 1, 1, 2, 0, 1, 0, 2, 1, 2,
        1, 2, 2, 0, 1, 2, 0, 2, 2, 1, 0, 2, 2, 0, 0, 1, 0, 1, 1, 0, 1, 0, 1, 2, 0, 1, 0,
        0, 0, 2, 1, 1, 0, 0, 1, 0, 2, 2, 1, 1, 1, 2, 0, 0, 2, 1, 1, 2, 2, 1, 2, 2, 0, 1,
        1, 0, 1, 0, 0, 0, 2, 2, 2, 0, 0, 2, 0, 2, 2, 2, 2, 1, 1, 0, 0, 2, 0, 2, 0, 2, 2,
        1, 2, 1, 0, 2, 1, 2, 0, 1, 0, 2, 2, 0, 0, 2, 1, 0, 1, 2, 1, 0, 1, 0, 1, 0, 2, 1,
        1, 2, 2, 2, 1, 2, 2, 0, 1, 0, 1, 1, 2, 0, 0, 2, 2, 1, 1, 0, 2, 2, 2, 0, 2, 1, 2,
        1, 1, 1, 2, 1, 0, 2, 2, 2, 0, 2, 1, 0, 2, 0, 1, 2, 1, 0, 2, 0, 0, 2, 1, 0, 1, 2,
        0, 2, 0, 0, 1, 0, 2, 1, 0, 1, 1, 0, 2, 0, 2, 0, 0, 2, 0, 0, 1, 2, 2, 1, 0, 0, 0,
        0, 2, 2, 2, 0, 1, 1, 2, 0, 2, 2, 2, 1, 2, 2, 2, 2, 1, 0, 2, 2, 0, 0, 1, 0, 2, 1,
    ],
    [
        0, 1, 0, 1, 2, 0, 2, 0, 0, 2, 2, 1, 1, 0, 1, 1, 0, 0, 2, 1, 2, 1, 0, 0, 0, 2, 1,
        1, 2, 2, 2, 1, 2, 2, 1, 1, 0, 1, 1, 2, 0, 0, 0, 2, 1, 0, 0, 2, 2, 2, 1, 2, 1, 0,
        1, 1, 2, 2, 2, 0, 2, 2, 2, 0, 2, 1, 1, 1, 0, 0, 2, 1, 0, 2, 1, 2, 2, 2, 1, 1, 0,
        0, 0, 2, 0, 1, 2, 2, 1, 2, 2, 2, 0, 1, 0, 2, 0, 0, 0, 1, 1, 2, 1, 2, 2, 0, 1, 1,
        1, 2, 0, 1, 0, 2, 2, 2, 1, 1, 2, 0, 1, 2, 1, 2, 2, 2, 0, 2, 0, 0, 1, 1, 0, 1, 1,
        0, 1, 0, 2, 1, 0, 0, 0, 0, 0, 2, 2, 0, 0, 1, 2, 0, 0, 2, 2, 0, 1, 2, 2, 0, 2, 0,
        2, 0, 2, 0, 2, 2, 0, 1, 2, 1, 2, 1, 2, 0, 0, 2, 0, 1, 1, 2, 1, 1, 2, 0, 0, 1, 2,
        2, 0, 0, 0, 2, 2, 2, 2, 2, 2, 1, 1, 2, 2, 2, 0, 0, 0, 2, 2, 0, 1, 1, 1, 1, 1, 2,
        2, 0, 2, 2, 1, 0, 0, 1, 1, 2, 0, 0, 1, 1, 1, 0, 1, 2, 2, 2, 2, 1, 1, 2, 0, 1, 2,
    ],
    [
        1, 0, 2, 2, 0, 2, 0, 0, 1, 2, 0, 1, 0, 0, 1, 0, 2, 2, 0, 0, 1, 0, 0, 0, 2, 1, 0,
        1, 2, 0, 0, 2, 2, 1, 0, 2, 1, 0, 2, 0, 2, 1, 1, 0, 0, 0, 0, 2, 2, 2, 1, 1, 2, 2,
        0, 2, 2, 2, 2, 2, 0, 1, 2, 0, 0, 2, 0, 0, 1, 2, 0, 0, 2, 0, 0, 0, 2, 2, 0, 2, 0,
        0, 0, 1, 2, 2, 0, 0, 1, 0, 1, 1, 2, 2, 2, 1, 2, 0, 1, 0, 2, 1, 1, 2, 0, 1, 0, 1,
        2, 0, 1, 0, 2, 0, 1, 1, 1, 0, 0, 1, 2, 2, 1, 2, 1, 2, 2, 0, 2, 2, 0, 0, 2, 1, 0,
        2, 0, 0, 0, 1, 0, 1, 0, 0, 2, 0, 1, 1, 0, 1, 2, 0, 1, 0, 1, 2, 0, 0, 1, 0, 0, 1,
        1, 1, 0, 2, 2, 0, 0, 0, 1, 1, 2, 1, 1, 0, 1, 1, 1, 1, 2, 0, 0, 1, 0, 0, 1, 0, 1,
        2, 2, 2, 0, 0, 0, 0, 1, 1, 2, 1, 1, 1, 1, 0, 1, 1, 2, 0, 0, 2, 0, 2, 0, 0, 2, 2,
        2, 0, 0, 2, 1, 0, 0, 2, 2, 1, 1, 0, 1, 0, 1, 1, 2, 1, 2, 1, 0, 2, 1, 0, 2, 0, 1,
    ],
    [
        2, 2, 0, 0, 0, 0, 2, 1, 0, 2, 2, 1, 1, 0, 2, 1, 0, 0, 1, 1, 2, 1, 1, 0, 0, 1, 0,
        1, 2, 0, 0, 1, 2, 0, 0, 1, 1, 0, 2, 2, 2, 0, 2, 2, 1, 0, 1, 1, 2, 1, 0, 0, 1, 1,
        2, 0, 2, 0, 2, 1, 0, 1, 2, 2, 1, 1, 2, 2, 0, 2, 1, 2, 0, 2, 0, 1, 2, 0, 2, 2, 1,
        1, 1, 1, 0, 0, 1, 0, 1, 2, 2, 0, 2, 2, 0, 0, 1, 1, 2, 2, 0, 0, 0, 1, 2, 1, 2, 1,
        2, 1, 1, 1, 2, 1, 1, 2, 1, 2, 0, 2, 1, 0, 0, 0, 0, 1, 1, 1, 2, 0, 1, 2, 0, 1, 1,
        1, 1, 2, 0, 0, 0, 0, 2, 1, 0, 1, 2, 2, 1, 0, 2, 1, 0, 2, 1, 2, 0, 1, 0, 0, 0, 0,
        0, 2, 1, 0, 1, 0, 2, 0, 0, 2, 1, 0, 2, 2, 2, 2, 0, 0, 1, 0, 0, 1, 2, 0, 1, 1, 2,
        0, 0, 0, 2, 0, 0, 2, 2, 2, 2, 1, 2, 0, 0, 0, 2, 2, 0, 2, 0, 1, 2, 1, 2, 2, 0, 0,
        1, 1, 0, 1, 1, 0, 2, 1, 2, 1, 0, 0, 0, 0, 1, 0, 2, 2, 2, 1, 2, 0, 1, 0, 2, 1, 2,
    ],
    [
        2, 0, 1, 0, 1, 2, 0, 2, 0, 2, 2, 1, 1, 1, 0, 1, 1, 2, 0, 1, 2, 2, 2, 0, 0, 2, 2,
        0, 0, 2, 1, 1, 1, 0, 2, 0, 1, 0, 1, 1, 2, 2, 1, 2, 1, 1, 1, 0, 2, 1, 0, 0, 2, 0,
        2, 2, 1, 0, 0, 1, 1, 0, 2, 0, 1, 1, 1, 0, 1, 0, 1, 2, 1, 2, 1, 2, 0, 2, 1, 1, 1,
        1, 2, 1, 1, 1, 2, 1, 2, 0, 1, 0, 0, 2, 1, 0, 1, 1, 0, 1, 0, 1, 1, 0, 2, 0, 0, 0,
        2, 1, 0, 0, 1, 2, 0, 1, 2, 1, 0, 1, 0, 2, 0, 0, 0, 1, 2, 2, 2, 2, 2, 0, 1, 1, 2,
        2, 1, 0, 0, 1, 2, 2, 2, 1, 0, 1, 1, 0, 2, 2, 1, 2, 1, 2, 0, 0, 1, 1, 1, 0, 2, 1,
        1, 2, 2, 1, 1, 2, 1, 0, 1, 0, 1, 0, 2, 0, 0, 2, 2, 2, 1, 2, 2, 2, 0, 0, 2, 2, 2,
        0, 0, 1, 1, 1, 0, 2, 2, 1, 1, 2, 1, 1, 2, 1, 1, 1, 2, 0, 0, 0, 0, 0, 0, 2, 1, 2,
        2, 1, 0, 0, 0, 2, 1, 2, 0, 0, 1, 1, 2, 2, 1, 2, 1, 2, 2, 1, 2, 1, 0, 0, 2, 1, 0,
    ],
    [
        0, 0, 2, 2, 1, 1, 1, 0, 1, 2, 2, 2, 1, 2, 2, 2, 0, 1, 2, 1, 2, 0, 0, 1, 1, 2, 0,
        1, 1, 1, 2, 2, 1, 2, 2, 0, 2, 1, 1, 1, 0, 0, 0, 2, 0, 2, 1, 2, 2, 2, 2, 2, 0, 2,
        2, 2, 0, 1, 0, 0, 1, 0, 0, 2, 1, 2, 1, 0, 0, 0, 0, 1, 1, 2, 2, 2, 1, 2, 0, 1, 1,
        2, 1, 1, 2, 0, 1, 0, 2, 2, 0, 0, 0, 2, 0, 1, 2, 1, 0, 1, 1, 2, 0, 1, 0, 1, 2, 2,
        0, 2, 2, 0, 1, 1, 1, 2, 2, 0, 0, 0, 2, 2, 1, 1, 1, 2, 1, 1, 2, 2, 1, 2, 2, 1, 0,
        0, 0, 1, 0, 0, 0, 0, 1, 1, 2, 1, 0, 0, 2, 0, 1, 1, 2, 0, 2, 1, 1, 0, 1, 2, 2, 2,
        1, 2, 1, 1, 0, 1, 2, 1, 2, 0, 2, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 0,
        0, 0, 0, 0, 2, 0, 2, 0, 2, 0, 0, 0, 2, 0, 2, 1, 2, 1, 0, 1, 2, 0, 2, 2, 2, 2, 2,
        2, 1, 0, 1, 0, 2, 0, 0, 0, 2, 1, 2, 2, 2, 2, 0, 1, 2, 1, 2, 0, 1, 0, 1, 2, 0, 1,
    ],
    [
        1, 1, 0, 1, 1, 1, 0, 0, 2, 1, 2, 0, 0, 1, 2, 2, 1, 1, 2, 1, 2, 2, 2, 2, 0, 2, 0,
        0, 1, 0, 1, 0, 1, 0, 1, 0, 2, 0, 1, 2, 1, 2, 1, 2, 2, 2, 1, 0, 1, 1, 2, 1, 0, 1,
        2, 1, 2, 0, 2, 0, 2, 2, 1, 1, 1, 1, 1, 1, 2, 0, 1, 2, 2, 0, 0, 0, 1, 2, 0, 0, 2,
        2, 1, 1, 1, 2, 0, 2, 0, 2, 1, 2, 2, 1, 2, 1, 1, 2, 2, 2, 0, 0, 0, 2, 0, 0, 1, 1,
        1, 1, 1, 2, 0, 0, 2, 1, 1, 0, 0, 1, 2, 2, 0, 1, 1, 1, 2, 0, 2, 2, 2, 2, 2, 1, 1,
        2, 1, 0, 2, 0, 0, 2, 2, 0, 0, 2, 0, 2, 0, 0, 2, 0, 1, 0, 0, 2, 1, 0, 0, 0, 1, 1,
        0, 1, 1, 0, 1, 2, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 1, 2, 2, 0, 0, 1, 1, 0, 0, 1, 2,
        2, 1, 2, 1, 0, 2, 0, 2, 2, 0, 0, 2, 2, 0, 2, 2, 0, 0, 1, 0, 2, 0, 0, 0, 0, 1, 2,
        0, 2, 2, 0, 1, 0, 1, 2, 0, 1, 0, 0, 2, 1, 1, 1, 0, 0, 1, 0, 1, 1, 1, 2, 2, 2, 0,
    ],
];

pub const SBOX_LOOKUP: [Trit; 27] = [
    6, 25, 17, 5, 15, 10, 4, 20, 24, 0, 1, 2, 9, 22, 26, 18, 16, 14, 3, 13, 23, 7, 11,
    12, 8, 21, 19,
];

pub const SHIFT_ROWS_LANES: [usize; 729] = [
    216, 379, 164, 462, 85, 329, 681, 493, 656, 366, 16, 584, 603, 712, 65, 147, 121,
    203, 561, 292, 50, 429, 538, 269, 315, 451, 641, 243, 406, 191, 489, 112, 356, 708,
    520, 683, 393, 43, 611, 630, 10, 92, 174, 148, 230, 588, 319, 77, 456, 565, 296, 342,
    478, 668, 270, 433, 218, 516, 139, 383, 6, 547, 710, 420, 70, 638, 657, 37, 119, 201,
    175, 257, 615, 346, 104, 483, 592, 323, 369, 505, 695, 297, 460, 245, 543, 166, 410,
    33, 574, 8, 447, 97, 665, 684, 64, 146, 228, 202, 284, 642, 373, 131, 510, 619, 350,
    396, 532, 722, 324, 487, 272, 570, 193, 437, 60, 601, 35, 474, 124, 692, 711, 91,
    173, 255, 229, 311, 669, 400, 158, 537, 646, 377, 423, 559, 20, 351, 514, 299, 597,
    220, 464, 87, 628, 62, 501, 151, 719, 9, 118, 200, 282, 256, 338, 696, 427, 185, 564,
    673, 404, 450, 586, 47, 378, 541, 326, 624, 247, 491, 114, 655, 89, 528, 178, 17, 36,
    145, 227, 309, 283, 365, 723, 454, 212, 591, 700, 431, 477, 613, 74, 405, 568, 353,
    651, 274, 518, 141, 682, 116, 555, 205, 44, 63, 172, 254, 336, 310, 392, 21, 481,
    239, 618, 727, 458, 504, 640, 101, 432, 595, 380, 678, 301, 545, 168, 709, 143, 582,
    232, 71, 90, 199, 281, 363, 337, 419, 48, 508, 266, 645, 25, 485, 531, 667, 128, 459,
    622, 407, 705, 328, 572, 195, 7, 170, 609, 259, 98, 117, 226, 308, 390, 364, 446, 75,
    535, 293, 672, 52, 512, 558, 694, 155, 486, 649, 434, 3, 355, 599, 222, 34, 197, 636,
    286, 125, 144, 253, 335, 417, 391, 473, 102, 562, 320, 699, 79, 539, 585, 721, 182,
    513, 676, 461, 30, 382, 626, 249, 61, 224, 663, 313, 152, 171, 280, 362, 444, 418,
    500, 129, 589, 347, 726, 106, 566, 612, 19, 209, 540, 703, 488, 57, 409, 653, 276,
    88, 251, 690, 340, 179, 198, 307, 389, 471, 445, 527, 156, 616, 374, 24, 133, 593,
    639, 46, 236, 567, 1, 515, 84, 436, 680, 303, 115, 278, 717, 367, 206, 225, 334, 416,
    498, 472, 554, 183, 643, 401, 51, 160, 620, 666, 73, 263, 594, 28, 542, 111, 463,
    707, 330, 142, 305, 15, 394, 233, 252, 361, 443, 525, 499, 581, 210, 670, 428, 78,
    187, 647, 693, 100, 290, 621, 55, 569, 138, 490, 5, 357, 169, 332, 42, 421, 260, 279,
    388, 470, 552, 526, 608, 237, 697, 455, 105, 214, 674, 720, 127, 317, 648, 82, 596,
    165, 517, 32, 384, 196, 359, 69, 448, 287, 306, 415, 497, 579, 553, 635, 264, 724,
    482, 132, 241, 701, 18, 154, 344, 675, 109, 623, 192, 544, 59, 411, 223, 386, 96,
    475, 314, 333, 442, 524, 606, 580, 662, 291, 22, 509, 159, 268, 728, 45, 181, 371,
    702, 136, 650, 219, 571, 86, 438, 250, 413, 123, 502, 341, 360, 469, 551, 633, 607,
    689, 318, 49, 536, 186, 295, 26, 72, 208, 398, 0, 163, 677, 246, 598, 113, 465, 277,
    440, 150, 529, 368, 387, 496, 578, 660, 634, 716, 345, 76, 563, 213, 322, 53, 99,
    235, 425, 27, 190, 704, 273, 625, 140, 492, 304, 467, 177, 556, 395, 414, 523, 605,
    687, 661, 14, 372, 103, 590, 240, 349, 80, 126, 262, 452, 54, 217, 2, 300, 652, 167,
    519, 331, 494, 204, 583, 422, 441, 550, 632, 714, 688, 41, 399, 130, 617, 267, 376,
    107, 153, 289, 479, 81, 244, 29, 327, 679, 194, 546, 358, 521, 231, 610, 449, 468,
    577, 659, 12, 715, 68, 426, 157, 644, 294, 403, 134, 180, 316, 506, 108, 271, 56,
    354, 706, 221, 573, 385, 548, 258, 637, 476, 495, 604, 686, 39, 13, 95, 453, 184,
    671, 321, 430, 161, 207, 343, 533, 135, 298, 83, 381, 4, 248, 600, 412, 575, 285,
    664, 503, 522, 631, 713, 66, 40, 122, 480, 211, 698, 348, 457, 188, 234, 370, 560,
    162, 325, 110, 408, 31, 275, 627, 439, 602, 312, 691, 530, 549, 658, 11, 93, 67, 149,
    507, 238, 725, 375, 484, 215, 261, 397, 587, 189, 352, 137, 435, 58, 302, 654, 466,
    629, 339, 718, 557, 576, 685, 38, 120, 94, 176, 534, 265, 23, 402, 511, 242, 288,
    424, 614,
];