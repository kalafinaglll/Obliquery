use rand::Rng;

use crate::constants::inverse_modulus_Zmod2n;
use crate::constants::MODULUS_Zmod2n;
use crate::constants::SHARE_TRIPLES_boolean_bitwise_fake;
use crate::constants::SHARE_TRIPLES_single_boolean;
use crate::constants::MODULUS;
use crate::constants::SHARE_TRIPLES_U64;
use crate::constants::SHARE_TRIPLES_arithmetic;

/// A secret share in RSS3: three parties, each holds two shares.
#[derive(Clone, Debug)]
pub struct RSS3Share_arithmetic {
    pub shares: (u64, u64), // each party holds two values
}

pub struct RSS3_arithmetic {}

impl RSS3_arithmetic {
    /// Secret-share a value into three shares
    pub fn share(secret: u64) -> [RSS3Share_arithmetic; 3] {
        let (x1, x2, x3) = SHARE_TRIPLES_arithmetic[0]; // just use the first triple for simplicity
        let verify = x1.wrapping_add(x2).wrapping_add(x3) % MODULUS_Zmod2n;
        println!("verify: {:?}", verify);
        let a1 = (x3 - secret ) % MODULUS_Zmod2n;
        let a2 = (x1 - secret ) % MODULUS_Zmod2n;
        let a3 = (x2 - secret ) % MODULUS_Zmod2n;

        println!("share == x1: {}, x2: {}, x3: {}", x1, x2, x3);
        println!("share == a1: {}, a2: {}, a3: {}", a1, a2, a3);

        // replicate so each party holds two shares
        [
            RSS3Share_arithmetic { shares: (x1, a1) },
            RSS3Share_arithmetic { shares: (x2, a2) },
            RSS3Share_arithmetic { shares: (x3, a3) },
        ]
    }

    /// Reconstruct secret from three parties
    pub fn reconstruct(shares: &[RSS3Share_arithmetic; 3]) -> u64 {
        let (x1, a1) = shares[0].shares;
        let (x2, a2) = shares[1].shares;
        let (x3, a3) = shares[2].shares;
        //println!("x1: {}, x2: {}, x3: {}", x1, x2, x3);
        //println!("a1: {}, a2: {}, a3: {}", a1, a2, a3);
        MODULUS_Zmod2n.wrapping_add(x1).wrapping_sub(a2) % MODULUS_Zmod2n
        //x3.wrapping_sub(a1) % MODULUS_Zmod2n
        //MODULUS_Zmod2n.wrapping_sub((x2.wrapping_add(x1).wrapping_add(a1))) % MODULUS_Zmod2n
    }

    pub fn reconstruct_fromS0andS1(shares0: (u64,u64),shares1: (u64,u64)) -> u64{
        let (x1, a1) = shares0;
        let (x2, a2) = shares1;
        MODULUS_Zmod2n.wrapping_add(x1).wrapping_sub(a2) % MODULUS_Zmod2n

    }

    /// Addition over secret shares
    pub fn add(x: &[RSS3Share_arithmetic; 3], y: &[RSS3Share_arithmetic; 3]) -> [RSS3Share_arithmetic; 3] {
        [
            RSS3Share_arithmetic { shares: ((x[0].shares.0.wrapping_add(y[0].shares.0))% MODULUS_Zmod2n,
                                 (x[0].shares.1.wrapping_add(y[0].shares.1)) % MODULUS_Zmod2n)},
            RSS3Share_arithmetic { shares: ((x[1].shares.0.wrapping_add(y[1].shares.0)) % MODULUS_Zmod2n,
                                 (x[1].shares.1.wrapping_add(y[1].shares.1)) % MODULUS_Zmod2n)},
            RSS3Share_arithmetic { shares: ((x[2].shares.0.wrapping_add(y[2].shares.0)) % MODULUS_Zmod2n,
                                 (x[2].shares.1.wrapping_add(y[2].shares.1)) % MODULUS_Zmod2n)},
        ]
    }


    pub fn sub(x: &[RSS3Share_arithmetic; 3], y: &[RSS3Share_arithmetic; 3]) -> [RSS3Share_arithmetic; 3] {
        //println!("*******************test value: a2:{}", MODULUS_Zmod2n.wrapping_add(x[1].shares.1).wrapping_sub(y[1].shares.1) % MODULUS_Zmod2n);

        [
            RSS3Share_arithmetic {
                shares: (
                    (MODULUS_Zmod2n.wrapping_add(x[0].shares.0).wrapping_sub(y[0].shares.0)) % MODULUS_Zmod2n,
                    (MODULUS_Zmod2n.wrapping_add(x[0].shares.1).wrapping_sub(y[0].shares.1)) % MODULUS_Zmod2n
                )
            },
            RSS3Share_arithmetic {
                shares: (
                    (MODULUS_Zmod2n.wrapping_add(x[1].shares.0).wrapping_sub(y[1].shares.0)) % MODULUS_Zmod2n,
                    (MODULUS_Zmod2n.wrapping_add(x[1].shares.1).wrapping_sub(y[1].shares.1)) % MODULUS_Zmod2n
                )
            },
            RSS3Share_arithmetic {
                shares: (
                    (MODULUS_Zmod2n.wrapping_add(x[2].shares.0).wrapping_sub(y[2].shares.0)) % MODULUS_Zmod2n,
                    (MODULUS_Zmod2n.wrapping_add(x[2].shares.1).wrapping_sub(y[2].shares.1)) % MODULUS_Zmod2n
                )
            },
        ]
    }

    pub fn mul(v1: &[RSS3Share_arithmetic; 3], v2: &[RSS3Share_arithmetic; 3]) -> [RSS3Share_arithmetic; 3] {
        // Placeholder for multiplication logic
        // compute 2/3-sharing of the product of two values
        let (alpha, beta, gamma) = SHARE_TRIPLES_arithmetic[0]; // just use the first triple for simplicity
        let (x1,a1) = v1[0].shares; // p1 v1
        let (x2,a2) = v1[1].shares; // p2 v1
        let (x3,a3) = v1[2].shares; // p3 v1
        let (y1,b1) = v2[0].shares; // p1 v2
        let (y2,b2) = v2[1].shares; // p2 v2
        let (y3,b3) = v2[2].shares; // p3 v2

        let r1 = (a1.wrapping_mul(b1).wrapping_sub(x1.wrapping_mul(y1).wrapping_add(alpha))).wrapping_mul(inverse_modulus_Zmod2n) % MODULUS_Zmod2n; // P1 sends to P2
        let r2 = (a2.wrapping_mul(b2).wrapping_sub(x2.wrapping_mul(y2).wrapping_add(beta))).wrapping_mul(inverse_modulus_Zmod2n) % MODULUS_Zmod2n; // P2 sends to P3
        let r3 = (a3.wrapping_mul(b3).wrapping_sub(x3.wrapping_mul(y3).wrapping_add(gamma))).wrapping_mul(inverse_modulus_Zmod2n) % MODULUS_Zmod2n; // P3 sends to P1

        //p1 define r1, r3
        let z1 = r3.wrapping_sub(r1) % MODULUS_Zmod2n;
        let c1 = (MODULUS_Zmod2n.wrapping_sub(2*r3).wrapping_sub(r1)) % MODULUS_Zmod2n;
        //p2 define r2, r1
        let z2 = r1.wrapping_sub(r2) % MODULUS_Zmod2n;
        let c2 = (MODULUS_Zmod2n.wrapping_sub(2*r1).wrapping_sub(r2)) % MODULUS_Zmod2n;
        //p3 define r3, r2
        let z3 = r2.wrapping_sub(r3) % MODULUS_Zmod2n;
        let c3 = (MODULUS_Zmod2n.wrapping_sub(2*r2).wrapping_sub(r3)) % MODULUS_Zmod2n;

        [
            RSS3Share_arithmetic { shares: (z1,c1)}, // p1 (zi,ci)
            RSS3Share_arithmetic { shares: (z2,c2)}, // p2 (zi,ci)
            RSS3Share_arithmetic { shares: (z3,c3)}, // p3 (zi,ci)
        ]   


    }
}


pub struct RSS3Share_boolean {
    pub shares: (u64, u64), // each party holds two values
}

pub struct RSS3_boolean {}

impl RSS3_boolean {
    /// Secret-share a value into three shares
    pub fn share(secret: u64) -> [RSS3Share_boolean; 3] {
        let (x1, x2, x3) = SHARE_TRIPLES_U64[0]; // just use the first triple for simplicity
        //let verify = x1 ^ x2 ^ x3;
        //println!("verify: {:04b}", verify);
        let a1 = x3 ^ secret;
        let a2 = x1 ^ secret;
        let a3 = x2 ^ secret;
        // replicate so each party holds two shares
        [
            RSS3Share_boolean { shares: (x1, a1) }, //p1
            RSS3Share_boolean { shares: (x2, a2) }, //p2
            RSS3Share_boolean { shares: (x3, a3) }, //p3
        ]
    }

    /// Reconstruct secret from three parties
    pub fn reconstruct(shares: &[RSS3Share_boolean; 3]) -> u64 {
        let (x1, _) = shares[0].shares;
        let (_, a2) = shares[1].shares;
        a2 ^ x1
    }

    pub fn reconstruct_2_test(shares: &[RSS3Share_boolean; 3]) -> u64 {
        let (x2, _) = shares[1].shares;
        let (_, a3) = shares[2].shares;
        a3 ^ x2
    }

    /// Addition over secret shares
    pub fn xor(v1: &[RSS3Share_boolean; 3], v2: &[RSS3Share_boolean; 3]) -> [RSS3Share_boolean; 3] {
        [
            RSS3Share_boolean { shares: (v1[0].shares.0 ^ v2[0].shares.0,
                                 v1[0].shares.1 ^ v2[0].shares.1)}, // p1 (zi,ci)
            RSS3Share_boolean { shares: (v1[1].shares.0 ^ v2[1].shares.0,
                                 v1[1].shares.1 ^ v2[1].shares.1)}, // p2 (zi,ci)
            RSS3Share_boolean { shares: (v1[2].shares.0 ^ v2[2].shares.0,
                                 v1[2].shares.1 ^ v2[2].shares.1)}, // p3 (zi,ci)
        ]
    }
    pub fn and(v1: &[RSS3Share_boolean; 3], v2: &[RSS3Share_boolean; 3]) -> [RSS3Share_boolean; 3] {
        // Placeholder for multiplication logic
        //unimplemented!("Multiplication is not implemented yet");
        //first step: compute 3/3 xor-sharing of the AND of the input bits
        let (alpha, beta, gamma) = SHARE_TRIPLES_U64[0]; // just use the first triple for simplicity

        let (x1, a1) = v1[0].shares; // p1 v1
        let (y1, b1) = v2[0].shares; // p1 v2
        let r1 = (x1 & y1) ^ (a1 & b1) ^ alpha; // P1 sends to P2

        let (x2, a2) = v1[1].shares; // p2 v1
        let (y2, b2) = v2[1].shares; // p2 v2
        let r2 = (x2 & y2) ^ (a2 & b2) ^ beta; // P2 sends to P3

        let (x3, a3) = v1[2].shares; // p3 v1
        let (y3, b3) = v2[2].shares; // p3 v2
        let r3 = (x3 & y3) ^ (a3 & b3) ^ gamma; // P3 sends to P1

        //second step: compute 2/3-sharing
        let z1 = r1 ^ r3;
        let z2 = r2 ^ r1;
        let z3 = r3 ^ r2;

        let c1 = r1;
        let c2 = r2;
        let c3 = r3;

        [
            RSS3Share_boolean { shares: (z1,c1)}, // p1 (zi,ci)
            RSS3Share_boolean { shares: (z2,c2)}, // p2 (zi,ci)
            RSS3Share_boolean { shares: (z3,c3)}, // p3 (zi,ci)
        ]

    }
}


#[derive(Clone)]
pub struct RSS3Share_single_boolean {
    pub shares: (bool, bool), // each party holds two values
}

pub struct RSS3_single_boolean {}

impl RSS3_single_boolean {
    /// Secret-share a value into three shares
    pub fn share(secret: bool) -> [RSS3Share_single_boolean; 3] {
        let (x1, x2, x3) = SHARE_TRIPLES_single_boolean[0]; // just use the first triple for simplicity
        //let verify = x1 ^ x2 ^ x3;
        //println!("verify: {:04b}", verify);
        let a1 = x3 ^ secret;
        let a2 = x1 ^ secret;
        let a3 = x2 ^ secret;
        // replicate so each party holds two shares
        [
            RSS3Share_single_boolean { shares: (x1, a1) }, //p1
            RSS3Share_single_boolean { shares: (x2, a2) }, //p2
            RSS3Share_single_boolean { shares: (x3, a3) }, //p3
        ]
    }

    pub fn one_share() -> [RSS3Share_single_boolean; 3] {
        let (x1, x2, x3) = SHARE_TRIPLES_single_boolean[1]; // just use the first triple for simplicity
        //let verify = x1 ^ x2 ^ x3;
        //println!("verify: {:04b}", verify);
        let a1 = x3 ^ true;
        let a2 = x1 ^ true;
        let a3 = x2 ^ true;
        // replicate so each party holds two shares
        [
            RSS3Share_single_boolean { shares: (x1, a1) }, //p1
            RSS3Share_single_boolean { shares: (x2, a2) }, //p2
            RSS3Share_single_boolean { shares: (x3, a3) }, //p3
        ]
    }

    /// Reconstruct secret from three parties
    pub fn reconstruct(shares: &[RSS3Share_single_boolean; 3]) -> bool {
        let (x1, _) = shares[0].shares;
        let (_, a2) = shares[1].shares;
        a2 ^ x1
    }

    /// Addition over secret shares
    pub fn xor(v1: &[RSS3Share_single_boolean; 3], v2: &[RSS3Share_single_boolean; 3]) -> [RSS3Share_single_boolean; 3] {
        [
            RSS3Share_single_boolean { shares: (v1[0].shares.0 ^ v2[0].shares.0,
                                 v1[0].shares.1 ^ v2[0].shares.1)}, // p1 (zi,ci)
            RSS3Share_single_boolean { shares: (v1[1].shares.0 ^ v2[1].shares.0,
                                 v1[1].shares.1 ^ v2[1].shares.1)}, // p2 (zi,ci)
            RSS3Share_single_boolean { shares: (v1[2].shares.0 ^ v2[2].shares.0,
                                 v1[2].shares.1 ^ v2[2].shares.1)}, // p3 (zi,ci)
        ]
    }
    pub fn and(v1: &[RSS3Share_single_boolean; 3], v2: &[RSS3Share_single_boolean; 3]) -> [RSS3Share_single_boolean; 3] {
        // Placeholder for multiplication logic
        //unimplemented!("Multiplication is not implemented yet");
        //first step: compute 3/3 xor-sharing of the AND of the input bits
        let (alpha, beta, gamma) = SHARE_TRIPLES_single_boolean[0]; // just use the first triple for simplicity

        let (x1, a1) = v1[0].shares; // p1 v1
        let (y1, b1) = v2[0].shares; // p1 v2
        let r1 = (x1 & y1) ^ (a1 & b1) ^ alpha; // P1 sends to P2

        let (x2, a2) = v1[1].shares; // p2 v1
        let (y2, b2) = v2[1].shares; // p2 v2
        let r2 = (x2 & y2) ^ (a2 & b2) ^ beta; // P2 sends to P3

        let (x3, a3) = v1[2].shares; // p3 v1
        let (y3, b3) = v2[2].shares; // p3 v2
        let r3 = (x3 & y3) ^ (a3 & b3) ^ gamma; // P3 sends to P1

        //second step: compute 2/3-sharing
        let z1 = r1 ^ r3;
        let z2 = r2 ^ r1;
        let z3 = r3 ^ r2;

        let c1 = r1;
        let c2 = r2;
        let c3 = r3;

        [
            RSS3Share_single_boolean { shares: (z1,c1)}, // p1 (zi,ci)
            RSS3Share_single_boolean { shares: (z2,c2)}, // p2 (zi,ci)
            RSS3Share_single_boolean { shares: (z3,c3)}, // p3 (zi,ci)
        ]

    }
    pub fn not(v:&[RSS3Share_single_boolean; 3]) -> [RSS3Share_single_boolean; 3] {
        let one = Self::one_share();
        Self::xor(v, &one)
    }
}



pub struct RSS3Share_boolean_bitwise {
    pub shares: ([bool;64], [bool;64]), // each party holds two values
}
pub struct RSS3S_boolean_bitwise {}

impl RSS3S_boolean_bitwise {
    /// Secret-share a value into three shares
    pub fn share(secret: u64) -> [RSS3Share_boolean_bitwise; 3] {
        let (x1, x2, x3) = SHARE_TRIPLES_boolean_bitwise_fake[0]; // just use the first triple for simplicity
        // let verify: [bool; 64] = x1.iter().zip(x2.iter().zip(x3.iter()))
        //     .map(|(b1, (b2, b3))| *b1 ^ *b2 ^ *b3)
        //     .collect::<Vec<bool>>()
        //     .try_into()
        //     .expect("Failed to convert Vec<bool> to [bool; 64]");
        // println!("verify bitwise triples: {:?}", verify);
        let secret_bits = Self::u64_to_bits(secret);
        let a1 = Self::bool_xor(&x3, &secret_bits);
        let a2 = Self::bool_xor(&x1,&secret_bits);
        let a3 = Self::bool_xor(&x2,&secret_bits);
        // replicate so each party holds two shares
        [
            RSS3Share_boolean_bitwise { shares: (x1, a1) }, //p1
            RSS3Share_boolean_bitwise { shares: (x2, a2) }, //p2
            RSS3Share_boolean_bitwise { shares: (x3, a3) }, //p3
        ]
    }

    fn u64_to_bits(value: u64) -> [bool; 64] {
    let mut bits = [false; 64];
    for i in 0..64 {
        bits[i] = (value & (1 << i)) != 0;
    }
    bits
    }

    fn bits_to_u64(bits: &[bool; 64]) -> u64 {
    let mut value = 0u64;
    for i in 0..64 {
        if bits[i] {
            value |= 1 << i;
        }
    }
    value
}

    fn bool_xor(a: &[bool; 64], b: &[bool; 64]) -> [bool; 64] {
    let mut result = [false; 64];
    for i in 0..64 {
        result[i] = a[i] ^ b[i];
    }
    result
    }

    fn bool_xor_three(a: &[bool; 64], b: &[bool; 64], c: &[bool; 64]) -> [bool; 64] {
    let mut result = [false; 64];
    for i in 0..64 {
        result[i] = a[i] ^ b[i] ^ c[i];
    }
    result
    }

    fn bool_and(a: &[bool; 64], b: &[bool; 64]) -> [bool; 64] {
    let mut result = [false; 64];
    for i in 0..64 {
        result[i] = a[i] & b[i];
    }
    result
    }


    /// Reconstruct secret from three parties
    pub fn reconstruct(shares: &[RSS3Share_boolean_bitwise; 3]) -> u64 {
        let (x1, _) = shares[0].shares;
        let (_, a2) = shares[1].shares;
        let result_bits = Self::bool_xor(&x1, &a2);
        Self::bits_to_u64(&result_bits)
    }

    /// Addition over secret shares
    pub fn xor(v1: &[RSS3Share_boolean_bitwise; 3], v2: &[RSS3Share_boolean_bitwise; 3]) -> [RSS3Share_boolean_bitwise; 3] {
        [
            RSS3Share_boolean_bitwise { shares: (Self::bool_xor(&v1[0].shares.0, &v2[0].shares.0),
                                 Self::bool_xor(&v1[0].shares.1, &v2[0].shares.1))}, // p1 (zi,ci)
            RSS3Share_boolean_bitwise { shares: (Self::bool_xor(&v1[1].shares.0, &v2[1].shares.0),
                                 Self::bool_xor(&v1[1].shares.1,&v2[1].shares.1))}, // p2 (zi,ci)
            RSS3Share_boolean_bitwise { shares: (Self::bool_xor(&v1[2].shares.0,&v2[2].shares.0),
                                 Self::bool_xor(&v1[2].shares.1,&v2[2].shares.1))}, // p3 (zi,ci)
        ]
    }
    pub fn and(v1: &[RSS3Share_boolean_bitwise; 3], v2: &[RSS3Share_boolean_bitwise; 3]) -> [RSS3Share_boolean_bitwise; 3] {
        // Placeholder for multiplication logic
        //unimplemented!("Multiplication is not implemented yet");
        //first step: compute 3/3 xor-sharing of the AND of the input bits
        let (alpha, beta, gamma) = SHARE_TRIPLES_boolean_bitwise_fake[0]; // just use the first triple for simplicity

        let (x1, a1) = v1[0].shares; // p1 v1
        let (y1, b1) = v2[0].shares; // p1 v2
        let r1 = Self::bool_xor_three(&Self::bool_and(&x1,&y1), &Self::bool_and(&a1, &b1), &alpha); // P1 sends to P2

        let (x2, a2) = v1[1].shares; // p2 v1
        let (y2, b2) = v2[1].shares; // p2 v2
        let r2 = Self::bool_xor_three(&Self::bool_and(&x2,&y2), &Self::bool_and(&a2, &b2), &beta);  // P2 sends to P3

        let (x3, a3) = v1[2].shares; // p3 v1
        let (y3, b3) = v2[2].shares; // p3 v2
        let r3 = Self::bool_xor_three(&Self::bool_and(&x3,&y3), &Self::bool_and(&a3, &b3), &gamma); // P3 sends to P1

        //second step: compute 2/3-sharing
        let z1 = Self::bool_xor(&r1, &r3);
        let z2 = Self::bool_xor(&r2,&r1);
        let z3 = Self::bool_xor(&r3,&r2);

        let c1 = r1;
        let c2 = r2;
        let c3 = r3;

        [
            RSS3Share_boolean_bitwise { shares: (z1,c1)}, // p1 (zi,ci)
            RSS3Share_boolean_bitwise { shares: (z2,c2)}, // p2 (zi,ci)
            RSS3Share_boolean_bitwise { shares: (z3,c3)}, // p3 (zi,ci)
        ]

    }

    fn RSS3Share_boolean_bitwise64bits_to_SingleBooleanArray(v: &[RSS3Share_boolean_bitwise; 3]) -> [[RSS3Share_single_boolean; 3];64] {

        std::array::from_fn(|i| {
            [
                RSS3Share_single_boolean { shares: (v[0].shares.0[i], v[0].shares.1[i]) },
                RSS3Share_single_boolean { shares: (v[1].shares.0[i], v[1].shares.1[i]) },
                RSS3Share_single_boolean { shares: (v[2].shares.0[i], v[2].shares.1[i]) },
            ]
        })

    }

    fn shares_bit_and(v: &[RSS3Share_boolean_bitwise; 3]) -> [RSS3Share_single_boolean; 3] {
        let RSS3Share_single_boolean_array = Self::RSS3Share_boolean_bitwise64bits_to_SingleBooleanArray(v);

        let mut result = &RSS3Share_single_boolean_array[0];
        for i in 0..63{
            let result = RSS3_single_boolean::and(&RSS3Share_single_boolean_array[i], &RSS3Share_single_boolean_array[i+1]);
            
        }
        result.clone()

    }

    pub fn equal(v1: &[RSS3Share_boolean_bitwise; 3], v2: &[RSS3Share_boolean_bitwise; 3]) -> [RSS3Share_single_boolean; 3]{

        let diff = Self::xor(v1, v2);
        let and_result = Self::shares_bit_and(&diff);
        let result = RSS3_single_boolean::not(&and_result);
        result

    }

}

