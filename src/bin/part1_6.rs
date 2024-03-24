use std::fs::read_to_string;
use base64::prelude::*;
use crypto_pals::*;

fn main() {
    let filename = "inputs/input_1_6.txt";
    let text = read_to_string(filename)
        .expect("unable to read from input file");
    let trimmed = text.trim();
    // FIXME: something wrong with base64 decoding (maybe trim?)
    let decoded = BASE64_STANDARD_NO_PAD.decode(trimmed).expect("unable to decode input");

    println!("{}",decoded.to_string());

}

fn calc_hamming_distance(in1: Vec<u8>, in2: Vec<u8>) -> u32 {
    if in1.len() == in2.len() {
        let y = repeating_key_xor(in1, in2).expect("error with xor")
            .iter()
            .map(|x| x.count_ones())
            .sum::<u32>();
        println!("y: {}", y);
        y
    }
    else {
        0xFFFFFFFF
    }
}

#[test]
fn set1_chal6() {
    let line1 = String::from("this is a test");
    let line2 = String::from("wokka wokka!!!");
    assert_eq!(calc_hamming_distance(line1.as_bytes().to_vec(), line2.as_bytes().to_vec()), 37);
}