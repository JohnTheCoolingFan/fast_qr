//! Contains all different levels of quality.
//! And allows to find easily max bits per version/quality pair

#![deny(unsafe_code)]
#![warn(missing_docs)]

use crate::runtime::qrcode::ecl::ECL;
use crate::runtime::qrcode::encode::Mode;
use crate::runtime::qrcode::version::Version;

#[cfg(test)]
mod test;

/// Fetches the right array to retrieve the information on **groups**
pub const fn ecc_to_groups(quality: ECL, version: Version) -> [(usize, usize); 2] {
    const L: [[(usize, usize); 2]; 40] = [
        [(1, 19), (0, 0)],
        [(1, 34), (0, 0)],
        [(1, 55), (0, 0)],
        [(1, 80), (0, 0)],
        [(1, 108), (0, 0)],
        [(2, 68), (0, 0)],
        [(2, 78), (0, 0)],
        [(2, 97), (0, 0)],
        [(2, 116), (0, 0)],
        [(2, 68), (2, 69)],
        [(4, 81), (0, 0)],
        [(2, 92), (2, 93)],
        [(4, 107), (0, 0)],
        [(3, 115), (1, 116)],
        [(5, 87), (1, 88)],
        [(5, 98), (1, 99)],
        [(1, 107), (5, 108)],
        [(5, 120), (1, 121)],
        [(3, 113), (4, 114)],
        [(3, 107), (5, 108)],
        [(4, 116), (4, 117)],
        [(2, 111), (7, 112)],
        [(4, 121), (5, 122)],
        [(6, 117), (4, 118)],
        [(8, 106), (4, 107)],
        [(10, 114), (2, 115)],
        [(8, 122), (4, 123)],
        [(3, 117), (10, 118)],
        [(7, 116), (7, 117)],
        [(5, 115), (10, 116)],
        [(13, 115), (3, 116)],
        [(17, 115), (0, 0)],
        [(17, 115), (1, 116)],
        [(13, 115), (6, 116)],
        [(12, 121), (7, 122)],
        [(6, 121), (14, 122)],
        [(17, 122), (4, 123)],
        [(4, 122), (18, 123)],
        [(20, 117), (4, 118)],
        [(19, 118), (6, 119)],
    ];

    const M: [[(usize, usize); 2]; 40] = [
        [(1, 16), (0, 0)],
        [(1, 28), (0, 0)],
        [(1, 44), (0, 0)],
        [(2, 32), (0, 0)],
        [(2, 43), (0, 0)],
        [(4, 27), (0, 0)],
        [(4, 31), (0, 0)],
        [(2, 38), (2, 39)],
        [(3, 36), (2, 37)],
        [(4, 43), (1, 44)],
        [(1, 50), (4, 51)],
        [(6, 36), (2, 37)],
        [(8, 37), (1, 38)],
        [(4, 40), (5, 41)],
        [(5, 41), (5, 42)],
        [(7, 45), (3, 46)],
        [(10, 46), (1, 47)],
        [(9, 43), (4, 44)],
        [(3, 44), (11, 45)],
        [(3, 41), (13, 42)],
        [(17, 42), (0, 0)],
        [(17, 46), (0, 0)],
        [(4, 47), (14, 48)],
        [(6, 45), (14, 46)],
        [(8, 47), (13, 48)],
        [(19, 46), (4, 47)],
        [(22, 45), (3, 46)],
        [(3, 45), (23, 46)],
        [(21, 45), (7, 46)],
        [(19, 47), (10, 48)],
        [(2, 46), (29, 47)],
        [(10, 46), (23, 47)],
        [(14, 46), (21, 47)],
        [(14, 46), (23, 47)],
        [(12, 47), (26, 48)],
        [(6, 47), (34, 48)],
        [(29, 46), (14, 47)],
        [(13, 46), (32, 47)],
        [(40, 47), (7, 48)],
        [(18, 47), (31, 48)],
    ];

    const Q: [[(usize, usize); 2]; 40] = [
        [(1, 13), (0, 0)],
        [(1, 22), (0, 0)],
        [(2, 17), (0, 0)],
        [(2, 24), (0, 0)],
        [(2, 15), (2, 16)],
        [(4, 19), (0, 0)],
        [(2, 14), (4, 15)],
        [(4, 18), (2, 19)],
        [(4, 16), (4, 17)],
        [(6, 19), (2, 20)],
        [(4, 22), (4, 23)],
        [(4, 20), (6, 21)],
        [(8, 20), (4, 21)],
        [(11, 16), (5, 17)],
        [(5, 24), (7, 25)],
        [(15, 19), (2, 20)],
        [(1, 22), (15, 23)],
        [(17, 22), (1, 23)],
        [(17, 21), (4, 22)],
        [(15, 24), (5, 25)],
        [(17, 22), (6, 23)],
        [(7, 24), (16, 25)],
        [(11, 24), (14, 25)],
        [(11, 24), (16, 25)],
        [(7, 24), (22, 25)],
        [(28, 22), (6, 23)],
        [(8, 23), (26, 24)],
        [(4, 24), (31, 25)],
        [(1, 23), (37, 24)],
        [(15, 24), (25, 25)],
        [(42, 24), (1, 25)],
        [(10, 24), (35, 25)],
        [(29, 24), (19, 25)],
        [(44, 24), (7, 25)],
        [(39, 24), (14, 25)],
        [(46, 24), (10, 25)],
        [(49, 24), (10, 25)],
        [(48, 24), (14, 25)],
        [(43, 24), (22, 25)],
        [(34, 24), (34, 25)],
    ];

    const H: [[(usize, usize); 2]; 40] = [
        [(1, 9), (0, 0)],
        [(1, 16), (0, 0)],
        [(2, 13), (0, 0)],
        [(4, 9), (0, 0)],
        [(2, 11), (2, 12)],
        [(4, 15), (0, 0)],
        [(4, 13), (1, 14)],
        [(4, 14), (2, 15)],
        [(4, 12), (4, 13)],
        [(6, 15), (2, 16)],
        [(3, 12), (8, 13)],
        [(7, 14), (4, 15)],
        [(12, 11), (4, 12)],
        [(11, 12), (5, 13)],
        [(11, 12), (7, 13)],
        [(3, 15), (13, 16)],
        [(2, 14), (17, 15)],
        [(2, 14), (19, 15)],
        [(9, 13), (16, 14)],
        [(15, 15), (10, 16)],
        [(19, 16), (6, 17)],
        [(34, 13), (0, 0)],
        [(16, 15), (14, 16)],
        [(30, 16), (2, 17)],
        [(22, 15), (13, 16)],
        [(33, 16), (4, 17)],
        [(12, 15), (28, 16)],
        [(11, 15), (31, 16)],
        [(19, 15), (26, 16)],
        [(23, 15), (25, 16)],
        [(23, 15), (28, 16)],
        [(19, 15), (35, 16)],
        [(11, 15), (46, 16)],
        [(59, 16), (1, 17)],
        [(22, 15), (41, 16)],
        [(2, 15), (64, 16)],
        [(24, 15), (46, 16)],
        [(42, 15), (32, 16)],
        [(10, 15), (67, 16)],
        [(20, 15), (61, 16)],
    ];

    let version = version as usize;
    match quality {
        ECL::L => L[version],
        ECL::M => M[version],
        ECL::Q => Q[version],
        ECL::H => H[version],
    }
}

/// Fetches the right array to retrieve the **format information**
pub const fn ecm_to_format_information(quality: ECL, mask_nb: usize) -> u16 {
    const L: [u16; 8] = [
        0b111011111000100,
        0b111001011110011,
        0b111110110101010,
        0b111100010011101,
        0b110011000101111,
        0b110001100011000,
        0b110110001000001,
        0b110100101110110,
    ];

    const M: [u16; 8] = [
        0b101010000010010,
        0b101000100100101,
        0b101111001111100,
        0b101101101001011,
        0b100010111111001,
        0b100000011001110,
        0b100111110010111,
        0b100101010100000,
    ];

    const Q: [u16; 8] = [
        0b011010101011111,
        0b011000001101000,
        0b011111100110001,
        0b011101000000110,
        0b010010010110100,
        0b010000110000011,
        0b010111011011010,
        0b010101111101101,
    ];

    const H: [u16; 8] = [
        0b001011010001001,
        0b001001110111110,
        0b001110011100111,
        0b001100111010000,
        0b000011101100010,
        0b000001001010101,
        0b000110100001100,
        0b000100000111011,
    ];

    match quality {
        ECL::L => L[mask_nb],
        ECL::M => M[mask_nb],
        ECL::Q => Q[mask_nb],
        ECL::H => H[mask_nb],
    }
}

/// Returns the number of **data codewords** according to `version` and `ecl`
pub const fn data_codewords(version: Version, ecl: ECL) -> usize {
    use Version::*;

    match ecl {
        ECL::L => match version {
            V01 => 19,
            V02 => 34,
            V03 => 55,
            V04 => 80,
            V05 => 108,
            V06 => 136,
            V07 => 156,
            V08 => 194,
            V09 => 232,
            V10 => 274,
            V11 => 324,
            V12 => 370,
            V13 => 428,
            V14 => 461,
            V15 => 523,
            V16 => 589,
            V17 => 647,
            V18 => 721,
            V19 => 795,
            V20 => 861,
            V21 => 932,
            V22 => 1006,
            V23 => 1094,
            V24 => 1174,
            V25 => 1276,
            V26 => 1370,
            V27 => 1468,
            V28 => 1531,
            V29 => 1631,
            V30 => 1735,
            V31 => 1843,
            V32 => 1955,
            V33 => 2071,
            V34 => 2191,
            V35 => 2306,
            V36 => 2434,
            V37 => 2566,
            V38 => 2702,
            V39 => 2812,
            V40 => 2956,
        },
        ECL::M => match version {
            V01 => 16,
            V02 => 28,
            V03 => 44,
            V04 => 64,
            V05 => 86,
            V06 => 108,
            V07 => 124,
            V08 => 154,
            V09 => 182,
            V10 => 216,
            V11 => 254,
            V12 => 290,
            V13 => 334,
            V14 => 365,
            V15 => 415,
            V16 => 453,
            V17 => 507,
            V18 => 563,
            V19 => 627,
            V20 => 669,
            V21 => 714,
            V22 => 782,
            V23 => 860,
            V24 => 914,
            V25 => 1000,
            V26 => 1062,
            V27 => 1128,
            V28 => 1193,
            V29 => 1267,
            V30 => 1373,
            V31 => 1455,
            V32 => 1541,
            V33 => 1631,
            V34 => 1725,
            V35 => 1812,
            V36 => 1914,
            V37 => 1992,
            V38 => 2102,
            V39 => 2216,
            V40 => 2334,
        },
        ECL::Q => match version {
            V01 => 13,
            V02 => 22,
            V03 => 34,
            V04 => 48,
            V05 => 62,
            V06 => 76,
            V07 => 88,
            V08 => 110,
            V09 => 132,
            V10 => 154,
            V11 => 180,
            V12 => 206,
            V13 => 244,
            V14 => 261,
            V15 => 295,
            V16 => 325,
            V17 => 367,
            V18 => 397,
            V19 => 445,
            V20 => 485,
            V21 => 512,
            V22 => 568,
            V23 => 614,
            V24 => 664,
            V25 => 718,
            V26 => 754,
            V27 => 808,
            V28 => 871,
            V29 => 911,
            V30 => 985,
            V31 => 1033,
            V32 => 1115,
            V33 => 1171,
            V34 => 1231,
            V35 => 1286,
            V36 => 1354,
            V37 => 1426,
            V38 => 1502,
            V39 => 1582,
            V40 => 1666,
        },
        ECL::H => match version {
            V01 => 9,
            V02 => 16,
            V03 => 26,
            V04 => 36,
            V05 => 46,
            V06 => 60,
            V07 => 66,
            V08 => 86,
            V09 => 100,
            V10 => 122,
            V11 => 140,
            V12 => 158,
            V13 => 180,
            V14 => 197,
            V15 => 223,
            V16 => 253,
            V17 => 283,
            V18 => 313,
            V19 => 341,
            V20 => 385,
            V21 => 406,
            V22 => 442,
            V23 => 464,
            V24 => 514,
            V25 => 538,
            V26 => 596,
            V27 => 628,
            V28 => 661,
            V29 => 701,
            V30 => 745,
            V31 => 793,
            V32 => 845,
            V33 => 901,
            V34 => 961,
            V35 => 986,
            V36 => 1054,
            V37 => 1096,
            V38 => 1142,
            V39 => 1222,
            V40 => 1276,
        },
    }
}

/// Returns the number **data bits** according to `version` and `ecl`
pub const fn data_bits(version: Version, ecl: ECL) -> usize {
    data_codewords(version, ecl) * 8
}

/// Returns the **number of bits** required to represent a number according to `version` and `mode`
pub const fn cci_bits(version: Version, mode: Mode) -> usize {
    use Version::*;

    match mode {
        Mode::Numeric => match version {
            V01 => 10,
            V02 => 10,
            V03 => 10,
            V04 => 10,
            V05 => 10,
            V06 => 10,
            V07 => 10,
            V08 => 10,
            V09 => 10,
            V10 => 12,
            V11 => 12,
            V12 => 12,
            V13 => 12,
            V14 => 12,
            V15 => 12,
            V16 => 12,
            V17 => 12,
            V18 => 12,
            V19 => 12,
            V20 => 12,
            V21 => 12,
            V22 => 12,
            V23 => 12,
            V24 => 12,
            V25 => 12,
            V26 => 12,
            V27 => 14,
            V28 => 14,
            V29 => 14,
            V30 => 14,
            V31 => 14,
            V32 => 14,
            V33 => 14,
            V34 => 14,
            V35 => 14,
            V36 => 14,
            V37 => 14,
            V38 => 14,
            V39 => 14,
            V40 => 14,
        },
        Mode::Alphanumeric => match version {
            V01 => 9,
            V02 => 9,
            V03 => 9,
            V04 => 9,
            V05 => 9,
            V06 => 9,
            V07 => 9,
            V08 => 9,
            V09 => 9,
            V10 => 11,
            V11 => 11,
            V12 => 11,
            V13 => 11,
            V14 => 11,
            V15 => 11,
            V16 => 11,
            V17 => 11,
            V18 => 11,
            V19 => 11,
            V20 => 11,
            V21 => 11,
            V22 => 11,
            V23 => 11,
            V24 => 11,
            V25 => 11,
            V26 => 11,
            V27 => 13,
            V28 => 13,
            V29 => 13,
            V30 => 13,
            V31 => 13,
            V32 => 13,
            V33 => 13,
            V34 => 13,
            V35 => 13,
            V36 => 13,
            V37 => 13,
            V38 => 13,
            V39 => 13,
            V40 => 13,
        },
        Mode::Byte => match version {
            V01 => 8,
            V02 => 8,
            V03 => 8,
            V04 => 8,
            V05 => 8,
            V06 => 8,
            V07 => 8,
            V08 => 8,
            V09 => 8,
            V10 => 16,
            V11 => 16,
            V12 => 16,
            V13 => 16,
            V14 => 16,
            V15 => 16,
            V16 => 16,
            V17 => 16,
            V18 => 16,
            V19 => 16,
            V20 => 16,
            V21 => 16,
            V22 => 16,
            V23 => 16,
            V24 => 16,
            V25 => 16,
            V26 => 16,
            V27 => 16,
            V28 => 16,
            V29 => 16,
            V30 => 16,
            V31 => 16,
            V32 => 16,
            V33 => 16,
            V34 => 16,
            V35 => 16,
            V36 => 16,
            V37 => 16,
            V38 => 16,
            V39 => 16,
            V40 => 16,
        },
    }
}

/// Returns required **dividing polynomial** according to `version` and `ecl`
pub const fn get_polynomial(version: Version, ecl: ECL) -> &'static [u8] {
    use Version::*;
    use ECL::*;

    match (version, ecl) {
        (V01, L) => &[0, 87, 229, 146, 149, 238, 102, 21],
        (V01, M) | (V02, L) => &[0, 251, 67, 46, 61, 118, 70, 64, 94, 32, 45],
        (V01, Q) => &[
            0, 74, 152, 176, 100, 86, 100, 106, 104, 130, 218, 206, 140, 78,
        ],
        (V03, L) => &[
            0, 8, 183, 61, 91, 202, 37, 51, 58, 58, 237, 140, 124, 5, 99, 105,
        ],
        (V02, M) | (V04, H) | (V06, M) => &[
            0, 120, 104, 107, 109, 102, 161, 76, 3, 91, 191, 147, 169, 182, 194, 225, 120,
        ],
        (V01, H) => &[
            0, 43, 139, 206, 78, 43, 239, 123, 206, 214, 147, 24, 99, 150, 39, 243, 163, 136,
        ],
        (V03, Q) | (V04, M) | (V05, Q) | (V06, L) | (V07, M) | (V07, Q) | (V10, L) => &[
            0, 215, 234, 158, 94, 184, 97, 118, 170, 79, 187, 152, 148, 252, 179, 5, 98, 96, 153,
        ],
        (V04, L) | (V07, L) | (V09, Q) | (V11, L) | (V14, Q) => &[
            0, 17, 60, 79, 50, 61, 163, 26, 187, 202, 180, 221, 225, 83, 239, 156, 164, 212, 212,
            188, 190,
        ],
        (V02, Q)
        | (V03, H)
        | (V05, H)
        | (V08, M)
        | (V08, Q)
        | (V09, M)
        | (V12, M)
        | (V13, M)
        | (V13, H)
        | (V15, L) => &[
            0, 210, 171, 247, 242, 93, 230, 14, 109, 221, 53, 200, 74, 8, 172, 98, 80, 219, 134,
            160, 105, 165, 231,
        ],
        (V05, M)
        | (V06, Q)
        | (V08, L)
        | (V09, H)
        | (V10, Q)
        | (V11, H)
        | (V12, L)
        | (V13, Q)
        | (V14, M)
        | (V14, H)
        | (V15, M)
        | (V15, H)
        | (V16, L)
        | (V16, Q)
        | (V22, H) => &[
            0, 229, 121, 135, 48, 211, 117, 251, 126, 159, 180, 169, 152, 192, 226, 228, 218, 111,
            0, 117, 232, 87, 96, 227, 21,
        ],
        (V03, M)
        | (V04, Q)
        | (V05, L)
        | (V07, H)
        | (V08, H)
        | (V10, M)
        | (V12, Q)
        | (V13, L)
        | (V18, M)
        | (V19, M)
        | (V19, Q)
        | (V19, H)
        | (V20, M)
        | (V21, M)
        | (V25, L) => &[
            0, 173, 125, 158, 2, 103, 182, 118, 17, 145, 201, 111, 28, 165, 53, 161, 21, 245, 142,
            13, 102, 48, 227, 153, 145, 218, 70,
        ],
        (V02, H)
        | (V06, H)
        | (V10, H)
        | (V11, Q)
        | (V12, H)
        | (V16, M)
        | (V17, L)
        | (V17, M)
        | (V17, Q)
        | (V17, H)
        | (V18, Q)
        | (V18, H)
        | (V19, L)
        | (V20, L)
        | (V20, H)
        | (V21, L)
        | (V21, Q)
        | (V22, L)
        | (V22, M)
        | (V23, M)
        | (V24, M)
        | (V25, M)
        | (V26, L)
        | (V26, M)
        | (V26, Q)
        | (V27, M)
        | (V28, M)
        | (V29, M)
        | (V30, M)
        | (V31, M)
        | (V32, M)
        | (V33, M)
        | (V34, M)
        | (V35, M)
        | (V36, M)
        | (V37, M)
        | (V38, M)
        | (V39, M)
        | (V40, M) => &[
            0, 168, 223, 200, 104, 224, 234, 108, 180, 110, 190, 195, 147, 205, 27, 232, 201, 21,
            43, 245, 87, 42, 195, 212, 119, 242, 37, 9, 123,
        ],
        (V09, L)
        | (V11, M)
        | (V14, L)
        | (V15, Q)
        | (V16, H)
        | (V18, L)
        | (V20, Q)
        | (V21, H)
        | (V22, Q)
        | (V23, L)
        | (V23, Q)
        | (V23, H)
        | (V24, L)
        | (V24, Q)
        | (V24, H)
        | (V25, Q)
        | (V25, H)
        | (V26, H)
        | (V27, L)
        | (V27, Q)
        | (V27, H)
        | (V28, L)
        | (V28, Q)
        | (V28, H)
        | (V29, L)
        | (V29, Q)
        | (V29, H)
        | (V30, L)
        | (V30, Q)
        | (V30, H)
        | (V31, L)
        | (V31, Q)
        | (V31, H)
        | (V32, L)
        | (V32, Q)
        | (V32, H)
        | (V33, L)
        | (V33, Q)
        | (V33, H)
        | (V34, L)
        | (V34, Q)
        | (V34, H)
        | (V35, L)
        | (V35, Q)
        | (V35, H)
        | (V36, L)
        | (V36, Q)
        | (V36, H)
        | (V37, L)
        | (V37, Q)
        | (V37, H)
        | (V38, L)
        | (V38, Q)
        | (V38, H)
        | (V39, L)
        | (V39, Q)
        | (V39, H)
        | (V40, L)
        | (V40, Q)
        | (V40, H) => &[
            0, 41, 173, 145, 152, 216, 31, 179, 182, 50, 48, 110, 86, 239, 96, 222, 125, 42, 173,
            226, 193, 224, 130, 156, 37, 251, 216, 238, 40, 192, 180,
        ],
    }
}

/// Contains the score for [**light/dark module ratio**](https://www.thonky.com/qr-code-tutorial/data-masking#evaluation-condition-4)
pub const PERCENT_SCORE: [u32; 256] = [
    90, 90, 90, 90, 90, 80, 80, 80, 80, 80, 70, 70, 70, 70, 70, 60, 60, 60, 60, 60, 50, 50, 50, 50,
    50, 40, 40, 40, 40, 40, 30, 30, 30, 30, 30, 20, 20, 20, 20, 20, 10, 10, 10, 10, 10, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 10, 10, 10, 10, 10, 20, 20, 20, 20, 20, 30, 30, 30, 30, 30, 40, 40, 40, 40,
    40, 50, 50, 50, 50, 50, 60, 60, 60, 60, 60, 70, 70, 70, 70, 70, 80, 80, 80, 80, 80, 90, 90, 90,
    90, 90, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255,
];

/// Contains the score for [**trailing bits**](https://www.thonky.com/qr-code-tutorial/data-masking#evaluation-condition-1)
pub const fn trailing(buffer: u16, buffer_size: u32) -> u32 {
    const TRAILING_SCORE_11: [u32; 128] = [
        9, 3, 8, 4, 8, 4, 7, 5, 8, 4, 7, 5, 7, 5, 6, 6, 8, 4, 7, 5, 7, 5, 6, 6, 7, 5, 6, 6, 6, 6,
        5, 7, 8, 4, 7, 5, 7, 5, 6, 6, 7, 5, 6, 6, 6, 6, 5, 7, 7, 5, 6, 6, 6, 6, 5, 7, 6, 6, 5, 7,
        5, 7, 4, 8, 8, 4, 7, 5, 7, 5, 6, 6, 7, 5, 6, 6, 6, 6, 5, 7, 7, 5, 6, 6, 6, 6, 5, 7, 6, 6,
        5, 7, 5, 7, 4, 8, 7, 5, 6, 6, 6, 6, 5, 7, 6, 6, 5, 7, 5, 7, 4, 8, 6, 6, 5, 7, 5, 7, 4, 8,
        5, 7, 4, 8, 4, 8, 3, 9,
    ];

    const TRAILING_SCORE_10: [u32; 64] = [
        8, 3, 7, 4, 7, 4, 6, 5, 7, 4, 6, 5, 6, 5, 5, 6, 7, 4, 6, 5, 6, 5, 5, 6, 6, 5, 5, 6, 5, 6,
        4, 7, 7, 4, 6, 5, 6, 5, 5, 6, 6, 5, 5, 6, 5, 6, 4, 7, 6, 5, 5, 6, 5, 6, 4, 7, 5, 6, 4, 7,
        4, 7, 3, 8,
    ];

    const TRAILING_SCORE_9: [u32; 32] = [
        7, 3, 6, 4, 6, 4, 5, 5, 6, 4, 5, 5, 5, 5, 4, 6, 6, 4, 5, 5, 5, 5, 4, 6, 5, 5, 4, 6, 4, 6,
        3, 7,
    ];

    const TRAILING_SCORE_8: [u32; 16] = [6, 3, 5, 4, 5, 4, 4, 5, 5, 4, 4, 5, 4, 5, 3, 6];

    const TRAILING_SCORE_7: [u32; 8] = [5, 3, 4, 4, 4, 4, 3, 5];

    const TRAILING_SCORE_6: [u32; 4] = [4, 3, 3, 4];

    const TRAILING_SCORE_5: [u32; 2] = [3, 3];

    let b = buffer & 0b11111;
    if b != 0b00000 && b != 0b11111 {
        return 0u32;
    }

    let buffer = buffer >> 4;
    match buffer_size {
        11 => TRAILING_SCORE_11[buffer as usize],
        10 => TRAILING_SCORE_10[buffer as usize],
        9 => TRAILING_SCORE_9[buffer as usize],
        8 => TRAILING_SCORE_8[buffer as usize],
        7 => TRAILING_SCORE_7[buffer as usize],
        6 => TRAILING_SCORE_6[buffer as usize],
        5 => TRAILING_SCORE_5[buffer as usize],
        _ => u8::MAX as u32,
    }
}