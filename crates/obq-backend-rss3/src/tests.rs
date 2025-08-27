use crate::constants::MODULUS_Zmod2n;
use crate::rss3::RSS3S_boolean_bitwise;
use crate::rss3::RSS3_single_boolean;

use super::rss3::RSS3_arithmetic;
use super::rss3::RSS3_boolean;

#[test]
fn test_rss3_share_and_reconstruct() {
    println!("Sharesxxxxxxxxxxxxxxxxxxxxxxxxxx");
    let secret = 12345;
    // let test = u64::pow(2, 63) - 9223372036854763463;
    // println!("Secret: {}, test: {}", secret, test);
    let shares = RSS3_arithmetic::share(secret);
    //println!("shares: {:?}", shares);

    let rec = RSS3_arithmetic::reconstruct(&shares);
    
    println!("Original: {}, Reconstructed: {}", secret, rec);
    assert_eq!(secret, rec);
}

#[test]
fn test_rss3_add() {
    let a = 10;
    let b = 20;
    let sa = RSS3_arithmetic::share(a);
    let sb = RSS3_arithmetic::share(b);
    let sum_shares = RSS3_arithmetic::add(&sa, &sb);
    let sum = RSS3_arithmetic::reconstruct(&sum_shares);

    assert_eq!(sum, a + b);
}

#[test]
fn test_rss3_sub() {
    let a = 23;
    let b = 123;
    let sa = RSS3_arithmetic::share(a);
    let sb = RSS3_arithmetic::share(b);
    let a_reconstruct = RSS3_arithmetic::reconstruct(&sa);
    let b_reconstruct = RSS3_arithmetic::reconstruct(&sb);
    println!("a: {}, b: {}", a_reconstruct, b_reconstruct);
    let sub_shares = RSS3_arithmetic::sub(&sa, &sb);
    let sub = RSS3_arithmetic::reconstruct(&sub_shares);
    let expected = (MODULUS_Zmod2n+ a - b) % MODULUS_Zmod2n;
    println!("Subtraction: {} - {} = {}", a, b, sub);
    println!("Expected: {}", expected);
    assert_eq!(expected,sub);
}

#[test]
fn test_rss3_mul() {
    let a = 535435;
    let b = 491663;
    let sa = RSS3_arithmetic::share(a);
    let sb = RSS3_arithmetic::share(b);
    let mul_shares = RSS3_arithmetic::mul(&sa, &sb);
    let mul = RSS3_arithmetic::reconstruct(&mul_shares);
    let expected = a * b;
    println!("Multiplication: {} * {} = {}", a, b, mul);
    println!("Expected: {}", expected);
    assert_eq!(mul, expected);
}

#[test]
fn test_rss3_boolean_share_and_reconstruct() {
    println!("Boolean Sharesxxxxxxxxxxxxxxxxxxxxxxxxxx");
    let secret = 0b11011011; //
    println!("Secret: {:04b}", secret);
    let shares = RSS3_boolean::share(secret);
    //println!("shares: {:?}", shares);
    let rec = RSS3_boolean::reconstruct(&shares);
    println!("Original: {:04b}, Reconstructed: {:04b}", secret, rec);
    assert_eq!(secret, rec);

    let rec2 = RSS3_boolean::reconstruct_2_test(&shares);
    println!("Reconstructed2: {:04b}", rec2);
    assert_eq!(secret, rec2);
}

#[test]
fn test_rss3_boolean_xor() {
    let a = 0b1101;
    let b = 0b1011;
    let expected_xor = a ^ b;
    println!("a: {:04b}, b: {:04b}", a, b);
    let sa = RSS3_boolean::share(a);
    let rec_sa = RSS3_boolean::reconstruct(&sa);
    println!("Reconstructed a: {:04b}", rec_sa);
    assert_eq!(rec_sa, a);

    let sb = RSS3_boolean::share(b);
    let rec_sb = RSS3_boolean::reconstruct(&sb);
    println!("Reconstructed b: {:04b}", rec_sb);
    assert_eq!(rec_sb, b);

    let xor_shares = RSS3_boolean::xor(&sa, &sb);
    let test = xor_shares[0].shares.0 ^ xor_shares[1].shares.0 ^ xor_shares[2].shares.0;
    println!("Test xor from x values: {:04b}", test);
    let xor = RSS3_boolean::reconstruct(&xor_shares);
    println!("XOR: {:08b}", xor);
    let xor2 = RSS3_boolean::reconstruct_2_test(&xor_shares);
    println!("XOR2: {:08b}", xor2);
    println!("Expected XOR: {:04b}", expected_xor);
    assert_eq!(xor, expected_xor);
}


#[test]
fn test_rss3_boolean_and() {
    let a = 0b11011010101001010101010101;
    let b = 0b10111111111111000000011111;
    let expected_and = a & b;
    println!("a: {:04b}, b: {:04b}", a, b);
    let sa = RSS3_boolean::share(a);
    let sb = RSS3_boolean::share(b);
    let and_shares = RSS3_boolean::and(&sa, &sb);
    let and = RSS3_boolean::reconstruct(&and_shares);
    println!("AND: {:08b}", and);
    println!("Expected AND: {:04b}", expected_and);
    assert_eq!(and, expected_and);
}

#[test]
fn test_rss3_boolean_bitwise() {
    let a = 0b1111111111111111111111111111111111111111111111111111111111111111;
    let b = 0b1111111111111111111111111111111111111111111111111111111111111111;
    let sa = RSS3S_boolean_bitwise::share(a);
    let sb = RSS3S_boolean_bitwise::share(b);
    let rec = RSS3S_boolean_bitwise::reconstruct(&sa);
    println!("Original: {}, Reconstructed: {}", a, rec);
    assert_eq!(a, rec);
    let xorab = RSS3S_boolean_bitwise::xor(&sa, &sb);
    let xor = RSS3S_boolean_bitwise::reconstruct(&xorab);
    let expected_xor = a ^ b;
    println!("XOR: {}, Expected XOR: {}", xor, expected_xor);
    assert_eq!(xor, expected_xor);

    let andab = RSS3S_boolean_bitwise::and(&sa, &sb);
    let and = RSS3S_boolean_bitwise::reconstruct(&andab);
    let expected_and = a & b;
    println!("AND: {}, Expected AND: {}", and, expected_and);
    assert_eq!(and, expected_and);

    let equal_ab = RSS3S_boolean_bitwise::equal(&sa, &sb);
    let equal = RSS3_single_boolean::reconstruct(&equal_ab);
    let expected_equal = a == b;
    println!("EQUAL: {}, Expected EQUAL: {}", equal, expected_equal);
    assert_eq!(equal, expected_equal);
}

