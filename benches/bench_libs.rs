use crypto_pals::*;

pub fn main() {
    divan::main();
}

#[divan::bench]
pub fn bench_single_xor() {
    let mess: Vec<u8> = Vec::from("This is a Test Message for Benchmarking");
    let _ = single_byte_xor(mess, 150);
}

#[divan::bench]
pub fn bench_score_single() {
    let mess: Vec<u8> = Vec::from("This is a Test Message for Benchmarking");
    let _ = score_single_byte(mess);
}

#[divan::bench]
pub fn bench_repeating_key_xor() {
    let mess: Vec<u8> = Vec::from("This is a Test Message for Benchmarking");
    let key: Vec<u8> = Vec::from("T#ST");
    let _ = repeating_key_xor(mess, key);
}

