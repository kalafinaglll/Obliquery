pub const MAX_BUCKET_SIZE: usize = 65536;
pub const SCALE_FACTOR: u32 = 2;
pub const MODULUS: u64 = 9114733978116259; // A large prime modulus for operations
pub const MODULUS_Zmod2n: u64 = u64::pow(2,63); // 2^64 for boolean operations
pub const inverse_modulus_Zmod2n: u64 = 3074457345618258603; // modular inverse of MODULUS for 3 under 2^63
pub const SHARE_TRIPLES_U64: [(u64, u64, u64); 2] = [
    (0b1010, 0b0110, 0b1100),  // 0b1010 ^ 0b0110 ^ 0b1100 = 0
    (0b1111, 0b0001, 0b1110),  // 0b1111 ^ 0b0001 ^ 0b1110 = 0
];

pub const SHARE_TRIPLES_arithmetic: [(u64, u64, u64); 2] = [
    (23006664470262279, 16439908748041893412, 1983828661197395925),  // Just example triples
    (10385717825052324162, 17674106093612811965, 8833664228753967105),  // Just example triples
];

pub const SHARE_TRIPLES_boolean_bitwise_fake: [([bool; 64], [bool; 64], [bool; 64]); 2] = [
    (
        [false; 64], // All false, XOR is 0
        [false; 64], // All false, XOR is 0
        [false; 64], // All false, XOR is 0
    ),
    (
        [true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false],  // Two true values, XOR is 0
        [true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false],  // Two true values, XOR is 0
        [true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false],  // Two true values, XOR is 0
    )
];

pub const SHARE_TRIPLES_single_boolean: [(bool, bool, bool); 2] = [
    (
        false, false, false, 
    ),
    (
        true, false, true, 
    )
];