use crypto_pals::*;
use divan::bench;

pub fn main() {
    divan::main();
}

#[bench]
pub fn bench_single_xor() {
    let mess: Vec<u8> = Vec::from("This is a Test Message for Benchmarking");
    let _ = single_byte_xor(mess, 150);
}

#[bench]
pub fn bench_score_single() {
    let mess: Vec<u8> = Vec::from("This is a Test Message for Benchmarking");
    let _ = score_single_byte(mess);
}

//#[divan::bench(args )]
