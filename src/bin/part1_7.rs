use base64::{engine::general_purpose, Engine};
use crypto_pals::*;
use std::fs::read_to_string;


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
    text.retain(|c| { !c.is_whitespace() });
    let decoded = general_purpose::STANDARD.decode(text).expect("unable to decode input");
    let key = b"YELLOW SUBMARINE";
    let decrypted = decrypt_aes_128_ecb(decoded, key.to_vec()).unwrap();
    println!("{}", decrypted.to_string());
}

#[test]
fn set1_chal7() {
    let filename = "inputs/input_1_7.txt";
    let mut text = read_to_string(filename).expect("unable to read from input file");
    text.retain(|c| { !c.is_whitespace() });
    let decoded = general_purpose::STANDARD.decode(text).expect("unable to decode input");
    let key = b"YELLOW SUBMARINE";
    let expected_second_last_line = "Play that funky music, white boy Come on, Come on, Come on ";

    let decrypted = decrypt_aes_128_ecb(decoded, key.to_vec()).expect("Error decrypting input");
    let binding = decrypted.to_string();
    let lines: Vec<&str> = binding.lines().collect();
    let second_to_last = lines[lines.len()-2];
    println!("second to last: {}", second_to_last);

    assert_eq!(second_to_last, expected_second_last_line);
}