extern crate openssl;

use std::fs::read_to_string;
use base64::{engine::general_purpose, Engine};
use crypto_pals::*;
use openssl::symm::{Cipher,Crypter, Mode};

/*
AES in ECB mode
The Base64-encoded content in this file has been encrypted via AES-128 in ECB mode under the key
"YELLOW SUBMARINE".
(case-sensitive, without the quotes; exactly 16 characters; I like "YELLOW SUBMARINE" because it's exactly 16 bytes long, and now you do too).
Decrypt it. You know the key, after all.
Easiest way: use OpenSSL::Cipher and give it AES-128-ECB as the cipher.
Do this with code.
You can obviously decrypt this using the OpenSSL command-line tool, but we're having you get ECB working in code for a reason. You'll need it a lot later on, and not just for attacking ECB.
*/

fn main() {
    let filename = "inputs/input_1_7.txt";
    let mut text = read_to_string(filename).expect("unable to read from input file");
    text.retain(|c| {
       !c.is_whitespace()
    });
    let decoded = general_purpose::STANDARD.decode(text).expect("unable to decode input");
    println!("Decoded len: {}",decoded.len());
    let key = b"YELLOW SUBMARINE";
    let decrypted_len = decoded.len() + 16 - (decoded.len() % 16);
    println!("decrypted len: {}",decrypted_len);

    let decrypter = Crypter::new(Cipher::aes_128_ecb(), Mode::Decrypt, key, None);
    let mut decrypted = vec![0u8; 2896];
    let cipher_len = decrypter.unwrap().update(&decoded, decrypted.as_mut_slice());

    // println!("{}", decrypted.to_string());
}

// TODO: finish this func
// fn decrypt_aes_128_ecb() 

#[test]
fn set1_chal1() {
    todo!()
}