use crate::{types::Trit, types::Tryte};

use heapless::{consts::U32, FnvIndexMap, IndexMap as HashMap};

use core::iter::FromIterator;

// LUT-SIZE: 3 Bytes * 27 = 81 Byte
pub const TRYTE_CODE_TO_TRITS: [[Trit; 3]; 27] = [
    [0, 0, 0],    //  0 => 0
    [1, 0, 0],    //  1 => 1
    [-1, 1, 0],   //  2 => 2
    [0, 1, 0],    //  3 => 3
    [1, 1, 0],    //  4 => 4
    [-1, -1, 1],  //  5 => 5
    [0, -1, 1],   //  6 => 6
    [1, -1, 1],   //  7 => 7
    [-1, 0, 1],   //  8 => 8
    [0, 0, 1],    //  9 => 9
    [1, 0, 1],    // 10 => 10
    [-1, 1, 1],   // 11 => 11
    [0, 1, 1],    // 12 => 12
    [1, 1, 1],    // 13 => 13
    [-1, -1, -1], // 14 => -13
    [0, -1, -1],  // 15 => -12
    [1, -1, -1],  // 16 => -11
    [-1, 0, -1],  // 17 => -10
    [0, 0, -1],   // 18 => -9
    [1, 0, -1],   // 19 => -8
    [-1, 1, -1],  // 20 => -7
    [0, 1, -1],   // 21 => -6
    [1, 1, -1],   // 22 => -5
    [-1, -1, 0],  // 23 => -4
    [0, -1, 0],   // 24 => -3
    [1, -1, 0],   // 25 => -2
    [-1, 0, 0],   // 26 => -1
];

//  0 => 57
//  1 => 65
// ...
// 26 => 90
// LUT-SIZE: 27 Byte
pub const TRYTE_CODE_TO_ASCII_CODE: [Tryte; 27] = [
    57, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84,
    85, 86, 87, 88, 89, 90,
];

//  0 => 57
//  1 => 90
// ...
// 26 => 65
// LUT-SIZE: 27 Byte
pub const TRYTE_CODE_TO_ASCII_CODE_NEG: [Tryte; 27] = [
    57, 90, 89, 88, 87, 86, 85, 84, 83, 82, 81, 80, 79, 78, 77, 76, 75, 74, 73, 72, 71,
    70, 69, 68, 67, 66, 65,
];

// 57 => 0
// 65 => 1
// ...
// 90 => 26
// LUT-SIZE: 2 Byte * 27 = 54 Byte
lazy_static! {
    pub static ref ASCII_CODE_TO_TRYTE_CODE: FnvIndexMap::<Tryte, usize, U32> = {
        HashMap::from_iter(
            TRYTE_CODE_TO_ASCII_CODE.iter().enumerate().map(|(v, &k)| (k, v)),
        )
    };
}

// LUT-SIZE: 4 Byte * 32 = 128 Byte
lazy_static! {
    pub static ref ASCII_CODE_TO_TRITS: FnvIndexMap::<Tryte, [Trit; 3], U32> = [
        (57, [0, 0, 0]),
        (65, [1, 0, 0]),
        (66, [-1, 1, 0]),
        (67, [0, 1, 0]),
        (68, [1, 1, 0]),
        (69, [-1, -1, 1]),
        (70, [0, -1, 1]),
        (71, [1, -1, 1]),
        (72, [-1, 0, 1]),
        (73, [0, 0, 1]),
        (74, [1, 0, 1]),
        (75, [-1, 1, 1]),
        (76, [0, 1, 1]),
        (77, [1, 1, 1]),
        (78, [-1, -1, -1]),
        (79, [0, -1, -1]),
        (80, [1, -1, -1]),
        (81, [-1, 0, -1]),
        (82, [0, 0, -1]),
        (83, [1, 0, -1]),
        (84, [-1, 1, -1]),
        (85, [0, 1, -1]),
        (86, [1, 1, -1]),
        (87, [-1, -1, 0]),
        (88, [0, -1, 0]),
        (89, [1, -1, 0]),
        (90, [-1, 0, 0])
    ]
    .iter()
    .cloned()
    .collect();
}

// TODO: Make using this LUT optional!
// LUT-SIZE: 8 Byte * 34 * 11 = 2992 Byte
#[allow(clippy::unreadable_literal)]
#[rustfmt::skip]
pub const ASCII_CODE_SEQ_TO_NUM: [[i64; 34]; 11] = [
    [   0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, -13, -12, -11, -10, -9, -8, -7, -6, -5, -4, -3, -2, -1 ],
    [   0, 0, 0, 0, 0, 0, 0, 0, 27, 54, 81, 108, 135, 162, 189, 216, 243, 270, 297, 324, 351, -351, -324, -297, -270, -243, -216, -189, -162, -135, -108, -81, -54, -27 ],
    [   0, 0, 0, 0, 0, 0, 0, 0, 729, 1458, 2187, 2916, 3645, 4374, 5103, 5832, 6561, 7290, 8019, 8748, 9477, -9477, -8748, -8019, -7290, -6561, -5832, -5103, -4374, -3645, -2916, -2187, -1458, -729 ],
    [   0, 0, 0, 0, 0, 0, 0, 0, 19683, 39366, 59049, 78732, 98415, 118098, 137781, 157464, 177147, 196830, 216513, 236196, 255879, -255879, -236196, -216513, -196830, -177147, -157464, -137781, -118098, -98415, -78732, -59049, -39366, -19683 ],
    [   0, 0, 0, 0, 0, 0, 0, 0, 531441, 1062882, 1594323, 2125764, 2657205, 3188646, 3720087, 4251528, 4782969, 5314410, 5845851, 6377292, 6908733, -6908733, -6377292, -5845851, -5314410, -4782969, -4251528, -3720087, -3188646, -2657205, -2125764, -1594323, -1062882, -531441 ],
    [   0, 0, 0, 0, 0, 0, 0, 0, 14348907, 28697814, 43046721, 57395628, 71744535, 86093442, 100442349, 114791256, 129140163, 143489070, 157837977, 172186884, 186535791, -186535791, -172186884, -157837977, -143489070, -129140163, -114791256, -100442349, -86093442, -71744535, -57395628, -43046721, -28697814, -14348907 ],
    [   0, 0, 0, 0, 0, 0, 0, 0, 387420489, 774840978, 1162261467, 1549681956, 1937102445, 2324522934, 2711943423, 3099363912, 3486784401, 3874204890, 4261625379, 4649045868, 5036466357, -5036466357, -4649045868, -4261625379, -3874204890, -3486784401, -3099363912, -2711943423, -2324522934, -1937102445, -1549681956, -1162261467, -774840978, -387420489 ],
    [   0, 0, 0, 0, 0, 0, 0, 0, 10460353203, 20920706406, 31381059609, 41841412812, 52301766015, 62762119218, 73222472421, 83682825624, 94143178827, 104603532030, 115063885233, 125524238436, 135984591639, -135984591639, -125524238436, -115063885233, -104603532030, -94143178827, -83682825624, -73222472421, -62762119218, -52301766015, -41841412812, -31381059609, -20920706406, -10460353203 ],
    [   0, 0, 0, 0, 0, 0, 0, 0, 282429536481, 564859072962, 847288609443, 1129718145924, 1412147682405, 1694577218886, 1977006755367, 2259436291848, 2541865828329, 2824295364810, 3106724901291, 3389154437772, 3671583974253, -3671583974253, -3389154437772, -3106724901291, -2824295364810, -2541865828329, -2259436291848, -1977006755367, -1694577218886, -1412147682405, -1129718145924, -847288609443, -564859072962, -282429536481 ],
    [   0, 0, 0, 0, 0, 0, 0, 0, 7625597484987, 15251194969974, 22876792454961, 30502389939948, 38127987424935, 45753584909922, 53379182394909, 61004779879896, 68630377364883, 76255974849870, 83881572334857, 91507169819844, 99132767304831, -99132767304831, -91507169819844, -83881572334857, -76255974849870, -68630377364883, -61004779879896, -53379182394909, -45753584909922, -38127987424935, -30502389939948, -22876792454961, -15251194969974, -7625597484987 ],
    [   0, 0, 0, 0, 0, 0, 0, 0, 205891132094649, 411782264189298, 617673396283947, 823564528378596, 1029455660473245, 1235346792567894, 1441237924662543, 1647129056757192, 1853020188851841, 2058911320946490, 2264802453041139, 2470693585135788, 2676584717230437, -2676584717230437, -2470693585135788, -2264802453041139, -2058911320946490, -1853020188851841, -1647129056757192, -1441237924662543, -1235346792567894, -1029455660473245, -823564528378596, -617673396283947, -411782264189298, -205891132094649 ],
];