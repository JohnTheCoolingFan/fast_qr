//! Contains all different levels of quality.
//! And allows to find easily max bits per version/quality pair

/// Error Correction Coding has 4 levels

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum ECL {
    /// Low, 7%
    L,
    /// Medium, 15%
    M,
    /// Quartile, 25%
    Q,
    /// High, 30%
    H,
}

/// Contains the max bits for any version of `LOW` ECC
const L_DATABITS: [u16; 41] = [
    0, 152, 272, 440, 640, 864, 1088, 1248, 1552, 1856, 2192, 2592, 2960, 3424, 3688, 4184, 4712,
    5176, 5768, 6360, 6888, 7456, 8048, 8752, 9392, 10208, 10960, 11744, 12248, 13048, 13880,
    14744, 15640, 16568, 17528, 18448, 19472, 20528, 21616, 22496, 23648,
];

/// Contains the max bits for any version of `MED` ECC
const M_DATABITS: [u16; 41] = [
    0, 128, 224, 352, 512, 688, 864, 992, 1232, 1456, 1728, 2032, 2320, 2672, 2920, 3320, 3624,
    4056, 4504, 5016, 5352, 5712, 6256, 6880, 7312, 8000, 8496, 9024, 9544, 10136, 10984, 11640,
    12328, 13048, 13800, 14496, 15312, 15936, 16816, 17728, 18672,
];

/// Contains the max bits for any version of `QAR` ECC
const Q_DATABITS: [u16; 41] = [
    0, 104, 176, 272, 384, 496, 608, 704, 880, 1056, 1232, 1440, 1648, 1952, 2088, 2360, 2600,
    2936, 3176, 3560, 3880, 4096, 4544, 4912, 5312, 5744, 6032, 6464, 6968, 7288, 7880, 8264, 8920,
    9368, 9848, 10288, 10832, 11408, 12016, 12656, 13328,
];

/// Contains the max bits for any version of `HIG` ECC
const H_DATABITS: [u16; 41] = [
    0, 72, 128, 208, 288, 368, 480, 528, 688, 800, 976, 1120, 1264, 1440, 1576, 1784, 2024, 2264,
    2504, 2728, 3080, 3248, 3536, 3712, 4112, 4304, 4768, 5024, 5288, 5608, 5960, 6344, 6760, 7208,
    7688, 7888, 8432, 8768, 9136, 9776, 10208,
];

/// Fetches the right array to retrieve number of databits
pub const fn ecc_to_databits(quality: ECL, version: usize) -> u16 {
    return match quality {
        ECL::L => L_DATABITS[version],
        ECL::M => M_DATABITS[version],
        ECL::Q => Q_DATABITS[version],
        ECL::H => H_DATABITS[version],
    };
}

/// Contains the number of Data Codewords for a version at level L
const L_ECT: [usize; 41] = [
    0, 7, 10, 15, 20, 26, 18, 20, 24, 30, 18, 20, 24, 26, 30, 22, 24, 28, 30, 28, 28, 28, 28, 30,
    30, 26, 28, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
];

/// Contains the number of Data Codewords for a version at level M
const M_ECT: [usize; 41] = [
    0, 10, 16, 26, 18, 24, 16, 18, 22, 22, 26, 30, 22, 22, 24, 24, 28, 28, 26, 26, 26, 26, 28, 28,
    28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28,
];

/// Contains the number of Data Codewords for a version at level Q
const Q_ECT: [usize; 41] = [
    0, 13, 22, 18, 26, 18, 24, 18, 22, 20, 24, 28, 26, 24, 20, 30, 24, 28, 28, 26, 30, 28, 30, 30,
    30, 30, 28, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
];

/// Contains the number of Data Codewords for a version at level H
const H_ECT: [usize; 41] = [
    0, 17, 28, 22, 16, 22, 28, 26, 26, 24, 28, 24, 28, 22, 24, 24, 30, 28, 28, 26, 28, 30, 24, 30,
    30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
];

/// Fetches the right array to retrieve number of error correction code words
pub const fn ecc_to_ect(quality: ECL, version: usize) -> usize {
    return match quality {
        ECL::L => L_ECT[version],
        ECL::M => M_ECT[version],
        ECL::Q => Q_ECT[version],
        ECL::H => H_ECT[version],
    };
}

/// Contains information on groups required to encoded at level L
/// ```
/// 00000000 | 00000000 | 00000000 | 00000000
///    ^^         ^^         ^^         ^^
/// G1_count | G1_size  | G2_count | G2_size
/// ```
/// So the total of [Data Codewords](https://www.thonky.com/qr-code-tutorial/error-correction-table) is `(G1_count * G1_size) + (G2_count * G2_size)`
const L_GROUPS: [u32; 41] = [
    (0 << 24) | (0 << 16) | (0 << 8) | 0,
    (1 << 24) | (19 << 16) | (0 << 8) | 0,
    (1 << 24) | (34 << 16) | (0 << 8) | 0,
    (1 << 24) | (55 << 16) | (0 << 8) | 0,
    (1 << 24) | (80 << 16) | (0 << 8) | 0,
    (1 << 24) | (108 << 16) | (0 << 8) | 0,
    (2 << 24) | (68 << 16) | (0 << 8) | 0,
    (2 << 24) | (78 << 16) | (0 << 8) | 0,
    (2 << 24) | (97 << 16) | (0 << 8) | 0,
    (2 << 24) | (116 << 16) | (0 << 8) | 0,
    (2 << 24) | (68 << 16) | (2 << 8) | 69,
    (4 << 24) | (81 << 16) | (0 << 8) | 0,
    (2 << 24) | (92 << 16) | (2 << 8) | 93,
    (4 << 24) | (107 << 16) | (0 << 8) | 0,
    (3 << 24) | (115 << 16) | (1 << 8) | 116,
    (5 << 24) | (87 << 16) | (1 << 8) | 88,
    (5 << 24) | (98 << 16) | (1 << 8) | 99,
    (1 << 24) | (107 << 16) | (5 << 8) | 108,
    (5 << 24) | (120 << 16) | (1 << 8) | 121,
    (3 << 24) | (113 << 16) | (4 << 8) | 114,
    (3 << 24) | (107 << 16) | (5 << 8) | 108,
    (4 << 24) | (116 << 16) | (4 << 8) | 117,
    (2 << 24) | (111 << 16) | (7 << 8) | 112,
    (4 << 24) | (121 << 16) | (5 << 8) | 122,
    (6 << 24) | (117 << 16) | (4 << 8) | 118,
    (8 << 24) | (106 << 16) | (4 << 8) | 107,
    (10 << 24) | (114 << 16) | (2 << 8) | 115,
    (8 << 24) | (122 << 16) | (4 << 8) | 123,
    (3 << 24) | (117 << 16) | (10 << 8) | 118,
    (7 << 24) | (116 << 16) | (7 << 8) | 117,
    (5 << 24) | (115 << 16) | (10 << 8) | 116,
    (13 << 24) | (115 << 16) | (3 << 8) | 116,
    (17 << 24) | (115 << 16) | (0 << 8) | 0,
    (17 << 24) | (115 << 16) | (1 << 8) | 116,
    (13 << 24) | (115 << 16) | (6 << 8) | 116,
    (12 << 24) | (121 << 16) | (7 << 8) | 122,
    (6 << 24) | (121 << 16) | (14 << 8) | 122,
    (17 << 24) | (122 << 16) | (4 << 8) | 123,
    (4 << 24) | (122 << 16) | (18 << 8) | 123,
    (20 << 24) | (117 << 16) | (4 << 8) | 118,
    (19 << 24) | (118 << 16) | (6 << 8) | 119,
];

/// Contains information on groups required to encoded at level M
/// ```
/// 00000000 | 00000000 | 00000000 | 00000000
///    ^^         ^^         ^^         ^^
/// G1_count | G1_size  | G2_count | G2_size
/// ```
/// So the total of [Data Codewords](https://www.thonky.com/qr-code-tutorial/error-correction-table) is `(G1_count * G1_size) + (G2_count * G2_size)`
const M_GROUPS: [u32; 41] = [
    (0 << 24) | (0 << 16) | (0 << 8) | 0,
    (1 << 24) | (16 << 16) | (0 << 8) | 0,
    (1 << 24) | (28 << 16) | (0 << 8) | 0,
    (1 << 24) | (44 << 16) | (0 << 8) | 0,
    (2 << 24) | (32 << 16) | (0 << 8) | 0,
    (2 << 24) | (43 << 16) | (0 << 8) | 0,
    (4 << 24) | (27 << 16) | (0 << 8) | 0,
    (4 << 24) | (31 << 16) | (0 << 8) | 0,
    (2 << 24) | (38 << 16) | (2 << 8) | 39,
    (3 << 24) | (36 << 16) | (2 << 8) | 37,
    (4 << 24) | (43 << 16) | (1 << 8) | 44,
    (1 << 24) | (50 << 16) | (4 << 8) | 51,
    (6 << 24) | (36 << 16) | (2 << 8) | 37,
    (8 << 24) | (37 << 16) | (1 << 8) | 38,
    (4 << 24) | (40 << 16) | (5 << 8) | 41,
    (5 << 24) | (41 << 16) | (5 << 8) | 42,
    (7 << 24) | (45 << 16) | (3 << 8) | 46,
    (10 << 24) | (46 << 16) | (1 << 8) | 47,
    (9 << 24) | (43 << 16) | (4 << 8) | 44,
    (3 << 24) | (44 << 16) | (11 << 8) | 45,
    (3 << 24) | (41 << 16) | (13 << 8) | 42,
    (17 << 24) | (42 << 16) | (0 << 8) | 0,
    (17 << 24) | (46 << 16) | (0 << 8) | 0,
    (4 << 24) | (47 << 16) | (14 << 8) | 48,
    (6 << 24) | (45 << 16) | (14 << 8) | 46,
    (8 << 24) | (47 << 16) | (13 << 8) | 48,
    (19 << 24) | (46 << 16) | (4 << 8) | 47,
    (22 << 24) | (45 << 16) | (3 << 8) | 46,
    (3 << 24) | (45 << 16) | (23 << 8) | 46,
    (21 << 24) | (45 << 16) | (7 << 8) | 46,
    (19 << 24) | (47 << 16) | (10 << 8) | 48,
    (2 << 24) | (46 << 16) | (29 << 8) | 47,
    (10 << 24) | (46 << 16) | (23 << 8) | 47,
    (14 << 24) | (46 << 16) | (21 << 8) | 47,
    (14 << 24) | (46 << 16) | (23 << 8) | 47,
    (12 << 24) | (47 << 16) | (26 << 8) | 48,
    (6 << 24) | (47 << 16) | (34 << 8) | 48,
    (29 << 24) | (46 << 16) | (14 << 8) | 47,
    (13 << 24) | (46 << 16) | (32 << 8) | 47,
    (40 << 24) | (47 << 16) | (7 << 8) | 48,
    (18 << 24) | (47 << 16) | (31 << 8) | 48,
];

/// Contains information on groups required to encoded at level Q
/// ```
/// 00000000 | 00000000 | 00000000 | 00000000
///    ^^         ^^         ^^         ^^
/// G1_count | G1_size  | G2_count | G2_size
/// ```
/// So the total of [Data Codewords](https://www.thonky.com/qr-code-tutorial/error-correction-table) is `(G1_count * G1_size) + (G2_count * G2_size)`
const Q_GROUPS: [u32; 41] = [
    (0 << 24) | (0 << 16) | (0 << 8) | 0,
    (1 << 24) | (13 << 16) | (0 << 8) | 0,
    (1 << 24) | (22 << 16) | (0 << 8) | 0,
    (2 << 24) | (17 << 16) | (0 << 8) | 0,
    (2 << 24) | (24 << 16) | (0 << 8) | 0,
    (2 << 24) | (15 << 16) | (2 << 8) | 16,
    (4 << 24) | (19 << 16) | (0 << 8) | 0,
    (2 << 24) | (14 << 16) | (4 << 8) | 15,
    (4 << 24) | (18 << 16) | (2 << 8) | 19,
    (4 << 24) | (16 << 16) | (4 << 8) | 17,
    (6 << 24) | (19 << 16) | (2 << 8) | 20,
    (4 << 24) | (22 << 16) | (4 << 8) | 23,
    (4 << 24) | (20 << 16) | (6 << 8) | 21,
    (8 << 24) | (20 << 16) | (4 << 8) | 21,
    (11 << 24) | (16 << 16) | (5 << 8) | 17,
    (5 << 24) | (24 << 16) | (7 << 8) | 25,
    (15 << 24) | (19 << 16) | (2 << 8) | 20,
    (1 << 24) | (22 << 16) | (15 << 8) | 23,
    (17 << 24) | (22 << 16) | (1 << 8) | 23,
    (17 << 24) | (21 << 16) | (4 << 8) | 22,
    (15 << 24) | (24 << 16) | (5 << 8) | 25,
    (17 << 24) | (22 << 16) | (6 << 8) | 23,
    (7 << 24) | (24 << 16) | (16 << 8) | 25,
    (11 << 24) | (24 << 16) | (14 << 8) | 25,
    (11 << 24) | (24 << 16) | (16 << 8) | 25,
    (7 << 24) | (24 << 16) | (22 << 8) | 25,
    (28 << 24) | (22 << 16) | (6 << 8) | 23,
    (8 << 24) | (23 << 16) | (26 << 8) | 24,
    (4 << 24) | (24 << 16) | (31 << 8) | 25,
    (1 << 24) | (23 << 16) | (37 << 8) | 24,
    (15 << 24) | (24 << 16) | (25 << 8) | 25,
    (42 << 24) | (24 << 16) | (1 << 8) | 25,
    (10 << 24) | (24 << 16) | (35 << 8) | 25,
    (29 << 24) | (24 << 16) | (19 << 8) | 25,
    (44 << 24) | (24 << 16) | (7 << 8) | 25,
    (39 << 24) | (24 << 16) | (14 << 8) | 25,
    (46 << 24) | (24 << 16) | (10 << 8) | 25,
    (49 << 24) | (24 << 16) | (10 << 8) | 25,
    (48 << 24) | (24 << 16) | (14 << 8) | 25,
    (43 << 24) | (24 << 16) | (22 << 8) | 25,
    (34 << 24) | (24 << 16) | (34 << 8) | 25,
];

/// Contains information on groups required to encoded at level H
/// ```
/// 00000000 | 00000000 | 00000000 | 00000000
///    ^^         ^^         ^^         ^^
/// G1_count | G1_size  | G2_count | G2_size
/// ```
/// So the total of [Data Codewords](https://www.thonky.com/qr-code-tutorial/error-correction-table) is `(G1_count * G1_size) + (G2_count * G2_size)`
const H_GROUPS: [u32; 41] = [
    (0 << 24) | (0 << 16) | (0 << 8) | 0,
    (1 << 24) | (9 << 16) | (0 << 8) | 0,
    (1 << 24) | (16 << 16) | (0 << 8) | 0,
    (2 << 24) | (13 << 16) | (0 << 8) | 0,
    (4 << 24) | (9 << 16) | (0 << 8) | 0,
    (2 << 24) | (11 << 16) | (2 << 8) | 12,
    (4 << 24) | (15 << 16) | (0 << 8) | 0,
    (4 << 24) | (13 << 16) | (1 << 8) | 14,
    (4 << 24) | (14 << 16) | (2 << 8) | 15,
    (4 << 24) | (12 << 16) | (4 << 8) | 13,
    (6 << 24) | (15 << 16) | (2 << 8) | 16,
    (3 << 24) | (12 << 16) | (8 << 8) | 13,
    (7 << 24) | (14 << 16) | (4 << 8) | 15,
    (12 << 24) | (11 << 16) | (4 << 8) | 12,
    (11 << 24) | (12 << 16) | (5 << 8) | 13,
    (11 << 24) | (12 << 16) | (7 << 8) | 13,
    (3 << 24) | (15 << 16) | (13 << 8) | 16,
    (2 << 24) | (14 << 16) | (17 << 8) | 15,
    (2 << 24) | (14 << 16) | (19 << 8) | 15,
    (9 << 24) | (13 << 16) | (16 << 8) | 14,
    (15 << 24) | (15 << 16) | (10 << 8) | 16,
    (19 << 24) | (16 << 16) | (6 << 8) | 17,
    (34 << 24) | (13 << 16) | (0 << 8) | 0,
    (16 << 24) | (15 << 16) | (14 << 8) | 16,
    (30 << 24) | (16 << 16) | (2 << 8) | 17,
    (22 << 24) | (15 << 16) | (13 << 8) | 16,
    (33 << 24) | (16 << 16) | (4 << 8) | 17,
    (12 << 24) | (15 << 16) | (28 << 8) | 16,
    (11 << 24) | (15 << 16) | (31 << 8) | 16,
    (19 << 24) | (15 << 16) | (26 << 8) | 16,
    (23 << 24) | (15 << 16) | (25 << 8) | 16,
    (23 << 24) | (15 << 16) | (28 << 8) | 16,
    (19 << 24) | (15 << 16) | (35 << 8) | 16,
    (11 << 24) | (15 << 16) | (46 << 8) | 16,
    (59 << 24) | (16 << 16) | (1 << 8) | 17,
    (22 << 24) | (15 << 16) | (41 << 8) | 16,
    (2 << 24) | (15 << 16) | (64 << 8) | 16,
    (24 << 24) | (15 << 16) | (46 << 8) | 16,
    (42 << 24) | (15 << 16) | (32 << 8) | 16,
    (10 << 24) | (15 << 16) | (67 << 8) | 16,
    (20 << 24) | (15 << 16) | (61 << 8) | 16,
];

/// Fetches the right array to retrieve the information on groups
/// However, since we store it as u32, we decompose the information
/// in groups of four u8 as described below to have easier access to
/// information
/// ```
/// 00000000 | 00000000 | 00000000 | 00000000
///    ^^         ^^         ^^         ^^
/// G1_count | G1_size  | G2_count | G2_size
///
/// // converts to
///
/// [(G1_count, G1_size), (G2_count, G2_size)]
/// ```
pub const fn ecc_to_groups(quality: ECL, version: usize) -> [(usize, usize); 2] {
    let groups_bits = match quality {
        ECL::L => L_GROUPS[version],
        ECL::M => M_GROUPS[version],
        ECL::Q => Q_GROUPS[version],
        ECL::H => H_GROUPS[version],
    };

    // Removes front bits
    let grp1 = (groups_bits >> 24) & 0xFF;
    let grp2 = (groups_bits >> 16) & 0xFF;
    let grp3 = (groups_bits >> 08) & 0xFF;
    let grp4 = (groups_bits >> 00) & 0xFF;

    return [
        (grp1 as usize, grp2 as usize),
        (grp3 as usize, grp4 as usize),
    ];
}

const MISSING_BITS: [u8; 41] = [
    0, 0, 7, 7, 7, 7, 7, 0, 0, 0, 0, 0, 0, 0, 3, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 3, 3, 3, 3,
    3, 3, 3, 0, 0, 0, 0, 0, 0,
];

pub const fn version_missing_bits(version: usize) -> u8 {
    return MISSING_BITS[version];
}
